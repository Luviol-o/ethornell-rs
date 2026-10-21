use super::*;

const PARTICLE_HANDLE_BASE: i32 = 0xC000_0000u32 as i32;
const MAX_PARTICLE_SCREENS: usize = 8;
const MAX_SPLINE_POINTS: usize = 100;

#[derive(Debug, Default)]
pub(super) struct NativeEffectState {
    particle_screens: BTreeMap<i32, NativeParticleScreen>,
    particle_banks: native_particle::ParticleBanks,
    particle_interpolation_mode: i32,
    splines: BTreeMap<i32, NativeSpline>,
    next_spline_handle: i32,
}

#[derive(Debug, Clone)]
struct NativeParticleScreen {
    handle: i32,
    width: i32,
    height: i32,
    enabled: bool,
    display: [i32; 5],
    frame_count: usize,
    frame_tables: [Vec<i32>; 2],
    native_field_340: i32,
    auto_update_interval_ms: i32,
    auto_update_deadline: u32,
    commit_count: u64,
    emitter_layout: Vec<i32>,
    emission_percent: i32,
    advance_count: u64,
    emitter_configuration: Vec<i32>,
    world: native_particle::ParticleWorld,
    layers: Vec<i32>,
}

impl NativeParticleScreen {
    fn new(handle: i32, width: i32, height: i32) -> Self {
        Self {
            handle,
            width,
            height,
            enabled: true,
            display: [0, 0, 0, 128, 0],
            frame_count: 1,
            frame_tables: [vec![0x7fff], vec![0]],
            native_field_340: 4096,
            auto_update_interval_ms: 0,
            auto_update_deadline: 0,
            commit_count: 0,
            emitter_layout: Vec::new(),
            emission_percent: 0,
            advance_count: 0,
            emitter_configuration: Vec::new(),
            world: {
                let mut world = native_particle::ParticleWorld::default();
                world.center = [width / 2, height / 2];
                world
            },
            layers: Vec::new(),
        }
    }

    fn commit(&mut self) {
        self.commit_count = self.commit_count.saturating_add(1);
    }
}

#[derive(Debug, Clone)]
pub(super) struct NativeSpline {
    duration: i32,
    points: Vec<[i32; 3]>,
}

impl NativeEffectState {
    fn allocate_particle_screen(&mut self, width: i32, height: i32) -> Option<i32> {
        let slot = (0..MAX_PARTICLE_SCREENS).find(|slot| {
            !self
                .particle_screens
                .contains_key(&(PARTICLE_HANDLE_BASE | *slot as i32))
        })?;
        let handle = PARTICLE_HANDLE_BASE | slot as i32;
        self.particle_screens
            .insert(handle, NativeParticleScreen::new(handle, width, height));
        Some(handle)
    }

    fn release_particle_screen(&mut self, handle: i32) -> bool {
        self.particle_screens.remove(&handle).is_some()
    }

    fn particle_screen_mut(&mut self, handle: i32) -> Option<&mut NativeParticleScreen> {
        self.particle_screens.get_mut(&handle)
    }

    pub(super) fn configure_particle_frame_tables(
        &mut self,
        handle: i32,
        first: &[i32],
        second: &[i32],
    ) -> bool {
        let Some(screen) = self.particle_screens.get_mut(&handle) else {
            return false;
        };
        if first.is_empty()
            || first.len() > 16
            || first.len() != second.len()
            || second
                .iter()
                .any(|offset| !(0..=65535).contains(&screen.display[2].wrapping_add(*offset)))
        {
            return false;
        }
        screen.frame_count = first.len();
        screen.frame_tables = [
            first.to_vec(),
            if first.len() == 1 {
                vec![0]
            } else {
                second.to_vec()
            },
        ];
        true
    }

