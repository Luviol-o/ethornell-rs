//! PORT-ONLY (branch probe/regress, the Wonderful Everyday port's regression
//! suite; never in the app): the decode pass. Every sound, image and movie in
//! a BGI game's archives, decoded with the engine's own decoders the way the
//! app decodes them (port note 22). Prints numbers only, in stable lines the
//! suite compares with its baseline: entries are named by archive file name
//! and index in archive order, never by entry name, and an error by its
//! variant name, never its message (both can carry story).
//!
//!   regress_decode GAME_DIR
//!
//! Sounds go through `decode_static_sound` (both static paths of the app use
//! it: each sound cut to its bw header's sample count). Images through the
//! app's bitmap checks and `decode_image`. MPEG movies frame by frame through
//! `na_mpeg2_decoder` and their soundtrack through kira, as graph_effect.rs
//! does. BGI frame movies ("BF_Movie") through the app's Graph90:F6 path,
//! every frame in order over one bitmap.
//! A 64-bit digest of the decoded output (samples, pixels, frames) per archive
//! shows a decoder whose output changed. Single-threaded; no audio device is
//! opened and nothing is written.

use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Context;
use ethornell_archive::{ResourceManager, decode_payload};
use ethornell_audio::{AudioKind, decode_static_sound, probe_audio, unwrap_buriko_wave_ogg};
use ethornell_image::{
    ImageFormat, bmp_native_bitmap_format, decode_image, detect_image_format, parse_cbg_metadata,
};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::sound::streaming::{StreamingSoundData, StreamingSoundSettings};
use na_mpeg2_decoder::{Decoder, Demuxer, StreamType, frame_to_rgba_bt601_limited};

/// FNV-1a over 64-bit little-endian words (bytes for the tail): stable
/// across builds and fast enough for every decoded byte of the game.
#[derive(Clone, Copy)]
struct Digest(u64);

impl Default for Digest {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl Digest {
    const PRIME: u64 = 0x100_0000_01b3;

    fn word(&mut self, word: u64) {
        self.0 = (self.0 ^ word).wrapping_mul(Self::PRIME);
    }

    fn bytes(&mut self, data: &[u8]) {
        let mut chunks = data.chunks_exact(8);
        for chunk in &mut chunks {
            self.word(u64::from_le_bytes(chunk.try_into().expect("8 bytes")));
        }
        for &byte in chunks.remainder() {
            self.word(u64::from(byte));
        }
    }
}

/// The variant chain of an error's Debug form ("SymphoniaError/DecodeError"),
/// cut before the first message, path or field.
fn kind_of(err: &dyn std::fmt::Debug) -> String {
    let text = format!("{err:?}");
    let mut parts = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            current.push(ch);
        } else if ch == '(' && !current.is_empty() {
            parts.push(std::mem::take(&mut current));
        } else {
            break;
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    if parts.is_empty() {
        "unknown".into()
    } else {
        parts.join("/")
    }
}

/// Runs `f`, turning a panic into an error kind (the panic hook is silenced).
fn guarded<T>(f: impl FnOnce() -> Result<T, (String, String)>) -> Result<T, (String, String)> {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(result) => result,
        Err(_) => Err(("panic".into(), "panic".into())),
    }
}

/// Per archive and kind: entries, decoded, decoded units (sample frames or
/// pixels), and the digest of the decoded output.
#[derive(Default)]
struct Tally {
    entries: usize,
    decoded: usize,
    units: u64,
    digest: Digest,
}

fn last_ogg_granule(payload: &[u8]) -> Option<u64> {
    let mut at = payload.len().checked_sub(27)?;
    loop {
        if &payload[at..at + 4] == b"OggS" {
            let granule = i64::from_le_bytes(payload[at + 6..at + 14].try_into().ok()?);
            return (granule >= 0).then_some(granule as u64);
        }
        at = at.checked_sub(1)?;
    }
}

struct Sound {
    frames: u64,
    rate: u32,
    expected: Option<u64>,
}

fn check_sound(bytes: &[u8], digest: &mut Digest) -> Result<Sound, (String, String)> {
    let info = probe_audio(bytes);
    // The bw header's sample count, or a bare Ogg stream's end position.
    let expected = info.sample_len.map(u64::from).or_else(|| {
        matches!(info.kind, AudioKind::Ogg)
            .then(|| last_ogg_granule(unwrap_buriko_wave_ogg(bytes).unwrap_or(bytes)))
            .flatten()
    });
    let data = guarded(|| {
        decode_static_sound(bytes, StaticSoundSettings::default())
            .map_err(|err| ("decode".to_string(), kind_of(&err)))
    })?;
    if data.sample_rate == 0 {
        return Err(("decode".into(), "ZeroRate".into()));
    }
    for frame in data.frames.iter() {
        if frame.left.is_nan() || frame.right.is_nan() {
            return Err(("decode".into(), "NaN".into()));
        }
        digest.word(u64::from(frame.left.to_bits()) | (u64::from(frame.right.to_bits()) << 32));
    }
    Ok(Sound {
        frames: data.frames.len() as u64,
        rate: data.sample_rate,
        expected,
    })
}

