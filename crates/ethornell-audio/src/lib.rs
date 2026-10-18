use ethornell_core::{EthornellError, Result};
use kira::dsp::Frame;
use kira::manager::backend::mock::{MockBackend, MockBackendSettings};
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::{
    FromFileError, PlaybackPosition, PlaybackState,
    static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
    streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings},
};
use kira::tween::Tween;
use serde::Serialize;
use std::collections::BTreeMap;
use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct AudioInfo {
    pub kind: AudioKind,
    pub header_len: Option<u32>,
    pub file_size: Option<u32>,
    pub sample_len: Option<u32>,
    pub frequency: Option<u32>,
    pub channels: Option<u32>,
    pub payload_offset: Option<usize>,
    pub payload_kind: Option<AudioKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AudioKind {
    Ogg,
    RiffWave,
    BurikoWaveBoxOgg,
    BurikoWaveBoxUnknown,
    Unknown,
}

pub fn probe_audio(data: &[u8]) -> AudioInfo {
    if data.starts_with(b"OggS") {
        return simple_info(AudioKind::Ogg);
    }
    if data.starts_with(b"RIFF") {
        return simple_info(AudioKind::RiffWave);
    }
    if data.len() >= 0x40 && &data[4..8] == b"bw  " {
        let header_len = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let payload_offset = header_len as usize;
        let payload_kind = data
            .get(payload_offset..payload_offset + 4)
            .map(|magic| {
                if magic == b"OggS" {
                    AudioKind::Ogg
                } else if magic == b"RIFF" {
                    AudioKind::RiffWave
                } else {
                    AudioKind::Unknown
                }
            })
            .unwrap_or(AudioKind::Unknown);
        return AudioInfo {
            kind: if payload_kind == AudioKind::Ogg {
                AudioKind::BurikoWaveBoxOgg
            } else {
                AudioKind::BurikoWaveBoxUnknown
            },
            header_len: Some(header_len),
            file_size: read_u32(data, 8),
            sample_len: read_u32(data, 12),
            frequency: read_u32(data, 16),
            channels: read_u32(data, 20),
            payload_offset: Some(payload_offset),
            payload_kind: Some(payload_kind),
        };
    }
    AudioInfo {
        kind: AudioKind::Unknown,
        header_len: None,
        file_size: None,
        sample_len: None,
        frequency: None,
        channels: None,
        payload_offset: None,
        payload_kind: None,
    }
}

pub fn unwrap_buriko_wave_ogg(data: &[u8]) -> Option<&[u8]> {
    let info = probe_audio(data);
    if info.kind != AudioKind::BurikoWaveBoxOgg {
        return None;
    }
    let offset = info.payload_offset?;
    data.get(offset..)
}

fn simple_info(kind: AudioKind) -> AudioInfo {
    AudioInfo {
        kind,
        header_len: None,
        file_size: None,
        sample_len: None,
        frequency: None,
        channels: None,
        payload_offset: Some(0),
        payload_kind: Some(kind),
    }
}

fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    let bytes: [u8; 4] = data.get(offset..offset + 4)?.try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}

/// Port regression probe (branch probe/regress, never built into the app):
/// kira is built without its cpal backend, so the audio system has no
/// device and can't be heard. Sounds still decode and play inside kira; the
/// headless loop renders the mix in VM time with `probe_render`.
pub const PROBE_SAMPLE_RATE: u32 = 44_100;

/// What the probe's silent mix did, for its numbers-only log.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProbeMixStats {
    /// Static sounds started (script sounds, intro/loop pairs).
    pub plays: u64,
    /// Streamed sounds started (movie soundtracks).
    pub streams: u64,
    /// Mix frames rendered, and those with a sample above -60 dB.
    pub frames: u64,
    pub audible_frames: u64,
    pub peak: f32,
    pub sum_squares: f64,
}

pub struct AudioSystem {
    manager: AudioManager<MockBackend>,
    probe: ProbeMixStats,
    probe_carry: u64,
    volume: f64,
    channels: BTreeMap<i32, StaticSoundHandle>,
    /// Channels playing a sound that is decoded while it plays (movie
    /// soundtracks). A channel is in at most one of the two maps.
    streams: BTreeMap<i32, StreamingSoundHandle<FromFileError>>,
    next_unmanaged_channel: i32,
}