    pub(super) fn create_spline(&mut self) -> i32 {
        let handle = self.next_spline_handle;
        self.next_spline_handle = self.next_spline_handle.wrapping_add(1);
        self.splines.insert(
            handle,
            NativeSpline {
                duration: 0,
                points: Vec::new(),
            },
        );
        handle
    }

    pub(super) fn release_spline(&mut self, handle: i32) -> i32 {
        if self.splines.remove(&handle).is_some() {
            0
        } else {
            1
        }
    }

    pub(super) fn spline_exists(&self, handle: i32) -> bool {
        self.splines.contains_key(&handle)
    }

    pub(super) fn configure_spline(
        &mut self,
        handle: i32,
        duration: i32,
        points: &[[i32; 3]],
    ) -> i32 {
        if points.len() < 2 {
            return 2;
        }
        if duration < 2 {
            return 3;
        }
        let Some(spline) = self.splines.get_mut(&handle) else {
            return 1;
        };
        spline.duration = duration;
        spline.points.clear();
        spline
            .points
            .extend(points.iter().take(MAX_SPLINE_POINTS).copied());
        0
    }

    pub(super) fn sample_spline(
        &self,
        handle: i32,
        time: i32,
    ) -> std::result::Result<[i32; 3], i32> {
        let Some(spline) = self.splines.get(&handle) else {
            return Err(1);
        };
        if time < 0 || time >= spline.duration {
            return Err(4);
        }
        if spline.duration <= 0 || spline.points.is_empty() {
            return Err(-1);
        }
        let t = f64::from(time) / f64::from(spline.duration);
        let mut output = [0; 3];
        for axis in 0..3 {
            let values = spline
                .points
                .iter()
                .map(|point| f64::from(point[axis]))
                .collect::<Vec<_>>();
            output[axis] = sample_natural_cubic(&values, t) as i32;
        }
        Ok(output)
    }
}

fn sample_natural_cubic(values: &[f64], normalized: f64) -> f64 {
    match values {
        [] => 0.0,
        [value] => *value,
        [first, second] => first + (second - first) * normalized.clamp(0.0, 1.0),
        _ => {
            let segments = values.len() - 1;
            let scaled = normalized.clamp(0.0, 1.0) * segments as f64;
            let segment = (scaled.floor() as usize).min(segments - 1);
            let local = scaled - segment as f64;

            // The target CSpline builds a natural cubic spline independently
            // for x/y/z. Equal knot spacing lets us solve the tridiagonal
            // second-derivative system with this compact Thomas pass.
            let n = values.len();
            let mut lower = vec![0.0; n];
            let mut diagonal = vec![1.0; n];
            let mut upper = vec![0.0; n];
            let mut rhs = vec![0.0; n];
            for index in 1..n - 1 {
                lower[index] = 1.0;
                diagonal[index] = 4.0;
                upper[index] = 1.0;
                rhs[index] = 6.0 * (values[index + 1] - 2.0 * values[index] + values[index - 1]);
            }
            for index in 1..n {
                let factor = lower[index] / diagonal[index - 1];
                diagonal[index] -= factor * upper[index - 1];
                rhs[index] -= factor * rhs[index - 1];
            }
            let mut second = vec![0.0; n];
            for index in (0..n).rev() {
                second[index] = if index + 1 < n {
                    (rhs[index] - upper[index] * second[index + 1]) / diagonal[index]
                } else {
                    rhs[index] / diagonal[index]
                };
            }
            let a = 1.0 - local;
            let b = local;
            a * values[segment]
                + b * values[segment + 1]
                + ((a * a * a - a) * second[segment] + (b * b * b - b) * second[segment + 1]) / 6.0
        }
    }
}