fn check_image(
    bytes: &[u8],
    format: ImageFormat,
    digest: &mut Digest,
) -> Result<u64, (String, String)> {
    // The app's bitmap load: CBG header checks, BMP format, then decode_image.
    let mut expected = None;
    match format {
        ImageFormat::CompressedBg => {
            let meta =
                parse_cbg_metadata(bytes).map_err(|err| ("cbg-header".into(), kind_of(&err)))?;
            if meta.embedded_point_flag > 1 {
                return Err((
                    "cbg-flag".into(),
                    format!("flag{}", meta.embedded_point_flag),
                ));
            }
            if meta.native_bitmap_format().is_none() {
                return Err(("cbg-format".into(), format!("bpp{}", meta.bpp)));
            }
            expected = Some((meta.width, meta.height));
        }
        ImageFormat::RawBgiImage => {
            expected = Some((
                u32::from(u16::from_le_bytes([bytes[0], bytes[1]])),
                u32::from(u16::from_le_bytes([bytes[2], bytes[3]])),
            ));
        }
        ImageFormat::Bmp => {
            bmp_native_bitmap_format(bytes).map_err(|err| ("bmp-header".into(), kind_of(&err)))?;
        }
        _ => {}
    }
    let image = guarded(|| decode_image(bytes).map_err(|err| ("decode".into(), kind_of(&err))))?;
    if image.rgba.len() != image.width as usize * image.height as usize * 4 {
        return Err(("size".into(), "rgba-length".into()));
    }
    if let Some((w, h)) = expected
        && (w, h) != (image.width, image.height)
    {
        return Err(("size".into(), "dimensions".into()));
    }
    digest.word(u64::from(image.width) << 32 | u64::from(image.height));
    digest.bytes(&image.rgba);
    Ok(u64::from(image.width) * u64::from(image.height))
}

// Copies of graph_effect.rs's private helpers (the app's movie duration).
fn probe_mpeg_duration_ms(bytes: &[u8]) -> i32 {
    let mut demuxer = Demuxer::new_auto();
    let packets = demuxer.push(bytes, None);
    let mut first_pts = None;
    let mut last_pts = None;
    let mut pictures = 0_u64;
    let mut frame_rate = None;
    for packet in packets
        .iter()
        .filter(|packet| packet.stream_type == StreamType::MpegVideo)
    {
        if let Some(pts) = packet.pts_90k {
            first_pts.get_or_insert(pts);
            last_pts = Some(pts);
        }
        pictures += packet
            .data
            .windows(4)
            .filter(|w| *w == [0, 0, 1, 0])
            .count() as u64;
        frame_rate = frame_rate.or_else(|| sequence_frame_rate(&packet.data));
    }
    let frame_ms = frame_rate
        .map(|fps| (1000.0 / fps).round() as i64)
        .unwrap_or(40);
    let duration = match (first_pts, last_pts) {
        (Some(first), Some(last)) if last > first => {
            ((last - first) * 1000 / 90_000).saturating_add(frame_ms)
        }
        _ if pictures > 0 => {
            ((pictures as f64 * 1000.0) / frame_rate.unwrap_or(25.0)).round() as i64
        }
        _ => 0,
    };
    duration.clamp(0, i32::MAX as i64) as i32
}

fn sequence_frame_rate(bytes: &[u8]) -> Option<f64> {
    const RATES: [f64; 9] = [
        0.0,
        24000.0 / 1001.0,
        24.0,
        25.0,
        30000.0 / 1001.0,
        30.0,
        50.0,
        60000.0 / 1001.0,
        60.0,
    ];
    bytes.windows(8).find_map(|window| {
        (window[..4] == [0, 0, 1, 0xB3])
            .then(|| {
                RATES
                    .get((window[7] & 0x0F) as usize)
                    .copied()
                    .unwrap_or(0.0)
            })
            .filter(|rate| *rate > 0.0)
    })
}

struct Movie {
    frames: u64,
    app_ms: i32,
    stream_ms: u64,
    audio_frames: u64,
    video_digest: Digest,
    audio_digest: Digest,
}

