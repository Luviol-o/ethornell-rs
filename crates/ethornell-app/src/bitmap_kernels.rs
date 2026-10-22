//! Literal ports of BGI 1.622 bitmap kernels, run on the target's pixel
//! layout (B, G, R, A bytes; rows `stride` bytes apart) so that each one
//! can be compared with the original under Unicorn
//! (`src/tools/emu_kernels.py`).
use super::DecodedImage;

/// A bitmap in the target's memory layout (the 24-byte descriptor of
/// `sub_407F10`: pixels, stride, width, height, format).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TargetBitmap {
    pub(crate) pixels: Vec<u8>,
    pub(crate) stride: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl TargetBitmap {
    /// A 32-bit (format 1 or 2) bitmap from an app raster.
    pub(crate) fn from_image(image: &DecodedImage) -> Self {
        let mut pixels = Vec::with_capacity(image.rgba.len());
        for [r, g, b, a] in image.rgba.as_chunks::<4>().0.iter().copied() {
            pixels.extend_from_slice(&[b, g, r, a]);
        }
        Self {
            pixels,
            stride: image.width as usize * 4,
            width: image.width as usize,
            height: image.height as usize,
        }
    }

    /// Back to an app raster; `opaque` forces alpha 255 (format 1 keeps no
    /// alpha).
    pub(crate) fn to_image(&self, opaque: bool) -> DecodedImage {
        let mut rgba = Vec::with_capacity(self.width * self.height * 4);
        for y in 0..self.height {
            let row = &self.pixels[y * self.stride..y * self.stride + self.width * 4];
            for [b, g, r, a] in row.as_chunks::<4>().0.iter().copied() {
                rgba.extend_from_slice(&[r, g, b, if opaque { 255 } else { a }]);
            }
        }
        DecodedImage {
            width: self.width as u32,
            height: self.height as u32,
            rgba,
        }
    }
}

/// `sub_418F60` (91:1B, both bitmaps format 1): the destination is cleared,
/// then every source pixel is added (saturating) into the destination at
/// a position that shrinks towards the centre: `x_factor` / `y_factor` are
/// the 16.16 shrink amounts, `attenuation` 0..=256 dims the result. The
/// MMX weight computation is kept as is, including its `packssdw`
/// saturation, which sends any pixel with a nonzero horizontal fraction
/// wholly to the right-hand pair of neighbours.
pub(crate) fn concentrate(
    destination: &mut TargetBitmap,
    source: &TargetBitmap,
    attenuation: u32,
    x_factor: u32,
    y_factor: u32,
) {
    let mut table = [[0u16; 4]; 128];
    let mut level = 256u32.wrapping_sub(attenuation).wrapping_shl(7);
    let step = attenuation.wrapping_sub(256);
    for entry in table.iter_mut() {
        let t = (level >> 8) as u16;
        level = level.wrapping_add(step);
        *entry = [t, t, t, 0];
    }
    destination.pixels.fill(0);
    let row_x = (source.width as u32).wrapping_sub(1).wrapping_mul(x_factor) >> 1;
    let mut row_y = (source.height as u32).wrapping_sub(1).wrapping_mul(y_factor) >> 1;
    let dx = 0x10000u32.wrapping_sub(x_factor);
    // cdq: the sign of dx is added to the y lane on every step.
    let dx_carry = if (dx as i32) < 0 { u32::MAX } else { 0 };
    let dy = 0x10000u32.wrapping_sub(y_factor);
    let stride = destination.stride as u32;
    let sat16 = |value: u32| -> u16 {
        let value = value as i32;
        value.clamp(i16::MIN as i32, i16::MAX as i32) as i16 as u16
    };
    for sy in 0..source.height {
        let (mut x, mut y) = (row_x, row_y);
        row_y = row_y.wrapping_add(dy);
        for sx in 0..source.width {
            // psrad 16, packssdw, pmaddwd with [4, stride].
            let ix = sat16(((x as i32) >> 16) as u32) as i16 as i32;
            let iy = sat16(((y as i32) >> 16) as u32) as i16 as i32;
            let base = (ix * 4 + iy * stride as u16 as i16 as i32) as isize;
            let fx = ((x & 0xFE00) >> 9) as u16;
            let fy = ((y & 0xFE00) >> 9) as u16;
            x = x.wrapping_add(dx);
            y = y.wrapping_add(dx_carry);
            let a = [0x80u16.wrapping_sub(fx), fx, 0x80u16.wrapping_sub(fx), fx];
            let b = [0x80u16.wrapping_sub(fy), 0x80u16.wrapping_sub(fy), fy, fy];
            let w: [u16; 4] = std::array::from_fn(|i| a[i].wrapping_mul(b[i]) >> 7);
            let s0 = sat16(u32::from(w[0]) | u32::from(w[1]) << 16);
            let s1 = sat16(u32::from(w[2]) | u32::from(w[3]) << 16);
            let word0 = 0x8080u16.wrapping_sub(s0);
            let word1 = 0x8080u16.wrapping_sub(s1);
            let offset = sy * source.stride + sx * 4;
            let pixel: [u8; 4] = source.pixels[offset..offset + 4].try_into().unwrap();
            // The row step (`add ebx, stride`) sits inside the bottom-left
            // branch: when that neighbour is skipped, bottom-right lands in
            // the top row.
            let bottom = if (word1 as u8) < 0x80 { base + stride as isize } else { base };
            let targets = [
                (word0 as u8, base),
                ((word0 >> 8) as u8, base + 4),
                (word1 as u8, bottom),
                ((word1 >> 8) as u8, bottom + 4),
            ];
            for (index, at) in targets {
                if index >= 0x80 || at < 0 {
                    continue;
                }
                let at = at as usize;
                let Some(target) = destination.pixels.get_mut(at..at + 4) else {
                    continue;
                };
                let weights = table[index as usize];
                for lane in 0..4 {
                    let add = (weights[lane].wrapping_mul(u16::from(pixel[lane])) >> 7).min(255) as u8;
                    target[lane] = target[lane].saturating_add(add);
                }
            }
        }
    }
}