impl RuntimeTraceApi {
    pub(super) fn tick_native_effect_state(&mut self, elapsed_ms: u64) {
        let mut state = std::mem::take(&mut self.native_effect);
        let now = self.engine_time_ms as u32;
        let mut due = Vec::new();
        for (&handle, screen) in &mut state.particle_screens {
            screen.world.poll(now, &state.particle_banks, &mut |n| {
                self.native_particle_random(n)
            });
            let interval = screen.auto_update_interval_ms as u32;
            if interval != 0 && screen.auto_update_deadline <= now {
                if self.graph_object_is_drawable(handle) {
                    screen.commit();
                    due.push(handle);
                }
                if screen.auto_update_deadline == 0 {
                    screen.auto_update_deadline = now;
                }
                screen.auto_update_deadline = screen
                    .auto_update_deadline
                    .wrapping_add(interval.wrapping_mul(
                        now.wrapping_sub(screen.auto_update_deadline) / interval + 1,
                    ));
            }
        }
        for handle in due {
            self.render_native_particle_screen(&mut state, handle, now, false);
        }
        self.native_effect = state;
        self.effects.tick(elapsed_ms);
    }

    fn dispatch_particle_native(
        &mut self,
        id: u16,
        stack: &mut Vec<ethornell_vm::Value>,
    ) -> ethornell_vm::VmResult<ethornell_vm::Value> {
        use ethornell_vm::{Value, VmError};
        let count = match id {
            0 => 2,
            1 | 8 | 15 | 31 => 1,
            4 | 9 | 10 | 12 | 13 | 36 | 44 => 2,
            5 => 6,
            11 => 10,
            16 => 12,
            24 => 7,
            26 => 8,
            27 => 3,
            32 | 40 => 4,
            37 => 11,
            41 => 16,
            45 => 18,
            _ => {
                return Err(VmError::Runtime(format!(
                    "unregistered particle syscall C0:{id:02X}"
                )));
            }
        };
        let mut a = pop_args(stack, count)
            .iter()
            .map(value_to_i32)
            .collect::<Vec<_>>();
        a.reverse();
        let mut state = std::mem::take(&mut self.native_effect);
        let fail = |reason: &str| VmError::Runtime(format!("C0:{id:02X}: {reason}"));
        let now = self.engine_time_ms as u32;
        let result = (|| -> ethornell_vm::VmResult<Value> {
            let mut render = None;
            match id {
                0 => {
                    if a[0] <= 0
                        || a[1] <= 0
                        || (a[0] as usize)
                            .checked_mul(a[1] as usize)
                            .is_none_or(|n| n > 16 * 1024 * 1024)
                    {
                        return Err(fail("invalid screen dimensions"));
                    }
                    let handle = state
                        .allocate_particle_screen(a[0], a[1])
                        .ok_or_else(|| fail("all eight particle screens are occupied"))?;
                    state.particle_screens.get_mut(&handle).unwrap().world.clock = now;
                    self.display_tree.register_inferred(handle);
                    self.graph_object_enabled.insert(handle, true);
                    let p = self.graph_object_properties.entry(handle).or_default();
                    p.native.sort_class = 4;
                    p.native.draw_enabled = 1;
                    self.graph90_refresh_native_sort_key(handle);
                    self.render_native_particle_screen(&mut state, handle, now, false);
                    return Ok(Value::Int(handle));
                }
                1 => {
                    if !state.release_particle_screen(a[0]) {
                        return Err(fail("invalid particle screen"));
                    }
                    self.remove_graph_object(a[0]);
                }
                4 => {
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    s.enabled = a[1] != 0;
                    self.set_graph_object_enabled(a[0], s.enabled);
                }
                5 => {
                    if !is_native_blend_mode(a[3]) || a[4] as u32 > 256 || a[5] as u32 >= 65536 {
                        return Err(fail("invalid blend, transparency or priority"));
                    }
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    s.display = [a[1], a[2], a[5], a[3], a[4]];
                    let p = self.graph_object_properties.entry(a[0]).or_default();
                    p.blend_mode = a[3];
                    p.native.blend_mode = a[3];
                    p.set_alpha_parameter(a[4]);
                    p.native.priority = a[5] as u32;
                    self.graph90_set_position_recursive(a[0], a[1], a[2]);
                    self.display_tree.set_chain_depth(a[0], a[5]);
                    self.graph90_refresh_native_sort_key(a[0]);
                    render = Some((a[0], false));
                }
                8 => {
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    s.commit();
                    render = Some((a[0], false));
                }
                9 => {
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    if s.auto_update_interval_ms == 0 && a[1] != 0 {
                        s.auto_update_deadline = now;
                    }
                    s.auto_update_interval_ms = a[1];
                }
                10 => {
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    s.native_field_340 = a[1];
                }
                11 => {
                    if a[7] <= 0 {
                        return Err(fail("perspective distance must be positive"));
                    }
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    s.emitter_layout = a[1..].to_vec();
                    s.world.camera = [a[1] >> 8, a[2] >> 8, a[3] >> 8];
                    s.world.rotation =
                        [a[4], a[5], a[6]].map(|n| n as f64 * std::f64::consts::PI / 11796480.0);
                    s.world.focal = a[7];
                    s.world.center = [a[8], a[9]];
                }
                12 => {
                    if a[1] as u32 > 100 {
                        return Err(fail("simulation interval exceeds 100"));
                    }
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    s.emission_percent = a[1];
                    s.world.interval = a[1] as u32;
                    s.world.retime_environment();
                }
                13 => {
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    let mut rng = |n| self.native_particle_random(n);
                    s.world
                        .warm_up(a[1] as u32, now, &state.particle_banks, &mut rng);
                    s.advance_count = s.advance_count.wrapping_add(a[1] as u32 as u64);
                }
                15 => {
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    s.world.reset();
                    s.commit_count = 0;
                    s.advance_count = 0;
                }
                16 => {
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    s.emitter_configuration = a[1..].to_vec();
                    s.world
                        .configure_environment(&a, &mut |n| self.native_particle_random(n));
                }
                24 | 26 | 36 | 44 => {
                    let (kind, bank, frames, first, speed, mix) = if id == 36 || id == 44 {
                        (i32::from(id == 44), a[0], 1, a[1], [0, 0], None)
                    } else {
                        let numerator = (a[6] as u32).wrapping_shl(16);
                        let base = if a[4] != 0 {
                            numerator / (a[4] as u32)
                        } else {
                            0
                        };
                        let spread = match state.particle_interpolation_mode {
                            0 => {
                                if a[5] != 0 {
                                    numerator / (a[5] as u32)
                                } else {
                                    0
                                }
                            }
                            1 => {
                                if a[4] != 0 && a[5] != 0 {
                                    let d = a[4].wrapping_add(a[5]) as u32;
                                    if d != 0 {
                                        (numerator / d).wrapping_sub(base)
                                    } else {
                                        0
                                    }
                                } else {
                                    0
                                }
                            }
                            _ => {
                                if a[4] != 0 {
                                    (u64::from(base) * u64::from(a[5] as u32)
                                        / (a[4] as u32 as u64))
                                        as u32
                                } else {
                                    0
                                }
                            }
                        };
                        (
                            a[0],
                            a[1],
                            a[2],
                            a[3],
                            [base as i32, spread as i32],
                            (id == 26).then(|| a[7]),
                        )
                    };
                    if kind as u32 >= 2 || bank as u32 >= 64 || id == 26 && kind != 1 {
                        return Err(fail("invalid particle kind or animation bank"));
                    }
                    if frames as u32 > 32 {
                        return Err(fail("animation bank exceeds 32 frames"));
                    }
                    let mut images = Vec::new();
                    let mut format = 0;
                    let mut dimensions = None;
                    if frames != 0 && first != -1 {
                        for offset in 0..frames {
                            let resource = first.wrapping_add(offset);
                            let image = self
                                .graph_bitmap_image(resource)
                                .ok_or_else(|| fail("missing animation bitmap"))?;
                            let f = self.bitmap_formats.get(&resource).copied().unwrap_or(2);
                            let shape = (image.width, image.height, f);
                            if !matches!(f, 1 | 2) || dimensions.is_some_and(|d| d != shape) {
                                return Err(fail(
                                    "animation frames must share RGB/ARGB format and dimensions",
                                ));
                            }
                            format = f;
                            dimensions = Some(shape);
                            images.push(image);
                        }
                    }
                    let animation = native_particle::ParticleBank::new(images, format, speed);
                    if let Some(mix) = mix {
                        state.particle_banks.secondary.insert(bank, animation);
                        state.particle_banks.mix.insert(bank, mix);
                        for screen in state.particle_screens.values_mut() {
                            screen
                                .world
                                .rebind_secondary(bank, &state.particle_banks, &mut |n| {
                                    self.native_particle_random(n)
                                });
                        }
                    } else {
                        state.particle_banks.primary.insert((kind, bank), animation);
                    }
                }
                27 => {
                    if a[0] != 1
                        || a[1] as u32 >= 64
                        || !state
                            .particle_banks
                            .secondary
                            .get(&a[1])
                            .is_some_and(|b| !b.frames.is_empty())
                    {
                        return Err(fail("secondary animation bank is absent"));
                    }
                    state.particle_banks.mix.insert(a[1], a[2]);
                }
                31 => {
                    if a[0] as u32 > 2 {
                        return Err(fail("invalid interpolation mode"));
                    }
                    state.particle_interpolation_mode = a[0];
                }
                32 | 40 => {
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    if !s.world.configure_count(
                        usize::from(id == 40),
                        a[1] as usize,
                        a[2],
                        a[3] as u32,
                    ) {
                        return Err(fail(
                            "invalid emitter bank or aggregate count exceeds 32768",
                        ));
                    }
                }
                37 => {
                    if a[0] as u32 >= 64 {
                        return Err(fail("invalid snow style bank"));
                    }
                    let mut style = [0i32; 11];
                    style[0] = 1;
                    for (i, value) in a[1..10].iter().enumerate() {
                        style[i + 1] = if matches!(i, 0 | 2 | 4 | 6 | 8) {
                            value.wrapping_abs() >> 8
                        } else {
                            *value >> 8
                        };
                    }
                    style[10] = a[10] >> 8;
                    state.particle_banks.snow.insert(a[0], style);
                }
                41 => {
                    let s = state
                        .particle_screens
                        .get_mut(&a[0])
                        .ok_or_else(|| fail("invalid particle screen"))?;
                    if a[1] as u32 >= 64 {
                        return Err(fail("invalid firefly emitter bank"));
                    }
                    let e = &mut s.world.emitter[a[1] as usize];
                    e.copy_from_slice(&a[2..16]);
                    for i in [0usize, 2, 12] {
                        e[i] >>= 8;
                    }
                    for i in [1usize, 4, 5, 13] {
                        e[i] = e[i].wrapping_abs() >> 8;
                    }
                    for i in [6usize, 7, 8] {
                        e[i] = e[i].wrapping_abs();
                    }
                }
                45 => {
                    if a[0] as u32 >= 64 {
                        return Err(fail("invalid firefly style bank"));
                    }
                    let mut s = [0i32; 18];
                    s[0] = 1;
                    s[1] = a[1];
                    s[2] = a[2];
                    for i in 3..12 {
                        s[i] = if matches!(i, 3 | 5 | 7 | 9 | 11) {
                            a[i].wrapping_abs() >> 8
                        } else {
                            a[i] >> 8
                        };
                    }
                    s[12] = a[12];
                    s[13] = a[13];
                    s[14] = a[14] >> 8;
                    s[15] = if a[15] == 0 { 1 } else { a[15] };
                    s[16] = if a[16] == 0 { 1 } else { a[16] };
                    s[17] = a[17];
                    state.particle_banks.firefly.insert(a[0], s);
                }
                _ => unreachable!(),
            }
            if let Some((handle, poll)) = render {
                self.render_native_particle_screen(&mut state, handle, now, poll);
            }
            Ok(Value::None)
        })();
        self.native_effect = state;
        result
    }

