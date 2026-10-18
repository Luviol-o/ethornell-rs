// PORT-ONLY: the Wonderful Everyday port's regression-suite probe (branch
// probe/regress of the fork). Never merged into `port`, never built into the
// app. Headless only. Every line it logs holds numbers, census ids and fixed
// words: no game text, no archive entry names (the owner hasn't played the
// game). See regress/README.md in the port repository.
//
// Autopilot (PROBE_AUTOPLAY=<frame>), from notes 17 and 21 of the port:
// - skip mode (default): hold Ctrl, release and press it again after
//   PROBE_REPRESS idle frames (30; the skip gate latches on a fresh press).
// - auto mode (PROBE_MODE=auto): no Ctrl; the input script switches the
//   game's auto mode on, the autopilot only answers choices.
// After PROBE_STALL idle frames (120) look for a choice: a live, running icon
// processor configured since the last line with clickable on-screen items,
// other than a row of three or more on one line (the message window's
// buttons). Click item k of PROBE_CHOICES (comma list, one per choice met;
// default 0; items sorted top to bottom, then left to right). A menu that comes
// back within PROBE_LOOP_LINES (50) lines gets the next item (PROBE_NOLOOP=1:
// never); PROBE_RETRY=<frames> answers a menu that is still up and unchanged
// that long after a click again with the plan's next item. Without a choice,
// Enter every PROBE_NUDGE idle frames (600 in skip mode, never in auto mode);
// give up after PROBE_GIVEUP idle frames (30000 skip, 6000 auto).
// PROBE_TITLE=l,t,w,h of the title menu's first item: back at the title ends
// the run. PROBE_STOP_LINES=<n>: the run ends after n lines.
// PROBE_MACROS="L=script;L=script": when the run has drawn L lines, release
// Ctrl and play the input script (ETHORNELL_HEADLESS_SCRIPT syntax) with the
// autopilot paused; used to save checkpoints through the game's own menus.
// PROBE_RECTS=f,f,...: log every live icon processor's item rectangles at
// those frames (to find buttons without looking at the screen).
use super::*;
use ethornell_vm::GraphApi;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

static PROBE_LINES: AtomicU64 = AtomicU64::new(0);
static PROBE_FRAME: AtomicU64 = AtomicU64::new(0);
static PROBE_CONFIG_SEQ: AtomicU64 = AtomicU64::new(0);
/// object -> (configure sequence number, lines at configure)
static PROBE_CONFIGURED: Mutex<BTreeMap<i32, (u64, u64)>> = Mutex::new(BTreeMap::new());

pub(crate) fn probe_frame() -> u64 {
    PROBE_FRAME.load(Ordering::Relaxed)
}

/// FNV-1a 64 of a line's text: the record keeps its length and this hash only.
pub(crate) fn line_hash(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3)
    })
}

/// A line drawn to the message window: skipped lines (Graph92:91) and typed
/// ones (Graph92:90 with a visible glyph).
pub(crate) fn probe_record_line(kind: &str, text: &str) {
    let n = PROBE_LINES.fetch_add(1, Ordering::Relaxed) + 1;
    tracing::info!(
        frame = probe_frame(),
        n,
        kind,
        chars = text.chars().count(),
        hash = format_args!("{:016x}", line_hash(text)),
        "PROBELINE"
    );
}

pub(crate) fn probe_note_configure(object: i32) {
    let seq = PROBE_CONFIG_SEQ.fetch_add(1, Ordering::Relaxed) + 1;
    let lines = PROBE_LINES.load(Ordering::Relaxed);
    if let Ok(mut map) = PROBE_CONFIGURED.lock() {
        map.insert(object, (seq, lines));
    }
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

/// The census id of a scenario file ("01101#04": the fifth entry, in name
/// order, of data01101.arc), as the port's notes name them. None for other
/// archives.
pub(crate) fn census_id(
    manager: &ethornell_archive::ResourceManager,
    entry: &ethornell_archive::ResourceEntry,
) -> Option<String> {
    let stem = entry
        .archive_path
        .file_stem()?
        .to_str()?
        .to_ascii_lowercase();
    let digits = stem.strip_prefix("data")?;
    if !digits.starts_with("01") {
        return None;
    }
    let archive = manager
        .archives()
        .archives
        .iter()
        .find(|archive| archive.path == entry.archive_path)?;
    let mut names = archive
        .entries
        .iter()
        .map(|entry| entry.name.as_str())
        .collect::<Vec<_>>();
    names.sort_unstable();
    let index = names.iter().position(|name| *name == entry.entry_name)?;
    Some(format!("{digits}#{index:02}"))
}

/// The entry a census id names.
fn census_entry(
    manager: &ethornell_archive::ResourceManager,
    id: &str,
) -> Option<ethornell_archive::ResourceEntry> {
    let (digits, index) = id.split_once('#')?;
    let index: usize = index.parse().ok()?;
    let archive_name = format!("data{digits}.arc");
    let archive = manager.archives().archives.iter().find(|archive| {
        archive
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.eq_ignore_ascii_case(&archive_name))
    })?;
    let mut names = archive
        .entries
        .iter()
        .map(|entry| entry.name.as_str())
        .collect::<Vec<_>>();
    names.sort_unstable();
    manager.find_in_archive(&archive_name, names.get(index)?)
}

