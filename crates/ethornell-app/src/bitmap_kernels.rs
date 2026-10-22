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
                _ => panic!("unknown kernel {kind}"),
            }
            assert_eq!(destination.pixels, expected, "{}", path.display());
            checked += 1;
        }
        assert!(checked > 0);
    }
}