    fn native_particle_random(&mut self, n: i32) -> i32 {
        let value =
            ((self.next_native_crt_rand() as u32) << 15) | self.next_native_crt_rand() as u32;
        let divisor = n.wrapping_add(1).unsigned_abs();
        let r = if divisor == 0 {
            0
        } else {
            (value % divisor) as i32
        };
        if n < 0 { r.wrapping_neg() } else { r }
    }

    pub(super) fn render_native_particle_screen(
        &mut self,
        state: &mut NativeEffectState,
        handle: i32,
        now: u32,
        poll: bool,
    ) {
        let Some(screen) = state.particle_screens.get_mut(&handle) else {
            return;
        };
        if poll {
            screen.world.poll(now, &state.particle_banks, &mut |n| {
                self.native_particle_random(n)
            });
        }
        if let Some((x, y)) = self.graph_native_base_position(handle) {
            screen.display[0] = x;
            screen.display[1] = y;
        }
        if let Some(p) = self.graph_object_properties.get(&handle) {
            screen.display[2] = p.native.priority as i32;
        }
        let images = screen.world.raster(
            screen.width as u32,
            screen.height as u32,
            &screen.frame_tables[0],
            &state.particle_banks,
        );
        while screen.layers.len() > images.len() {
            let layer = screen.layers.pop().unwrap();
            self.graph_layers.remove(&layer);
            if let Some(layers) = self.graph_object_layers.get_mut(&handle) {
                layers.remove(&layer);
            }
        }
        for (index, image) in images.into_iter().enumerate() {
            let layer = if let Some(layer) = screen.layers.get(index) {
                *layer
            } else {
                let id = self.alloc_node();
                screen.layers.push(id);
                id
            };
            let key = format!("runtime:particle:{handle}:{index}");
            let (width, height) = (image.width as f32, image.height as f32);
            self.store_graph_image(key.clone(), image);
            let priority = screen.display[2]
                .wrapping_add(screen.frame_tables[1].get(index).copied().unwrap_or(0));
            self.graph_layers.insert(
                layer,
                RuntimeGraphLayer {
                    hit_id: handle,
                    owner_object: Some(handle),
                    key,
                    target_surface: None,
                    x: screen.display[0] as f32,
                    y: screen.display[1] as f32,
                    width,
                    height,
                    src_x: 0.0,
                    src_y: 0.0,
                    opacity: 1.0,
                    z: priority,
                    enabled: true,
                    transform_x: 0.0,
                    transform_y: 0.0,
                    transform_z: 0,
                    scale_x: 1.0,
                    scale_y: 1.0,
                    rotation_degrees: 0.0,
                    clip: None,
                },
            );
            self.graph_object_layers
                .entry(handle)
                .or_default()
                .insert(layer);
        }
        self.graph_redraw_requested = Some(false);
    }