fn read_pixel(bitmap: &TargetBitmap, offset: usize) -> [u8; 4] {
    bitmap
        .pixels
        .get(offset..offset + 4)
        .map_or([0; 4], |bytes| bytes.try_into().unwrap())
}

fn write_pixel(bitmap: &mut TargetBitmap, offset: usize, value: u32) {
    if let Some(target) = bitmap.pixels.get_mut(offset..offset + 4) {
        target.copy_from_slice(&value.to_le_bytes());
    }
}

/// x87 `fild`, `fdiv`, `fmul`, truncating `fistp` (control word set to
/// round toward zero).
fn x87_scaled(numerator: i32, denominator: u32, factor: f64) -> i64 {
    (f64::from(numerator) / f64::from(denominator) * factor) as i64
}

/// `sub_495680` (91:1C without interpolation): nearest sampling of the
/// source scaled by `x_scale` / `y_scale` (16.16) and centred in the
/// destination.
pub(crate) fn scale_nearest(
    destination: &mut TargetBitmap,
    source: &TargetBitmap,
    x_scale: u32,
    y_scale: u32,
) {
    let source_width = (source.width as u32) << 16;
    let source_height = (source.height as u32) << 16;
    let scaled_width = x_scale.wrapping_mul(source.width as u32).wrapping_add(0x8000) >> 16;
    let scaled_height = (source.height as u32).wrapping_mul(y_scale).wrapping_add(0x8000) >> 16;
    if scaled_width == 0 || scaled_height == 0 {
        return;
    }
    let left = (destination.width as i32 >> 1)
        - (x_scale.wrapping_mul(source.width as u32).wrapping_add(0x8000) >> 17) as i32;
    let top = (destination.height as i32 >> 1)
        - ((source.height as u32).wrapping_mul(y_scale).wrapping_add(0x8000) >> 17) as i32;
    let right = (left + scaled_width as i32).min(destination.width as i32);
    let bottom = (top + scaled_height as i32).min(destination.height as i32);
    let first_x = left.max(0);
    let first_y = top.max(0);
    let step_x = source_width / scaled_width;
    let step_y = source_height / scaled_height;
    let start_y = x87_scaled(first_y - top, scaled_height, f64::from(source_height as i32));
    let start_x = x87_scaled(first_x - left, scaled_width, f64::from(source_width as i32)) as u32;
    let mut source_row = source.stride.wrapping_mul((start_y as u32 >> 16) as usize);
    let mut fraction_y = start_y as u32 & 0xFFFF;
    let destination_stride = 4 * (destination.stride >> 2);
    let mut destination_row = first_y as usize * destination.stride;
    for _ in first_y..bottom {
        let mut position = start_x;
        for x in first_x..right {
            let pixel = read_pixel(source, source_row + 4 * (position >> 16) as usize);
            position = position.wrapping_add(step_x);
            write_pixel(destination, destination_row + 4 * x as usize, u32::from_le_bytes(pixel));
        }
        fraction_y = fraction_y.wrapping_add(step_y);
        destination_row += destination_stride;
        source_row += 4 * ((source.stride as u32).wrapping_mul(fraction_y >> 16) >> 2) as usize;
        fraction_y &= 0xFFFF;
    }
}

/// The weight table of `sub_495640`: entry k is k in every lane.
fn area_weight(k: u32) -> u16 {
    k.min(256) as u16
}