fn check_movie(bytes: &[u8]) -> Result<Movie, (String, String)> {
    // GraphEffectRegistry::configure_media + MovieDecoder (graph_effect.rs).
    let app_ms = probe_mpeg_duration_ms(bytes);
    let mut demuxer = Demuxer::new_auto();
    let mut packets = Vec::new();
    let mut audio = Vec::new();
    for packet in demuxer.push(bytes, None) {
        match packet.stream_type {
            StreamType::MpegVideo => packets.push(packet),
            StreamType::MpegAudio => audio.extend_from_slice(&packet.data),
            StreamType::Unknown => {}
        }
    }
    let mut movie = Movie {
        frames: 0,
        app_ms,
        stream_ms: 0,
        audio_frames: 0,
        video_digest: Digest::default(),
        audio_digest: Digest::default(),
    };
    let mut decoder = Decoder::new();
    let mut rgba = Vec::new();
    let mut convert = |frame: &na_mpeg2_decoder::Frame, movie: &mut Movie| {
        let len = frame.width * frame.height * 4;
        if rgba.len() != len {
            rgba = vec![0; len];
        }
        frame_to_rgba_bt601_limited(frame, &mut rgba);
        movie.frames += 1;
        movie
            .video_digest
            .word((frame.width as u64) << 32 | frame.height as u64);
        movie.video_digest.bytes(&rgba);
    };
    guarded(|| {
        for packet in &packets {
            let frames = decoder
                .decode_shared(&packet.data, packet.pts_90k)
                .map_err(|err| ("video-decode".to_string(), kind_of(&err)))?;
            for frame in frames {
                convert(frame.as_ref(), &mut movie);
            }
        }
        let frames = decoder
            .flush_shared()
            .map_err(|err| ("video-flush".to_string(), kind_of(&err)))?;
        for frame in frames {
            convert(frame.as_ref(), &mut movie);
        }
        Ok(())
    })?;
    if movie.frames == 0 {
        return Err(("video".into(), "NoFrames".into()));
    }
    if !audio.is_empty() {
        let shared: Arc<[u8]> = audio.into();
        let stream = guarded(|| {
            StreamingSoundData::from_cursor(
                Cursor::new(shared.clone()),
                StreamingSoundSettings::default(),
            )
            .map_err(|err| ("audio-stream".to_string(), kind_of(&err)))
        })?;
        movie.stream_ms = stream.duration().as_millis() as u64;
        let data = guarded(|| {
            StaticSoundData::from_cursor(
                Cursor::new(shared.clone()),
                StaticSoundSettings::default(),
            )
            .map_err(|err| ("audio-decode".to_string(), kind_of(&err)))
        })?;
        movie.audio_frames = data.frames.len() as u64;
        for frame in data.frames.iter() {
            movie
                .audio_digest
                .word(u64::from(frame.left.to_bits()) | (u64::from(frame.right.to_bits()) << 32));
        }
    }
    Ok(movie)
}

fn is_mpeg(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0, 0, 1, 0xBA]) || bytes.starts_with(&[0, 0, 1, 0xB3])
}