    pub(super) fn dispatch_native_effect(
        &mut self,
        call: &mut ethornell_vm::NativeCallFrame,
    ) -> Option<ethornell_vm::VmResult<ethornell_vm::Value>> {
        let (group, id) = (call.group(), call.id());
        let stack = call.args_mut();
        if group != 0xC0 {
            return None;
        }

        let value = match id {
            0x00..=0x2D => return Some(self.dispatch_particle_native(id, stack)),
            0x40..=0x4F => {
                let count = match id {
                    0x40 => 2,
                    0x41 => 1,
                    0x42..=0x44 | 0x47..=0x4B | 0x4E | 0x4F => 2,
                    0x45 => 6,
                    0x46 => 7,
                    0x4C | 0x4D => 4,
                    _ => unreachable!(),
                };
                let args = pop_args(stack, count);
                let result = match self.effects.call(group, id, &args) {
                    crate::effects::EffectCall::Handled(result) => result,
                    crate::effects::EffectCall::Unhandled => None,
                };
                self.sync_rain_screen_object(id, &args, result);
                result
                    .map(ethornell_vm::Value::Int)
                    .unwrap_or(ethornell_vm::Value::None)
            }
            0xC0 => ethornell_vm::Value::Int(self.native_effect.create_spline()),
            0xC1 => {
                let handle = pop_int_value(stack).unwrap_or_default();
                ethornell_vm::Value::Int(self.native_effect.release_spline(handle))
            }
            // C2/C3 are VM-owned because their target contracts dereference
            // caller memory. Reaching the host path indicates malformed routing.
            0xC2 => {
                let _ = pop_args(stack, 4);
                ethornell_vm::Value::Int(1)
            }
            0xC3 => {
                let _ = pop_args(stack, 3);
                ethornell_vm::Value::Int(4)
            }
            // F0 is VM-owned so it can write the BWEF table and count directly.
            0xF0 => {
                let _ = pop_args(stack, 5);
                ethornell_vm::Value::Int(i32::MIN + 1)
            }
            _ => return None,
        };
        Some(Ok(value))
    }