/// `sub_495E10`: bilinear sample at 16.16 (x, y); neighbours past the
/// last row or column read the zero pixel `dword_566AE0`.
fn sample_bilinear(source: &TargetBitmap, x: u32, y: u32) -> u32 {
    let fx = (x >> 8) & 0xFF;
    let fy = (y >> 8) & 0xFF;
    let (ix, iy) = ((x >> 16) as usize, (y >> 16) as usize);
    let at = iy * source.stride + 4 * ix;
    let p00 = read_pixel(source, at);
    let mut p01 = read_pixel(source, at + 4);
    let mut p10 = read_pixel(source, at + source.stride);
    let mut p11 = read_pixel(source, at + source.stride + 4);
    if ix >= source.width.wrapping_sub(1) {
        p01 = [0; 4];
        p11 = [0; 4];
    }
    if iy >= source.height.wrapping_sub(1) {
        p10 = [0; 4];
        p11 = [0; 4];
    }
    let (gx, gy) = (256 - fx, 256 - fy);
    let channel = |c: usize| {
        gy.wrapping_mul(gx * u32::from(p00[c]) + fx * u32::from(p01[c]))
            .wrapping_add(fy.wrapping_mul(gx * u32::from(p10[c]) + fx * u32::from(p11[c])))
    };
    let (b, g, r, a) = (channel(0), channel(1), channel(2), channel(3));
    (((b >> 8) | (g & 0xFFFF_00FF)) >> 8) | (r & 0xFFFF_0000) | ((a << 8) & 0xFF00_0000)
}

/// `sub_495B60`: average over the footprint rectangle, colour only from
/// pixels with alpha (or any pixel of a format-1 source).
fn sample_box(source: &TargetBitmap, format: u32, x: u32, y: u32, width: u32, height: u32) -> u32 {
    let left = (x >> 16) as i32;
    let top = (y >> 16) as i32;
    let right = (left + ((width.wrapping_add(255)) >> 8) as i32).min(source.width as i32);
    let bottom = (top + ((height.wrapping_add(255)) >> 8) as i32).min(source.height as i32);
    let (left, top) = (left.max(0), top.max(0));
    let (mut b, mut g, mut r, mut a, mut counted, mut total) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    for row in top..bottom {
        if (left as u32) < right as u32 {
            total = total.wrapping_add((right - left) as u32);
        }
        for column in left..right {
            let pixel = read_pixel(source, row as usize * source.stride + 4 * column as usize);
            if pixel[3] != 0 || format == 1 {
                b += u32::from(pixel[0]);
                g += u32::from(pixel[1]);
                r += u32::from(pixel[2]);
                a += u32::from(pixel[3]);
                counted += 1;
            }
        }
    }
    let counted = counted.max(1);
    let alpha = if total == 0 { 0 } else { a / total };
    ((((g / counted) | (((r / counted) | (alpha << 8)) << 8)) << 8)) | (b / counted)
}

/// `sub_495CA0`: area-weighted footprint in 24.8 coordinates with the
/// saturating 16-bit SSE accumulation.
fn sample_area(source: &TargetBitmap, x: u32, y: u32, width: u32, height: u32) -> u32 {
    let mut left = (x >> 8) as i32;
    let mut top = (y >> 8) as i32;
    let mut right = left.wrapping_add(width as i32);
    let mut bottom = top.wrapping_add(height as i32);
    left = left.max(0);
    top = top.max(0);
    right = right.min((source.width as i32) << 8);
    bottom = bottom.min((source.height as i32) << 8);
    let area = (right - left).wrapping_mul(bottom - top) >> 8;
    let mut sum = [0u16; 4];
    let mut row = (top >> 8) as usize * source.stride;
    while top < bottom {
        let next = ((top + 256) as u32 & 0xFFFF_FF00) as i32;
        let next = next.min(bottom);
        let span_y = next - top;
        let mut column = left;
        while column < right {
            let end = (((column + 256) as u32 & 0xFFFF_FF00) as i32).min(right);
            let weight = if area == 0 { 0 } else { ((span_y * (end - column)) << 8) / area };
            let pixel = read_pixel(source, row + ((column as u32 >> 6) & 0x3FF_FFFC) as usize);
            let k = area_weight((weight >> 8) as u32);
            for lane in 0..4 {
                sum[lane] = sum[lane].saturating_add(u16::from(pixel[lane]).wrapping_mul(k));
            }
            column = end;
        }
        row += source.stride;
        top = next;
    }
    u32::from_le_bytes(sum.map(|value| (value >> 8).min(255) as u8))
}

