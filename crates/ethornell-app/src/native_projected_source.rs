//! Sprite source caches: 42AA60/42AB80/42AED0 and 4293A0 -> 418720.
use super::*;

#[derive(Debug, Clone)]
pub(crate) struct ProjectionMipCache {
    bindings: Vec<(i32, u64, Vec<DecodedImage>)>,
}
pub(super) struct ProjectedSource {
    pub image: DecodedImage,
    pub level: u32,
    pub anchor_shift: i32,
    pub deformed: bool,
}

/// 419220 averages byte lanes independently, truncating each successive level.
fn half(source: &DecodedImage) -> DecodedImage {
    let mut out =
        crate::native_map_filter::blank(source.width.div_ceil(2), source.height.div_ceil(2));
    for y in 0..out.height {
        for x in 0..out.width {
            let rows = (source.height - y * 2).min(2);
            let cols = (source.width - x * 2).min(2);
            for c in 0..4 {
                let mut sum = 0u16;
                for dy in 0..rows {
                    for dx in 0..cols {
                        sum += u16::from(
                            source.rgba
                                [(((y * 2 + dy) * source.width + x * 2 + dx) * 4) as usize + c],
                        );
                    }
                }
                out.rgba[((y * out.width + x) * 4) as usize + c] =
                    (sum / (rows * cols) as u16) as u8;
            }
        }
    }
    out
}
fn pyramid(mut image: DecodedImage) -> Vec<DecodedImage> {
    let mut levels = Vec::new();
    while levels.len() < 4 && image.width >= 2 && image.height >= 2 {
        image = half(&image);
        levels.push(image.clone());
    }
    levels
}
fn selected_level(scale: u32) -> u32 {
    // The original loops forever for zero scale; only four levels can exist.
    if scale == 0 {
        return 4;
    }
    let mut level = 0;
    let mut threshold = 32768;
    while scale <= threshold && level < 4 {
        level += 1;
        threshold >>= 1;
    }
    level
}
pub(super) fn perspective_ratio(z: i32, focal: u32) -> u32 {
    if z == 0 || focal == 0 {
        return 65536;
    }
    if z < 0 {
        (((u64::from(focal) << 16) + u64::from(z.unsigned_abs())) / u64::from(focal)) as u32
    } else {
        ((u64::from(focal) << 32) / ((u64::from(focal) << 16) + z as u64)) as u32
    }
}
fn row_deform(
    source: &DecodedImage,
    wavelength: u32,
    phase: u32,
    amplitude: i32,
) -> Option<DecodedImage> {
    if wavelength == 0 {
        return None;
    }
    let width = source.width.wrapping_add(
        (source
            .width
            .wrapping_mul(amplitude.wrapping_add(65536) as u32)
            .wrapping_shr(16)
            .wrapping_sub(source.width)
            .wrapping_add(1))
            & !1,
    );
    if width == 0 || u64::from(width) * u64::from(source.height) > 16 * 1024 * 1024 {
        return None;
    }
    let mut out = crate::native_map_filter::blank(width, source.height);
    let start = source.width.wrapping_sub(width).wrapping_shl(15) as i32;
    for y in 0..source.height {
        let angle =
            f64::from(y.wrapping_add(phase)) * std::f64::consts::TAU / f64::from(wavelength);
        let displacement =
            (angle.sin() * f64::from((amplitude as u32).wrapping_mul(source.width))).trunc();
        // x87 FISTP's out-of-range result is the integer-indefinite value.
        let displacement = if displacement < i32::MIN as f64 || displacement > i32::MAX as f64 {
            i32::MIN
        } else {
            displacement as i32
        };
        let offset = start.wrapping_add(displacement >> 1);
        let mut logical = offset >> 16;
        let weight = 16 - ((offset as u32 >> 12) & 15) as i32;
        let mut cursor = 0u32;
        // 418720 advances the source pointer only for an in-bounds logical
        // index. A positive starting offset still reads the first source byte.
        let mut read = |index: i32| -> [u8; 4] {
            if (index as u32) >= source.width || cursor >= source.width {
                return [0; 4];
            }
            let i = ((y * source.width + cursor) * 4) as usize;
            cursor += 1;
            source.rgba[i..i + 4].try_into().unwrap()
        };
        let mut previous = read(logical);
        for x in 0..width {
            logical = logical.wrapping_add(1);
            let next = read(logical);
            let i = ((y * width + x) * 4) as usize;
            for c in 0..4 {
                out.rgba[i + c] = (i32::from(next[c])
                    + ((i32::from(previous[c]) - i32::from(next[c])) * weight >> 4))
                    .clamp(0, 255) as u8;
            }
            previous = next;
        }
    }
    Some(out)
}
impl RuntimeTraceApi {
    pub(super) fn projected_bindings_current(&self, object: i32) -> bool {
        self.native_projection_mips
            .get(&object)
            .is_none_or(|cache| {
                cache.bindings.iter().all(|(id, generation, _)| {
                    self.graph_resources
                        .get(id)
                        .is_some_and(|r| r.generation == *generation)
                })
            })
    }
    pub(super) fn prepare_projected_source(
        &mut self,
        object: i32,
        primary: i32,
        secondary: i32,
        transition: i32,
        scale: u32,
    ) -> Option<ProjectedSource> {
        if !self.projected_bindings_current(object) {
            return None;
        }
        let format = self.bitmap_formats.get(&primary).copied().unwrap_or(2);
        if !matches!(format, 1 | 2) {
            return None;
        }
        let ids = [primary, secondary]
            .into_iter()
            .filter(|id| *id != -1)
            .collect::<Vec<_>>();
        let images = ids
            .iter()
            .map(|id| self.graph_bitmap_image(*id))
            .collect::<Option<Vec<_>>>()?;
        if secondary != -1
            && (self.bitmap_formats.get(&secondary).copied().unwrap_or(2) != format
                || (images[0].width, images[0].height) != (images[1].width, images[1].height))
        {
            return None;
        }
        if !self.native_projection_mips.contains_key(&object) {
            let bindings = ids
                .iter()
                .zip(&images)
                .map(|(&id, img)| {
                    Some((
                        id,
                        self.graph_resources.get(&id)?.generation,
                        pyramid(img.clone()),
                    ))
                })
                .collect::<Option<Vec<_>>>()?;
            self.native_projection_mips
                .insert(object, ProjectionMipCache { bindings });
        }
        let cache = &self.native_projection_mips[&object];
        if cache.bindings.iter().map(|b| b.0).ne(ids.iter().copied()) {
            return None;
        }
        let level = selected_level(scale).min(cache.bindings[0].2.len() as u32);
        let at_level = |i: usize| {
            if level == 0 {
                images[i].clone()
            } else {
                cache.bindings[i].2[(level - 1) as usize].clone()
            }
        };
        let mut image = at_level(0);
        if secondary != -1 {
            image = crate::bitmap_blend::crossfade_cache(&image, &at_level(1), format, transition)?;
        }
        if format == 1 {
            for p in image.rgba.as_chunks_mut::<4>().0 {
                p[3] = 255;
            }
        }
        let (packed, amplitude) = self
            .graph_object_properties
            .get(&object)
            .and_then(|p| p.properties.get(&0x60))
            .copied()
            .unwrap_or((0, 0));
        let wavelength = (packed as u32) >> 16;
        let deformed = wavelength != 0 && amplitude != 0;
        let mut anchor_shift = 0;
        if deformed {
            let original_width = image.width;
            image = row_deform(
                &image,
                wavelength >> level,
                (packed as u32 & 65535) >> level,
                amplitude,
            )?;
            anchor_shift =
                (image.width.wrapping_sub(original_width).wrapping_shl(15) & 0xffff0000) as i32;
        }
        Some(ProjectedSource {
            image,
            level,
            anchor_shift,
            deformed,
        })
    }
    pub(super) fn invalidate_projected_mips(&mut self, object: i32, rect: [i32; 4]) {
        let Some(mut cache) = self.native_projection_mips.remove(&object) else {
            return;
        };
        for (id, generation, levels) in &mut cache.bindings {
            if self
                .graph_resources
                .get(id)
                .is_none_or(|r| r.generation != *generation)
            {
                continue;
            }
            let Some(mut source) = self.graph_bitmap_image(*id) else {
                continue;
            };
            let [mut left, mut top, width, height] = rect;
            let mut right = left.wrapping_add(width).wrapping_sub(1);
            let mut bottom = top.wrapping_add(height).wrapping_sub(1);
            for image in levels {
                if right.wrapping_sub(left) <= 0 {
                    if left < 1 {
                        right = right.wrapping_add(1);
                    } else {
                        left = left.wrapping_sub(1);
                    }
                }
                if bottom.wrapping_sub(top) <= 0 {
                    if top < 1 {
                        bottom = bottom.wrapping_add(1);
                    } else {
                        top = top.wrapping_sub(1);
                    }
                }
                left = (left & !1) >> 1;
                top = (top & !1) >> 1;
                right = (right | 1) >> 1;
                bottom = (bottom | 1) >> 1;
                let fresh = half(&source);
                for y in top.max(0)..=bottom.min(image.height as i32 - 1) {
                    for x in left.max(0)..=right.min(image.width as i32 - 1) {
                        let i = ((y as u32 * image.width + x as u32) * 4) as usize;
                        image.rgba[i..i + 4].copy_from_slice(&fresh.rgba[i..i + 4]);
                    }
                }
                source = image.clone();
            }
        }
        self.native_projection_mips.insert(object, cache);
    }
}