    /// A rain screen (UserC0:40, tag 0xC1) is a display object like a
    /// particle screen (UserC0:00): scripts fade it with Graph90:22 and
    /// reorder it with Graph90:3A, so it is registered with the display tree
    /// and its enabled state, depth, blend mode and alpha live in the object
    /// state those calls act on. `popped` is in reverse-pop order, so the
    /// handle (the first script argument) is last.
    fn sync_rain_screen_object(
        &mut self,
        id: u16,
        popped: &[ethornell_vm::Value],
        created: Option<i32>,
    ) {
        let handle = popped.last().map(value_to_i32).unwrap_or_default();
        match id {
            0x40 => {
                if let Some(handle) = created.filter(|handle| *handle != 0) {
                    self.display_tree.register_inferred(handle);
                    self.graph_object_enabled.insert(handle, true);
                    self.graph_object_properties.entry(handle).or_default();
                }
            }
            _ if NativeDisplayKind::from_handle(handle) != NativeDisplayKind::RainScreen
                || !self.display_tree.contains(handle) => {}
            0x41 => {
                self.remove_graph_object(handle);
            }
            0x44 => {
                let enabled = popped.first().map(value_to_i32).unwrap_or_default() != 0;
                self.set_graph_object_enabled(handle, enabled);
            }
            0x45 => {
                let values = popped.iter().rev().map(value_to_i32).collect::<Vec<_>>();
                if let [_, _, _, z, blend, alpha] = values.as_slice()
                    && is_native_blend_mode(*blend)
                    && (0..=256).contains(alpha)
                {
                    let properties = self.graph_object_properties.entry(handle).or_default();
                    properties.blend_mode = *blend;
                    properties.set_alpha_parameter(*alpha);
                    self.display_tree.set_chain_depth(handle, *z);
                }
            }
            _ => {}
        }
    }
}