/// `sub_495880` (91:1C with interpolation): the sampler is chosen by the
/// scales — bilinear when both enlarge, a box average when either shrinks
/// below one half, the area sampler otherwise.
pub(crate) fn scale_filtered(
    destination: &mut TargetBitmap,
    source: &TargetBitmap,
    format: u32,
    x_scale: u32,
    y_scale: u32,
) {
    let scaled_width = x_scale.wrapping_mul(source.width as u32).wrapping_add(0x8000) >> 16;
    let scaled_height = y_scale.wrapping_mul(source.height as u32).wrapping_add(0x8000) >> 16;
    if scaled_width == 0 || scaled_height == 0 || x_scale >> 8 == 0 || y_scale >> 8 == 0 {
        return;
    }
    let mut footprint = [0x10000 / (x_scale >> 8), 0x10000 / (y_scale >> 8)];
    let left = (destination.width as i32 >> 1) - (scaled_width >> 1) as i32;
    let top = (destination.height as i32 >> 1) - (scaled_height >> 1) as i32;
    let first_x = left.max(0);
    let first_y = top.max(0);
    let right = (left + scaled_width as i32).min(destination.width as i32);
    let bottom = (top + scaled_height as i32).min(destination.height as i32);
    let mut bias = [footprint[0] << 7, footprint[1] << 7];
    let mut shift = 8;
    #[derive(Clone, Copy)]
    enum Sampler {
        Bilinear,
        Box,
        Area,
    }
    let mut sampler = Sampler::Bilinear;
    if (x_scale >= 0x10000 || x_scale <= 0x7FFF) && y_scale.wrapping_sub(0x8000) > 0x7FFF {
        if x_scale < 0x8000 || y_scale < 0x8000 {
            sampler = Sampler::Box;
        } else {
            footprint = [512, 512];
            shift = 9;
            bias = [0, 0];
        }
    } else {
        sampler = Sampler::Area;
    }
    let span_x = ((source.width as u32).wrapping_sub(footprint[0] >> shift)) << 16;
    let span_y = ((source.height as u32).wrapping_sub(footprint[1] >> shift)) << 16;
    let step_x = span_x / scaled_width;
    let step_y = span_y / scaled_height;
    let mut y = bias[1].wrapping_add(x87_scaled(first_y - top, scaled_height, f64::from(span_y as i32)) as u32);
    let row_x = bias[0].wrapping_add(x87_scaled(first_x - left, scaled_width, f64::from(span_x as i32)) as u32);
    let mut destination_row = first_y as usize * destination.stride;
    let destination_stride = 4 * (destination.stride >> 2);
    for _ in first_y..bottom {
        let mut x = row_x;
        for column in first_x..right {
            let value = match sampler {
                Sampler::Bilinear => sample_bilinear(source, x, y),
                Sampler::Box => sample_box(source, format, x, y, footprint[0], footprint[1]),
                Sampler::Area => sample_area(source, x, y, footprint[0], footprint[1]),
            };
            x = x.wrapping_add(step_x);
            write_pixel(destination, destination_row + 4 * column as usize, value);
        }
        y = y.wrapping_add(step_y);
        destination_row += destination_stride;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(bytes: &[u8], count: usize) -> Vec<u32> {
        bytes[..count * 4]
            .chunks_exact(4)
            .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
            .collect()
    }

    /// Vectors from `src/tools/emu_kernels.py` (the target under Unicorn).
    #[test]
    #[ignore]
    fn kernels_match_the_target_vectors() {
        let directory = std::env::var("ETHORNELL_KERNEL_VECTORS")
            .expect("ETHORNELL_KERNEL_VECTORS names the vector directory");
        let mut checked = 0;
        let mut entries = std::fs::read_dir(&directory)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            let data = std::fs::read(&path).unwrap();
            let header = words(&data, 8);
            let [kind, sw, sh, dw, dh, p0, p1, p2] = header[..] else {
                unreachable!()
            };
            let (sw, sh, dw, dh) = (sw as usize, sh as usize, dw as usize, dh as usize);
            let mut at = 32;
            let mut take = |n: usize| {
                let slice = data[at..at + n].to_vec();
                at += n;
                slice
            };
            let source = TargetBitmap { pixels: take(sw * sh * 4), stride: sw * 4, width: sw, height: sh };
            let before = take(dw * dh * 4);
            let expected = take(dw * dh * 4);
            let mut destination = TargetBitmap { pixels: before, stride: dw * 4, width: dw, height: dh };
            match kind {
                1 => concentrate(&mut destination, &source, p0, p1, p2),
                2 => scale_nearest(&mut destination, &source, p0, p1),
                3 => scale_filtered(&mut destination, &source, p2, p0, p1),
                _ => panic!("unknown kernel {kind}"),
            }
            assert_eq!(destination.pixels, expected, "{}", path.display());
            checked += 1;
        }
        assert!(checked > 0);
    }
}