impl AudioSystem {
    pub fn new() -> Result<Self> {
        let manager = AudioManager::<MockBackend>::new(AudioManagerSettings {
            backend_settings: MockBackendSettings {
                sample_rate: PROBE_SAMPLE_RATE,
            },
            ..AudioManagerSettings::default()
        })
        .map_err(|()| EthornellError::Other("audio init failed".into()))?;
        Ok(Self {
            manager,
            probe: ProbeMixStats::default(),
            probe_carry: 0,
            volume: 1.0,
            channels: BTreeMap::new(),
            streams: BTreeMap::new(),
            next_unmanaged_channel: i32::MIN,
        })
    }

    pub fn play_ogg_file(&mut self, path: impl AsRef<std::path::Path>) -> Result<()> {
        let data = StaticSoundData::from_file(path, StaticSoundSettings::default())
            .map_err(|err| EthornellError::Other(format!("load audio failed: {err}")))?;
        self.manager
            .play(data)
            .map_err(|err| EthornellError::Other(format!("play audio failed: {err}")))?;
        Ok(())
    }

    pub fn play_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let channel = self.next_unmanaged_channel;
        self.next_unmanaged_channel = self.next_unmanaged_channel.saturating_add(1);
        self.play_on_channel(channel, bytes, false, 1.0, 1.0, 1.0, 0.5, 0)
    }

    pub fn play_on_channel(
        &mut self,
        channel: i32,
        bytes: &[u8],
        looped: bool,
        volume: f64,
        decode_gain: f64,
        playback_rate: f64,
        panning: f64,
        fade_in_ms: u64,
    ) -> Result<()> {
        self.play_on_channel_inner(
            channel,
            bytes,
            looped,
            volume,
            decode_gain,
            playback_rate,
            panning,
            fade_in_ms,
            true,
        )
        .map(|_| ())
    }

    pub fn play_on_channel_if_stopped(
        &mut self,
        channel: i32,
        bytes: &[u8],
        looped: bool,
        volume: f64,
        decode_gain: f64,
        playback_rate: f64,
        panning: f64,
        fade_in_ms: u64,
    ) -> Result<bool> {
        self.play_on_channel_inner(
            channel,
            bytes,
            looped,
            volume,
            decode_gain,
            playback_rate,
            panning,
            fade_in_ms,
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn play_intro_loop_on_channel(
        &mut self,
        channel: i32,
        intro_bytes: &[u8],
        loop_bytes: &[u8],
        looped: bool,
        volume: f64,
        decode_gain: f64,
        playback_rate: f64,
        panning: f64,
        fade_in_ms: u64,
        restart: bool,
    ) -> Result<bool> {
        if !restart && self.channel_is_active(channel) {
            return Ok(false);
        }

        let intro = decode_static_sound(intro_bytes, StaticSoundSettings::default())?;
        let loop_sound = decode_static_sound(loop_bytes, StaticSoundSettings::default())?;
        if intro.sample_rate != loop_sound.sample_rate {
            return Err(EthornellError::Other(format!(
                "BGM pair sample-rate mismatch: intro={}Hz loop={}Hz",
                intro.sample_rate, loop_sound.sample_rate
            )));
        }

        let loop_start = intro.frames.len();
        let mut frames = Vec::with_capacity(loop_start.saturating_add(loop_sound.frames.len()));
        frames.extend_from_slice(&intro.frames);
        frames.extend_from_slice(&loop_sound.frames);

        let mut settings = playback_settings(volume, playback_rate, panning, fade_in_ms);
        if looped {
            settings = settings
                .loop_region(PlaybackPosition::Samples(loop_start.min(i64::MAX as usize) as i64)..);
        }
        let mut data = StaticSoundData {
            sample_rate: intro.sample_rate,
            frames: frames.into(),
            settings,
        };
        apply_decode_gain(&mut data, decode_gain);
        self.start_static_sound(channel, data)
    }

    fn play_on_channel_inner(
        &mut self,
        channel: i32,
        bytes: &[u8],
        looped: bool,
        volume: f64,
        decode_gain: f64,
        playback_rate: f64,
        panning: f64,
        fade_in_ms: u64,
        restart: bool,
    ) -> Result<bool> {
        if !restart && self.channel_is_active(channel) {
            return Ok(false);
        }
        let mut settings = playback_settings(volume, playback_rate, panning, fade_in_ms);
        if looped {
            settings = settings.loop_region(..);
        }
        let mut data = decode_static_sound(bytes, settings)?;
        apply_decode_gain(&mut data, decode_gain);
        self.start_static_sound(channel, data)
    }

    fn start_static_sound(&mut self, channel: i32, data: StaticSoundData) -> Result<bool> {
        let handle = self
            .manager
            .play(data)
            .map_err(|err| EthornellError::Other(format!("play audio bytes failed: {err}")))?;
        self.probe.plays += 1;
        if let Some(mut previous) = self.channels.insert(channel, handle) {
            let _ = previous.stop(Tween::default());
        }
        if let Some(mut previous) = self.streams.remove(&channel) {
            let _ = previous.stop(Tween::default());
        }
        Ok(true)
    }

    /// Plays `bytes` (any stream Symphonia can probe, such as a movie's MPEG
    /// audio elementary stream) on `channel`, replacing what the channel
    /// played. Decoding happens on kira's streaming thread, so a long
    /// soundtrack neither stalls the caller nor is held decoded in memory.
    pub fn play_stream_on_channel(
        &mut self,
        channel: i32,
        bytes: Arc<[u8]>,
        looped: bool,
        volume: f64,
        start_ms: u64,
    ) -> Result<()> {
        let mut settings = StreamingSoundSettings::default()
            .volume(volume.clamp(0.0, 1.0))
            .playback_region(Duration::from_millis(start_ms).as_secs_f64()..);
        if looped {
            settings = settings.loop_region(..);
        }
        let data = StreamingSoundData::from_cursor(Cursor::new(bytes), settings)
            .map_err(|err| EthornellError::Other(format!("load audio stream failed: {err}")))?;
        let handle = self
            .manager
            .play(data)
            .map_err(|err| EthornellError::Other(format!("play audio stream failed: {err}")))?;
        self.probe.streams += 1;
        if let Some(mut previous) = self.channels.remove(&channel) {
            let _ = previous.stop(Tween::default());
        }
        if let Some(mut previous) = self.streams.insert(channel, handle) {
            let _ = previous.stop(Tween::default());
        }
        Ok(())
    }

    pub fn stop_all(&mut self) {
        for handle in self.channels.values_mut() {
            let _ = handle.stop(Tween::default());
        }
        self.channels.clear();
        for handle in self.streams.values_mut() {
            let _ = handle.stop(Tween::default());
        }
        self.streams.clear();
    }

    pub fn stop_channel(&mut self, channel: i32, fade_out_ms: u64) {
        if let Some(handle) = self.streams.get_mut(&channel) {
            let _ = handle.stop(Tween {
                duration: Duration::from_millis(fade_out_ms),
                ..Tween::default()
            });
            return;
        }
        let Some(handle) = self.channels.get_mut(&channel) else {
            return;
        };
        let _ = handle.stop(Tween {
            duration: Duration::from_millis(fade_out_ms),
            ..Tween::default()
        });
    }

    pub fn pause_channel(&mut self, channel: i32, paused: bool, fade_ms: u64) {
        let tween = Tween {
            duration: Duration::from_millis(fade_ms),
            ..Tween::default()
        };
        if let Some(handle) = self.streams.get_mut(&channel) {
            let _ = if paused {
                handle.pause(tween)
            } else {
                handle.resume(tween)
            };
            return;
        }
        let Some(handle) = self.channels.get_mut(&channel) else {
            return;
        };
        if paused {
            let _ = handle.pause(tween);
        } else {
            let _ = handle.resume(tween);
        }
    }

    pub fn set_channel_volume(&mut self, channel: i32, volume: f64, fade_ms: u64) {
        if let Some(handle) = self.streams.get_mut(&channel) {
            let _ = handle.set_volume(
                volume.clamp(0.0, 1.0),
                Tween {
                    duration: Duration::from_millis(fade_ms),
                    ..Tween::default()
                },
            );
            return;
        }
        let Some(handle) = self.channels.get_mut(&channel) else {
            return;
        };
        let _ = handle.set_volume(
            volume.clamp(0.0, 1.0),
            Tween {
                duration: Duration::from_millis(fade_ms),
                ..Tween::default()
            },
        );
    }

    pub fn set_channel_panning(&mut self, channel: i32, panning: f64, fade_ms: u64) {
        if let Some(handle) = self.streams.get_mut(&channel) {
            let _ = handle.set_panning(
                panning.clamp(0.0, 1.0),
                Tween {
                    duration: Duration::from_millis(fade_ms),
                    ..Tween::default()
                },
            );
            return;
        }
        let Some(handle) = self.channels.get_mut(&channel) else {
            return;
        };
        let _ = handle.set_panning(
            panning.clamp(0.0, 1.0),
            Tween {
                duration: Duration::from_millis(fade_ms),
                ..Tween::default()
            },
        );
    }

    pub fn channel_is_active(&self, channel: i32) -> bool {
        if let Some(handle) = self.streams.get(&channel) {
            return handle.state() != PlaybackState::Stopped;
        }
        self.channels
            .get(&channel)
            .is_some_and(|handle| handle.state() != PlaybackState::Stopped)
    }

    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    /// Renders `elapsed_ms` of the mix (the caller's clock) into nothing,
    /// keeping its peak and energy.
    pub fn probe_render(&mut self, elapsed_ms: u64) {
        let total = elapsed_ms * u64::from(PROBE_SAMPLE_RATE) + self.probe_carry;
        self.probe_carry = total % 1000;
        let backend = self.manager.backend_mut();
        backend.on_start_processing();
        for _ in 0..total / 1000 {
            let frame = backend.process();
            let level = frame.left.abs().max(frame.right.abs());
            self.probe.frames += 1;
            if level > 0.001 {
                self.probe.audible_frames += 1;
            }
            self.probe.peak = self.probe.peak.max(level);
            self.probe.sum_squares +=
                f64::from(frame.left) * f64::from(frame.left) + f64::from(frame.right) * f64::from(frame.right);
        }
    }

    pub fn probe_stats(&self) -> ProbeMixStats {
        self.probe
    }
}

/// Decodes a whole sound, cut to the length its file declares
/// (`declared_frame_count`).
pub fn decode_static_sound(bytes: &[u8], settings: StaticSoundSettings) -> Result<StaticSoundData> {
    let payload = unwrap_buriko_wave_ogg(bytes).unwrap_or(bytes);
    let mut data = StaticSoundData::from_cursor(Cursor::new(payload.to_vec()), settings)
        .map_err(|err| EthornellError::Other(format!("load audio bytes failed: {err}")))?;
    if let Some(frames) = declared_frame_count(bytes)
        && data.frames.len() > frames
    {
        data.frames = data.frames[..frames].into();
    }
    Ok(data)
}

/// How many frames a sound plays. For a BURIKO wave box, the header's sample
/// count (+0x0C): the target reads the 64-byte header (sub_4A8330), sizes
/// the decoded buffer as samples * channels * 2 bytes and asks for that many
/// samples (sub_4A91F0), and its Ogg reader clamps every request to the
/// samples left before that count (sub_4A8F40, through vorbisfile's ov_read,
/// sub_4BCAC0). For a bare Ogg stream, its end position. Symphonia, opened by
/// kira without gapless mode, decodes the last Vorbis block whole, so a sound
/// ran past its end position (which the box's count equals) by up to half a
/// block: 1023 frames with 2048-frame blocks.
fn declared_frame_count(bytes: &[u8]) -> Option<usize> {
    let info = probe_audio(bytes);
    let frames = match info.kind {
        AudioKind::BurikoWaveBoxOgg => u64::from(info.sample_len?),
        AudioKind::Ogg => ogg_end_position(bytes)?,
        _ => return None,
    };
    usize::try_from(frames).ok()
}

/// The granule position of an Ogg stream's last page, if the data ends with
/// a whole page of the stream the first page starts. For Vorbis that is the
/// number of frames up to the end of the stream.
fn ogg_end_position(data: &[u8]) -> Option<u64> {
    const MAX_PAGE_LEN: usize = 27 + 255 + 255 * 255;
    let serial = data.get(14..18)?;
    let first = data.len().saturating_sub(MAX_PAGE_LEN);
    let page = (first..data.len()).rev().find_map(|start| {
        let page = &data[start..];
        let segments = usize::from(*page.get(26)?);
        let body: usize = page
            .get(27..27 + segments)?
            .iter()
            .map(|&len| usize::from(len))
            .sum();
        (page.starts_with(b"OggS") && page[4] == 0 && 27 + segments + body == page.len())
            .then_some(page)
    })?;
    if &page[14..18] != serial {
        return None;
    }
    let granule = u64::from_le_bytes(page[6..14].try_into().ok()?);
    (granule != u64::MAX).then_some(granule)
}

fn playback_settings(
    volume: f64,
    playback_rate: f64,
    panning: f64,
    fade_in_ms: u64,
) -> StaticSoundSettings {
    let mut settings = StaticSoundSettings::default()
        .volume(volume.clamp(0.0, 1.0))
        .playback_rate(playback_rate.clamp(0.01, 16.0))
        .panning(panning.clamp(0.0, 1.0));
    if fade_in_ms > 0 {
        settings = settings.fade_in_tween(Tween {
            duration: Duration::from_millis(fade_in_ms),
            ..Tween::default()
        });
    }
    settings
}

fn apply_decode_gain(data: &mut StaticSoundData, decode_gain: f64) {
    let decode_gain = decode_gain.max(0.0) as f32;
    if (decode_gain - 1.0).abs() <= f32::EPSILON {
        return;
    }
    data.frames = data
        .frames
        .iter()
        .map(|frame| {
            Frame::new(
                (frame.left * decode_gain).clamp(-1.0, 1.0),
                (frame.right * decode_gain).clamp(-1.0, 1.0),
            )
        })
        .collect::<Vec<_>>()
        .into();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 10007 frames of a 440 Hz sine at half scale, 44.1 kHz stereo, encoded
    /// with libvorbis's vorbisenc (VBR quality 0, one audio page). The last
    /// page's granule position is 10007; the last block ends past it.
    const TONE_OGG: &[u8] = include_bytes!("../testdata/sine-440hz-10007-frames.ogg");
    const TONE_FRAMES: usize = 10007;

    /// The 64-byte BURIKO wave box header around an Ogg payload.
    fn buriko_wave_box(samples: u32, payload: &[u8]) -> Vec<u8> {
        let mut data = vec![0; 0x40];
        data[0..4].copy_from_slice(&0x40u32.to_le_bytes());
        data[4..8].copy_from_slice(b"bw  ");
        data[8..12].copy_from_slice(&(payload.len() as u32).to_le_bytes());
        data[12..16].copy_from_slice(&samples.to_le_bytes());
        data[16..20].copy_from_slice(&44_100u32.to_le_bytes());
        data[20..24].copy_from_slice(&2u32.to_le_bytes());
        data.extend_from_slice(payload);
        data
    }

    fn untrimmed_tone() -> StaticSoundData {
        StaticSoundData::from_cursor(Cursor::new(TONE_OGG), StaticSoundSettings::default()).unwrap()
    }

    #[test]
    fn a_wave_box_sound_decodes_to_its_header_sample_count() {
        let untrimmed = untrimmed_tone();
        assert!(untrimmed.frames.len() > TONE_FRAMES);

        let data = decode_static_sound(
            &buriko_wave_box(TONE_FRAMES as u32, TONE_OGG),
            StaticSoundSettings::default(),
        )
        .unwrap();
        assert_eq!(data.frames.len(), TONE_FRAMES);
        assert_eq!(data.frames[..], untrimmed.frames[..TONE_FRAMES]);
    }

    #[test]
    fn the_header_sample_count_bounds_a_wave_box_sound() {
        let shorter = decode_static_sound(
            &buriko_wave_box(4_000, TONE_OGG),
            StaticSoundSettings::default(),
        )
        .unwrap();
        assert_eq!(shorter.frames.len(), 4_000);

        // A count past the decoded frames (a stream with missing pages)
        // adds nothing.
        let longer = decode_static_sound(
            &buriko_wave_box(1_000_000, TONE_OGG),
            StaticSoundSettings::default(),
        )
        .unwrap();
        assert_eq!(longer.frames.len(), untrimmed_tone().frames.len());
    }

    #[test]
    fn a_bare_ogg_sound_decodes_to_its_end_position() {
        assert_eq!(ogg_end_position(TONE_OGG), Some(TONE_FRAMES as u64));
        let data = decode_static_sound(TONE_OGG, StaticSoundSettings::default()).unwrap();
        assert_eq!(data.frames.len(), TONE_FRAMES);
    }

    #[test]
    fn an_ogg_stream_without_a_whole_last_page_has_no_end_position() {
        assert_eq!(ogg_end_position(&TONE_OGG[..TONE_OGG.len() - 1]), None);
        assert_eq!(ogg_end_position(b"OggS"), None);
    }
}