pub(super) fn is_native_blend_mode(value: i32) -> bool {
    matches!(
        value,
        0x00..=0x09 | 0x20..=0x27 | 0x40 | 0x41 | 0x80 | 0xC0 | 0xC1 | 0xF0 | 0xFF
    )
}

#[cfg(test)]
mod tests {
    use super::{NativeEffectState, PARTICLE_HANDLE_BASE, sample_natural_cubic};

    #[test]
    fn snow_syscalls_copy_frames_advance_and_render_depth_planes() {
        use crate::*;
        use ethornell_vm::SoundApi;
        let manager = ResourceManager::open_game(env!("CARGO_MANIFEST_DIR")).unwrap();
        let mut api = RuntimeTraceApi::new(manager);
        let call = |api: &mut RuntimeTraceApi, id, args: &[i32]| {
            api.dispatch_particle_native(
                id,
                &mut args.iter().copied().map(ethornell_vm::Value::Int).collect(),
            )
            .unwrap()
        };
        api.store_graph_image(
            "test:snow".into(),
            DecodedImage {
                width: 8,
                height: 8,
                rgba: vec![255; 256],
            },
        );
        api.graph_resources
            .insert(1, RuntimeGraphResource::whole("test:snow".into()));
        api.bitmap_formats.insert(1, 2);
        let handle = call(&mut api, 0, &[64, 64]).as_i32();
        assert!(SoundApi::configure_particle_frame_tables(
            &mut api,
            handle,
            &[1, 32767],
            &[0, 1]
        ));
        call(&mut api, 0x0B, &[handle, 0, 0, 0, 0, 0, 0, 256, 32, 32]);
        call(&mut api, 0x24, &[0, 1]);
        call(&mut api, 0x25, &[0, 0, 0, 0, 65536, 0, 0, 0, 0, 0, 0]);
        call(&mut api, 0x20, &[handle, 0, 1, 0]);
        // Bank owns the copied native image, including after source release.
        api.graph_resources.remove(&1);
        api.tick_native_effect_state(0);
        call(&mut api, 8, &[handle]);
        let key = format!("runtime:particle:{handle}:0");
        assert_eq!(api.graph_images[&key].rgba[(28 * 64 + 28) * 4 + 3], 255);
        api.engine_time_ms = 20;
        api.tick_native_effect_state(20);
        call(&mut api, 8, &[handle]);
        assert_eq!(api.graph_images[&key].rgba[(28 * 64 + 28) * 4 + 3], 0);
        assert_eq!(api.graph_images[&key].rgba[(28 * 64 + 30) * 4 + 3], 255);
        assert_eq!(api.graph_object_layers[&handle].len(), 2);
        call(&mut api, 0x0F, &[handle]);
        api.tick_native_effect_state(0);
        call(&mut api, 8, &[handle]);
        // Reset clears live particles; requested emitter counts persist and
        // cause the next commit to emit a fresh particle at its initial point.
        assert_eq!(api.graph_images[&key].rgba[(28 * 64 + 28) * 4 + 3], 255);
    }