/// LoadFileBytes of a scenario file: logs its census id, and with
/// PROBE_SCEN_REDIRECT=FROM>TO hands the scripts file TO when they load FROM
/// (to start a file no script runs).
pub(crate) fn probe_scenario_load(
    manager: &ethornell_archive::ResourceManager,
    archive: &str,
    file: &str,
    bytes: Option<Vec<u8>>,
) -> Option<Vec<u8>> {
    let Some(entry) = find_runtime_resource(manager, archive, file) else {
        return bytes;
    };
    let Some(id) = census_id(manager, &entry) else {
        return bytes;
    };
    if let Ok(redirect) = std::env::var("PROBE_SCEN_REDIRECT")
        && let Some((from, to)) = redirect.split_once('>')
        && from.trim() == id
    {
        let replaced = census_entry(manager, to.trim())
            .and_then(|entry| manager.read_by_entry_decoded(&entry).ok());
        tracing::info!(
            frame = probe_frame(),
            census = %id,
            to = %to.trim(),
            found = replaced.is_some(),
            "PROBEREDIRECT"
        );
        if let Some(replaced) = replaced {
            tracing::info!(frame = probe_frame(), census = %to.trim(), size = replaced.len(), "PROBESCEN");
            return Some(replaced);
        }
    }
    tracing::info!(
        frame = probe_frame(),
        census = %id,
        size = bytes.as_ref().map(Vec::len).unwrap_or_default(),
        "PROBESCEN"
    );
    bytes
}