fn main() -> anyhow::Result<()> {
    let game = std::env::args()
        .nth(1)
        .context("usage: regress_decode GAME_DIR")?;
    std::panic::set_hook(Box::new(|_| {}));
    let manager = ResourceManager::open_game(&game)?;
    let started = Instant::now();

    let mut indices: HashMap<PathBuf, usize> = HashMap::new();
    let mut classes: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut tallies: BTreeMap<(&'static str, String), Tally> = BTreeMap::new();
    let mut lines: Vec<String> = Vec::new();
    let mut sound_secs = 0.0f64;
    let mut sound_exact = 0usize;
    let entries = manager.list();
    let total = entries.len();
    eprintln!("entries {total}");
    for (n, entry) in entries.into_iter().enumerate() {
        if n % 2000 == 0 && n > 0 {
            eprintln!(
                "progress {n}/{total} elapsed {:.0}s",
                started.elapsed().as_secs_f64()
            );
        }
        let archive = entry
            .archive_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("?")
            .to_string();
        let index = {
            let slot = indices.entry(entry.archive_path.clone()).or_default();
            let index = *slot;
            *slot += 1;
            index
        };
        let raw = match manager.read_by_entry_raw(&entry) {
            Ok(raw) => raw,
            Err(err) => {
                lines.push(format!(
                    "FAIL entry {archive} #{index} read {}",
                    kind_of(&err)
                ));
                continue;
            }
        };
        let bytes = match guarded(|| {
            decode_payload(&entry.entry_name, &raw).map_err(|err| ("unpack".into(), kind_of(&err)))
        }) {
            Ok(bytes) => bytes,
            Err((stage, err)) => {
                lines.push(format!("FAIL entry {archive} #{index} {stage} {err}"));
                continue;
            }
        };
        drop(raw);
        let audio_kind = probe_audio(&bytes).kind;
        let image_format = detect_image_format(&bytes);
        let class = if is_mpeg(&bytes) {
            let tally = tallies.entry(("movie", archive.clone())).or_default();
            tally.entries += 1;
            let t = Instant::now();
            match check_movie(&bytes) {
                Ok(movie) => {
                    tally.decoded += 1;
                    tally.units += movie.frames;
                    tally.digest.word(movie.video_digest.0);
                    tally.digest.word(movie.audio_digest.0);
                    lines.push(format!(
                        "movie {archive} #{index} frames {} app-ms {} stream-ms {} audio-frames {} \
                         video {:016x} audio {:016x}",
                        movie.frames,
                        movie.app_ms,
                        movie.stream_ms,
                        movie.audio_frames,
                        movie.video_digest.0,
                        movie.audio_digest.0
                    ));
                }
                Err((stage, err)) => {
                    lines.push(format!("FAIL movie {archive} #{index} {stage} {err}"))
                }
            }
            eprintln!("movie {archive} #{index} {:.0}s", t.elapsed().as_secs_f64());
            "movie"
        } else if bytes.starts_with(b"BF_Movie") {
            let tally = tallies.entry(("frame-movie", archive.clone())).or_default();
            tally.entries += 1;
            match ethornell_app::probe_frame_movie_status(&bytes) {
                Ok((statuses, frames_digest)) => {
                    let decoded = statuses.iter().filter(|status| **status == 0).count();
                    let mut by_status: BTreeMap<i32, usize> = BTreeMap::new();
                    for status in &statuses {
                        *by_status.entry(*status).or_default() += 1;
                    }
                    if decoded == statuses.len() {
                        tally.decoded += 1;
                    }
                    tally.units += decoded as u64;
                    tally.digest.word(frames_digest);
                    lines.push(format!(
                        "frame-movie {archive} #{index} frames {} decoded {decoded} statuses {by_status:?} \
                         pixels {frames_digest:016x}",
                        statuses.len()
                    ));
                }
                Err(status) => lines.push(format!(
                    "FAIL frame-movie {archive} #{index} load status{status}"
                )),
            }
            "frame movie"
        } else if matches!(
            audio_kind,
            AudioKind::BurikoWaveBoxOgg
                | AudioKind::BurikoWaveBoxUnknown
                | AudioKind::Ogg
                | AudioKind::RiffWave
        ) {
            let tally = tallies.entry(("sound", archive.clone())).or_default();
            tally.entries += 1;
            match check_sound(&bytes, &mut tally.digest) {
                Ok(sound) => {
                    tally.decoded += 1;
                    tally.units += sound.frames;
                    sound_secs += sound.frames as f64 / f64::from(sound.rate);
                    match sound.expected {
                        Some(expected) if expected == sound.frames => sound_exact += 1,
                        expected => lines.push(format!(
                            "LENGTH sound {archive} #{index} frames {} expected {expected:?}",
                            sound.frames
                        )),
                    }
                }
                Err((stage, err)) => {
                    lines.push(format!("FAIL sound {archive} #{index} {stage} {err}"))
                }
            }
            "sound"
        } else if image_format != ImageFormat::Unknown {
            let tally = tallies.entry(("image", archive.clone())).or_default();
            tally.entries += 1;
            match check_image(&bytes, image_format, &mut tally.digest) {
                Ok(pixels) => {
                    tally.decoded += 1;
                    tally.units += pixels;
                }
                Err((stage, err)) => lines.push(format!(
                    "FAIL image-{image_format:?} {archive} #{index} {stage} {err}"
                )),
            }
            "image"
        } else if bytes.starts_with(b"BurikoCompiledScriptVer1.00") {
            "BCS script"
        } else if entry.entry_name.to_ascii_lowercase().ends_with("._bp") {
            "._bp program"
        } else {
            "other"
        };
        *classes.entry(class).or_default() += 1;
    }

    println!("entries {total}");
    for (class, count) in &classes {
        println!("class {class} {count}");
    }
    let mut kinds: BTreeMap<&str, (usize, usize, u64)> = BTreeMap::new();
    for ((kind, archive), tally) in &tallies {
        println!(
            "archive {kind} {archive} entries {} decoded {} units {} digest {:016x}",
            tally.entries, tally.decoded, tally.units, tally.digest.0
        );
        let sum = kinds.entry(kind).or_default();
        sum.0 += tally.entries;
        sum.1 += tally.decoded;
        sum.2 += tally.units;
    }
    for (kind, (entries, decoded, units)) in &kinds {
        println!("kind {kind} entries {entries} decoded {decoded} units {units}");
    }
    println!("sounds exactly their header's length {sound_exact}, decoded {sound_secs:.1} s");
    for line in &lines {
        println!("{line}");
    }
    eprintln!("done {:.0}s", started.elapsed().as_secs_f64());
    Ok(())
}