    #[test]
    fn particle_screen_uses_target_tag_and_eight_slots() {
        let mut state = NativeEffectState::default();
        for slot in 0..8 {
            assert_eq!(
                state.allocate_particle_screen(1280, 720),
                Some(PARTICLE_HANDLE_BASE | slot)
            );
        }
        assert_eq!(state.allocate_particle_screen(1, 1), None);
        assert!(state.release_particle_screen(PARTICLE_HANDLE_BASE | 3));
        assert_eq!(
            state.allocate_particle_screen(640, 480),
            Some(PARTICLE_HANDLE_BASE | 3)
        );
    }

    #[test]
    fn spline_matches_target_status_and_natural_cubic_endpoints() {
        let mut state = NativeEffectState::default();
        let handle = state.create_spline();
        assert_eq!(
            state.configure_spline(handle, 1, &[[0, 0, 0], [1, 1, 1]]),
            3
        );
        assert_eq!(state.configure_spline(99, 10, &[[0, 0, 0], [1, 1, 1]]), 1);
        assert_eq!(
            state.configure_spline(handle, 10, &[[0, 0, 0], [10, 20, 30], [20, 40, 60]]),
            0
        );
        assert_eq!(state.sample_spline(handle, 0), Ok([0, 0, 0]));
        assert_eq!(state.sample_spline(handle, 10), Err(4));
        assert!((sample_natural_cubic(&[0.0, 10.0, 20.0], 0.5) - 10.0).abs() < 0.001);
        assert_eq!(state.release_spline(handle), 0);
        assert_eq!(state.release_spline(handle), 1);
    }
}