/// For the suite's decode pass: each frame of a BGI frame movie ("BF_Movie")
/// through the app's Graph90:F6 path (BurikoMovieRegistry::decode_frame), in
/// order, over one bitmap of the movie's own size and format (a frame codes
/// only the blocks that change). Returns each frame's status (0 = decoded)
/// and an FNV-1a digest of the bitmap after every frame; Err(status) when
/// the resource doesn't load.
pub fn probe_frame_movie_status(bytes: &[u8]) -> std::result::Result<(Vec<i32>, u64), i32> {
    let mut registry = graph_movie::BurikoMovieRegistry::default();
    let (handle, metadata) = registry.load(String::new(), String::new(), bytes.to_vec())?;
    let (width, height) = (metadata[0].max(0) as u32, metadata[1].max(0) as u32);
    let mut pixels = DecodedImage {
        width,
        height,
        rgba: vec![0; width as usize * height as usize * 4],
    };
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    let statuses = (0..metadata[4].max(0))
        .map(|frame| {
            let status = registry.decode_frame(
                Some((width, height, metadata[2])),
                Some(&mut pixels),
                handle,
                frame,
            );
            for &byte in &pixels.rgba {
                digest = (digest ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
            }
            status
        })
        .collect();
    Ok((statuses, digest))
}

/// PROBE_POKE=address=value[,address=value]: 32-bit words written into the
/// main VM's memory before every frame (a switch the retail scripts never
/// set, such as the title's word at 1024).
pub(crate) fn probe_pokes() -> Vec<(usize, u32)> {
    std::env::var("PROBE_POKE")
        .unwrap_or_default()
        .split(',')
        .filter_map(|item| {
            let (address, value) = item.split_once('=')?;
            Some((address.trim().parse().ok()?, value.trim().parse().ok()?))
        })
        .collect()
}

pub(crate) fn apply_pokes(memory: &mut [u8], pokes: &[(usize, u32)]) {
    for &(address, value) in pokes {
        if let Some(word) = memory.get_mut(address..address + 4) {
            word.copy_from_slice(&value.to_le_bytes());
        }
    }
}

/// The silent mix's counters (ethornell-audio's probe renderer).
pub(crate) fn log_audio(frame: usize, audio: &AudioSystem) {
    let stats = audio.probe_stats();
    let rms = if stats.frames > 0 {
        (stats.sum_squares / (2 * stats.frames) as f64).sqrt()
    } else {
        0.0
    };
    tracing::info!(
        frame,
        plays = stats.plays,
        streams = stats.streams,
        mix_frames = stats.frames,
        audible_frames = stats.audible_frames,
        peak = format_args!("{:.4}", stats.peak),
        rms = format_args!("{rms:.6}"),
        "PROBEAUDIO"
    );
}

#[derive(Clone, Copy, Debug)]
struct ItemRect {
    group: i32,
    index: i32,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    eligible: bool,
}

impl RuntimeTraceApi {
    /// Screen hit rectangles of a processor's live items, as
    /// hit_test_graph_input_object computes them.
    fn probe_item_rects(&self, object: i32) -> Vec<ItemRect> {
        let Some(input) = self.graph_input_objects.get(&object) else {
            return Vec::new();
        };
        let visible = !self
            .graph_surfaces
            .get(&input.layer)
            .is_some_and(|surface| !self.surface_display_chain_visible(input.layer, surface));
        let owner = SurfaceControlOwner::InputObject(object);
        let mut out = Vec::new();
        for (&layer_id, layer) in &self.graph_layers {
            if layer.target_surface != Some(input.layer)
                || !self
                    .surface_controls
                    .contains_layer_for_owner(owner, layer_id)
                || !layer.enabled
            {
                continue;
            }
            let Ok(ordinal) = usize::try_from(layer.hit_id) else {
                continue;
            };
            let Some(region) = input.descriptor.regions.get(ordinal).copied() else {
                continue;
            };
            if region.enabled_depth == 0 {
                continue;
            }
            let resource = native_surface_control_configure_resource(&region);
            let (width, height) = self
                .resource_image_region(resource)
                .map(|(_, source)| (source.width, source.height))
                .unwrap_or((layer.width.max(0.0), layer.height.max(0.0)));
            let (left, top, _) = self.layer_world_transform(layer_id, layer);
            out.push(ItemRect {
                group: region.group,
                index: region.index,
                left,
                top,
                width,
                height,
                eligible: visible
                    && input.descriptor.pointer_processing_enabled
                    && input.pointer_hit_eligible(region)
                    && width > 0.0
                    && height > 0.0,
            });
        }
        out.sort_by(|a, b| {
            a.top
                .total_cmp(&b.top)
                .then(a.left.total_cmp(&b.left))
                .then(a.group.cmp(&b.group))
                .then(a.index.cmp(&b.index))
        });
        out
    }

    /// Every live processor as "#id run en seq at regions [group/index@x,y wxh]"
    /// ("-" after an item that can't be clicked).
    fn probe_objects(&self, configured: &BTreeMap<i32, (u64, u64)>) -> String {
        let mut objects = Vec::new();
        for (&object, input) in &self.graph_input_objects {
            if !input.is_live() {
                continue;
            }
            let (seq, lines_at) = configured.get(&object).copied().unwrap_or((0, 0));
            let items = self
                .probe_item_rects(object)
                .iter()
                .map(|r| {
                    format!(
                        "{}/{}@{},{} {}x{}{}",
                        r.group,
                        r.index,
                        r.left as i32,
                        r.top as i32,
                        r.width as i32,
                        r.height as i32,
                        if r.eligible { "" } else { "-" }
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            objects.push(format!(
                "#{object} run={} en={} seq={seq} at={lines_at} regions={} [{items}]",
                u8::from(input.is_running()),
                input.enabled,
                input.descriptor.regions.len(),
            ));
        }
        objects.join(" | ")
    }
}

struct Macro {
    lines: u64,
    source: String,
}

pub(crate) struct ProbeAutoplay {
    start_frame: usize,
    auto_mode: bool,
    stall: u64,
    nudge: u64,
    giveup: u64,
    stop_lines: u64,
    choices: Vec<usize>,
    choice_no: usize,
    lines_at_start: u64,
    last_lines: u64,
    last_progress: usize,
    answered: BTreeSet<(i32, u64)>,
    last_click: Option<(i32, u64, usize, u64)>,
    retry: u64,
    release: Option<(f32, f32)>,
    key_release: bool,
    repress: u64,
    ctrl_down: bool,
    stall_logged: bool,
    stalls: u64,
    /// Menu signature (items, first item's rect) -> (lines when last
    /// answered, times answered in a row).
    menus: BTreeMap<(usize, i32, i32, i32, i32), (u64, usize)>,
    macros: VecDeque<Macro>,
    running_macro: Option<HeadlessInputScript>,
    macro_no: usize,
    rect_frames: Vec<usize>,
}

impl ProbeAutoplay {
    pub(crate) fn from_env() -> Option<Self> {
        let start_frame = std::env::var("PROBE_AUTOPLAY").ok()?.parse().ok()?;
        let auto_mode = std::env::var("PROBE_MODE").is_ok_and(|mode| mode == "auto");
        let choices = std::env::var("PROBE_CHOICES")
            .unwrap_or_default()
            .split(',')
            .filter_map(|item| item.trim().parse().ok())
            .collect();
        let mut macros = std::env::var("PROBE_MACROS")
            .unwrap_or_default()
            .split(';')
            .filter_map(|item| {
                let (lines, source) = item.split_once('=')?;
                Some(Macro {
                    lines: lines.trim().parse().ok()?,
                    source: source.trim().to_string(),
                })
            })
            .collect::<Vec<_>>();
        macros.sort_by_key(|item| item.lines);
        let rect_frames = std::env::var("PROBE_RECTS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|item| item.trim().parse().ok())
            .collect();
        tracing::info!(start_frame, auto_mode, macros = macros.len(), "PROBECONFIG");
        Some(Self {
            start_frame,
            auto_mode,
            stall: env_u64("PROBE_STALL", 120),
            nudge: env_u64("PROBE_NUDGE", if auto_mode { 0 } else { 600 }),
            giveup: env_u64("PROBE_GIVEUP", if auto_mode { 6000 } else { 30000 }),
            stop_lines: env_u64("PROBE_STOP_LINES", 0),
            choices,
            choice_no: 0,
            lines_at_start: 0,
            last_lines: 0,
            last_progress: 0,
            answered: BTreeSet::new(),
            last_click: None,
            retry: env_u64("PROBE_RETRY", 0),
            release: None,
            key_release: false,
            repress: env_u64("PROBE_REPRESS", 30),
            ctrl_down: false,
            stall_logged: false,
            stalls: 0,
            menus: BTreeMap::new(),
            macros: macros.into(),
            running_macro: None,
            macro_no: 0,
            rect_frames,
        })
    }

    fn summary(&self, frame: usize, lines: u64, event: &str) {
        tracing::info!(
            frame,
            lines,
            run_lines = lines - self.lines_at_start,
            choices = self.choice_no,
            stalls = self.stalls,
            macros = self.macro_no,
            event,
            "PROBEEND"
        );
    }

    /// Runs before the frame's VM pass. Returns true to end the run.
    pub(crate) fn tick(
        &mut self,
        frame: usize,
        api: &mut RuntimeTraceApi,
        vm: &ethornell_vm::Vm,
        last: Option<&ethornell_vm::VmRunReport>,
    ) -> bool {
        PROBE_FRAME.store(frame as u64, Ordering::Relaxed);
        if self.rect_frames.contains(&frame) {
            let configured = PROBE_CONFIGURED
                .lock()
                .map(|map| map.clone())
                .unwrap_or_default();
            tracing::info!(frame, objects = %api.probe_objects(&configured), "PROBERECTS");
        }
        if let Some((x, y)) = self.release.take() {
            apply_headless_input_event(api, HeadlessInputEvent::MouseRelease { x, y });
            return false;
        }
        if std::mem::take(&mut self.key_release) {
            apply_headless_input_event(
                api,
                HeadlessInputEvent::KeyRelease {
                    key: "enter".into(),
                },
            );
            return false;
        }
        if frame < self.start_frame {
            return false;
        }
        let lines = PROBE_LINES.load(Ordering::Relaxed);
        if frame == self.start_frame {
            self.last_progress = frame;
            self.last_lines = lines;
            self.lines_at_start = lines;
        }
        if let Some(script) = self.running_macro.as_mut() {
            if let Some(event) = script.tick() {
                apply_headless_input_event(api, event);
            } else if script.is_finished() {
                self.running_macro = None;
                self.last_progress = frame;
                self.last_lines = lines;
                self.stall_logged = false;
                tracing::info!(frame, lines, macro_no = self.macro_no, "PROBEMACROEND");
            }
            return false;
        }
        if self
            .macros
            .front()
            .is_some_and(|item| lines - self.lines_at_start >= item.lines)
        {
            let item = self.macros.pop_front().expect("checked above");
            self.macro_no += 1;
            match HeadlessInputScript::from_source(&item.source) {
                Ok(script) => {
                    tracing::info!(
                        frame,
                        lines,
                        macro_no = self.macro_no,
                        at = item.lines,
                        "PROBEMACRO"
                    );
                    if self.ctrl_down {
                        apply_headless_input_event(
                            api,
                            HeadlessInputEvent::KeyRelease { key: "ctrl".into() },
                        );
                        self.ctrl_down = false;
                    }
                    self.running_macro = Some(script);
                }
                Err(_) => tracing::warn!(frame, macro_no = self.macro_no, "PROBEMACROBAD"),
            }
            return false;
        }
        if self.stop_lines > 0 && lines - self.lines_at_start >= self.stop_lines {
            self.summary(frame, lines, "lines");
            return true;
        }
        if !self.auto_mode && !self.ctrl_down {
            apply_headless_input_event(api, HeadlessInputEvent::KeyPress { key: "ctrl".into() });
            self.ctrl_down = true;
            return false;
        }
        if lines != self.last_lines {
            if self.stall_logged {
                tracing::info!(
                    frame,
                    stalled_frames = frame - self.last_progress,
                    lines,
                    "PROBERESUME"
                );
            }
            self.last_lines = lines;
            self.last_progress = frame;
            self.stall_logged = false;
        }
        if frame % 600 == 0 {
            tracing::info!(
                frame,
                lines,
                choices = self.choice_no,
                stalls = self.stalls,
                "PROBEPROG"
            );
        }
        let idle = (frame - self.last_progress) as u64;
        if !self.auto_mode && self.repress > 0 && idle > 0 && idle % self.repress == 0 {
            // Released here, pressed again on the next frame.
            apply_headless_input_event(api, HeadlessInputEvent::KeyRelease { key: "ctrl".into() });
            self.ctrl_down = false;
            if idle < self.stall {
                return false;
            }
        }
        if idle < self.stall {
            return false;
        }

        // Choice candidates: live, running processors configured since the
        // last line, with clickable items.
        let configured = PROBE_CONFIGURED
            .lock()
            .map(|map| map.clone())
            .unwrap_or_default();
        let mut candidates = Vec::new();
        for (&object, input) in &api.graph_input_objects {
            if !input.is_live() || !input.is_running() {
                continue;
            }
            let Some(&(seq, lines_at)) = configured.get(&object) else {
                continue;
            };
            let eligible: Vec<ItemRect> = api
                .probe_item_rects(object)
                .into_iter()
                .filter(|rect| rect.eligible && rect.top >= 0.0 && rect.top + rect.height <= 960.0)
                .collect();
            let tops: BTreeSet<i32> = eligible.iter().map(|rect| rect.top as i32).collect();
            // The message window's button row: three or more items on one line.
            let button_row = tops.len() == 1 && eligible.len() >= 3;
            let retry_due = self.last_click.is_some_and(|(o, q, at, l)| {
                self.retry > 0
                    && o == object
                    && q == seq
                    && l == lines
                    && (frame - at) as u64 >= self.retry
            });
            if lines_at == lines
                && !eligible.is_empty()
                && !button_row
                && (!self.answered.contains(&(object, seq)) || retry_due)
            {
                candidates.push((seq, object, eligible));
            }
        }
        if !self.stall_logged {
            self.stall_logged = true;
            self.stalls += 1;
            self.log_state(frame, idle, lines, api, vm, last, &configured);
        }
        // Newest configured processor first.
        candidates.sort_by(|a, b| b.0.cmp(&a.0));
        if let Ok(title) = std::env::var("PROBE_TITLE") {
            let want: Vec<i32> = title
                .split(',')
                .filter_map(|v| v.trim().parse().ok())
                .collect();
            if candidates.iter().any(|(_, _, items)| {
                let r = items[0];
                want == [r.left as i32, r.top as i32, r.width as i32, r.height as i32]
            }) {
                self.summary(frame, lines, "title");
                return true;
            }
        }
        if let Some((seq, object, items)) = candidates.into_iter().next() {
            let planned = self.choices.get(self.choice_no).copied().unwrap_or(0);
            let first = items[0];
            let signature = (
                items.len(),
                first.left as i32,
                first.top as i32,
                first.width as i32,
                first.height as i32,
            );
            let loop_lines = env_u64("PROBE_LOOP_LINES", 50);
            let repeats = match self.menus.get(&signature) {
                Some(&(at, repeats))
                    if lines.saturating_sub(at) <= loop_lines
                        && std::env::var_os("PROBE_NOLOOP").is_none() =>
                {
                    repeats + 1
                }
                _ => 0,
            };
            self.menus.insert(signature, (lines, repeats));
            if repeats > 0 {
                tracing::info!(frame, repeats, items = items.len(), "PROBELOOP");
            }
            let picked = (planned.min(items.len() - 1) + repeats) % items.len();
            let item = items[picked];
            let x = (item.left + item.width * 0.5).round();
            let y = (item.top + item.height * 0.5).round();
            if self.answered.contains(&(object, seq)) {
                tracing::info!(frame, object, "PROBERETRY");
            }
            tracing::info!(
                frame,
                choice = self.choice_no,
                items = items.len(),
                planned,
                picked,
                x,
                y,
                lines,
                "PROBECHOICE"
            );
            self.answered.insert((object, seq));
            self.last_click = Some((object, seq, frame, lines));
            self.choice_no += 1;
            apply_headless_input_event(api, HeadlessInputEvent::MouseMove { x, y });
            apply_headless_input_event(api, HeadlessInputEvent::MousePress { x, y });
            self.release = Some((x, y));
            self.last_progress = frame;
            self.stall_logged = false;
            return false;
        }
        if idle >= self.giveup {
            self.log_state(frame, idle, lines, api, vm, last, &configured);
            self.summary(frame, lines, "gave-up");
            return true;
        }
        if self.nudge > 0 && idle % self.nudge == 0 {
            tracing::info!(frame, idle, lines, "PROBENUDGE");
            apply_headless_input_event(
                api,
                HeadlessInputEvent::KeyPress {
                    key: "enter".into(),
                },
            );
            self.key_release = true;
        }
        false
    }

    #[allow(clippy::too_many_arguments)]
    fn log_state(
        &self,
        frame: usize,
        idle: u64,
        lines: u64,
        api: &RuntimeTraceApi,
        vm: &ethornell_vm::Vm,
        last: Option<&ethornell_vm::VmRunReport>,
        configured: &BTreeMap<i32, (u64, u64)>,
    ) {
        let reason = last
            .map(|report| format!("{:?}", report.stop_reason))
            .unwrap_or_default();
        let reason_kind: String = reason
            .chars()
            .take_while(|ch| ch.is_ascii_alphanumeric())
            .collect();
        tracing::info!(
            frame,
            idle,
            lines,
            pc = last.map(|report| report.pc),
            reason = %reason_kind,
            stack = vm.stack.len(),
            yield_blockers = %api.vm_after_yield_blockers().join(","),
            message_active = api.native_message_active,
            animating = api.native_message_is_animating(),
            scenario_offset = format_args!(
                "0x{:08X}",
                read_vm_u32(&vm.memory, 0x0004_cc70).unwrap_or_default()
            ),
            objects = %api.probe_objects(configured),
            "PROBESTALL"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_hash_is_fnv1a_64() {
        assert_eq!(line_hash(""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(line_hash("a"), 0xaf63_dc4c_8601_ec8c);
    }

    #[test]
    fn pokes_write_little_endian_words_inside_memory_only() {
        let mut memory = vec![0u8; 16];
        apply_pokes(&mut memory, &[(4, 0x0102_0304), (14, 7)]);
        assert_eq!(&memory[4..8], &[4, 3, 2, 1]);
        assert_eq!(&memory[12..16], &[0, 0, 0, 0]);
    }
}
