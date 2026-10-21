//! CDspObjLandscape copied assets, composed columns and priority-sliced drawing.
use super::*;
#[derive(Debug, Clone)]
struct Part {
    image: DecodedImage,
    format: i32,
}
#[derive(Debug, Clone)]
struct Column {
    image: DecodedImage,
    format: i32,
    offset: i32,
    occluding: i32,
    boundary: i32,
    extra: i32,
    hit: Option<Vec<u8>>,
}
#[derive(Debug, Clone)]
struct CellGeometry {
    column: usize,
    x: i32,
    y: i32,
    height: u32,
    depth: u32,
}
#[derive(Debug, Clone, Default)]
pub(crate) struct LandscapeRaster {
    parts: Vec<Part>,
    columns: Vec<Column>,
    guides: Vec<Part>,
    cells: Vec<Option<CellGeometry>>,
    layers: BTreeMap<u32, i32>,
}
fn crop_part(source: &DecodedImage, format: i32, r: [i32; 5]) -> Option<Part> {
    let (x, y, w, h) = (r[0], r[1], r[2], r[3]);
    if w <= 0
        || h <= 0
        || i64::from(w) * i64::from(h) > 16 * 1024 * 1024
        || i64::from(x) >= i64::from(source.width)
        || i64::from(y) >= i64::from(source.height)
        || i64::from(x) + i64::from(w) <= 0
        || i64::from(y) + i64::from(h) <= 0
    {
        return None;
    }
    let mut image = crate::native_map_filter::blank(w as u32, h as u32);
    crate::bitmap_blend::blit_mode(&mut image, format, source, format, -x, -y, 0x80, 0);
    Some(Part { image, format })
}
fn compose_columns(
    parts: &[Part],
    records: &[Vec<i32>],
    flags: &[[i32; 5]],
    cell_width: i32,
    step: i32,
    spacing: i32,
) -> Option<Vec<Column>> {
    let mut result = Vec::new();
    for words in records {
        let count = words[0] as u32;
        let boundary = words[33] as u32;
        if count == 0 || count > 32 || boundary == 0 || boundary > 32 {
            return None;
        }
        let mut width = 0;
        let mut height = 0;
        for i in 0..count as usize {
            let id = words[1 + i];
            if id == -1 {
                continue;
            }
            let part = parts.get(id as usize)?;
            width = width.max(part.image.width);
            height = height.max((part.image.height as i32).wrapping_add(i as i32 * step) as u32);
        }
        if width == 0 || height == 0 || u64::from(width) * u64::from(height) > 16 * 1024 * 1024 {
            return None;
        }
        let format = parts.first()?.format;
        let mut image = crate::native_map_filter::blank(width, height);
        let mut hit = None;
        let mut extra = 0;
        let mut occluding = 0;
        let mut counting = true;
        for i in 0..count as usize {
            let id = words[1 + i];
            if id == -1 {
                counting = false;
                continue;
            }
            let part = &parts[id as usize];
            crate::bitmap_blend::blit_mode(
                &mut image,
                format,
                &part.image,
                part.format,
                ((width - part.image.width) / 2) as i32,
                height as i32 - i as i32 * step - part.image.height as i32,
                1,
                0,
            );
            if counting {
                if flags[id as usize][4] != 0 {
                    counting = false;
                } else if i + 1 == boundary as usize {
                    occluding += spacing;
                    counting = false;
                } else {
                    occluding += 1;
                }
            }
            if i + 1 == boundary as usize && i + 1 < count as usize {
                hit = Some(
                    image
                        .rgba
                        .as_chunks::<4>()
                        .0
                        .iter()
                        .map(|p| if format == 2 { p[3] } else { 0 })
                        .collect(),
                );
                extra = (height as i32 / step - boundary as i32 - 2).max(1);
            }
        }
        let offset = cell_width.wrapping_sub(width as i32) >> 1;
        result.push(Column {
            image,
            format,
            offset,
            occluding: if offset > 0 { 0 } else { occluding * step },
            boundary: boundary as i32,
            extra,
            hit,
        });
    }
    Some(result)
}
impl LandscapeRaster {
    pub(super) fn layer_ids(&self) -> Vec<i32> {
        self.layers.values().copied().collect()
    }
}
impl RuntimeTraceApi {
    pub(super) fn configure_native_landscape_parts(
        &mut self,
        object: i32,
        bitmap: i32,
        count: i32,
        words: &[i32],
        spacing: i32,
        column_count: i32,
        column_words: &[i32],
    ) -> bool {
        let Some(state) = self.graph91_landscapes.get(&object) else {
            return false;
        };
        if count <= 0
            || column_count <= 0
            || words.len() != count as usize * 5
            || column_words.len() != column_count as usize * 34
        {
            return false;
        }
        let Some(source) = self.graph_bitmap_image(bitmap) else {
            return false;
        };
        let format = self.bitmap_formats.get(&bitmap).copied().unwrap_or(2);
        let flags = words.as_chunks::<5>().0.to_vec();
        let mut parts = Vec::new();
        for &record in &flags {
            let Some(part) = crop_part(&source, format, record) else {
                return false;
            };
            parts.push(part);
        }
        let records = column_words
            .chunks_exact(34)
            .map(|c| c.to_vec())
            .collect::<Vec<_>>();
        if state.column_step == 0 {
            return false;
        }
        let Some(columns) = compose_columns(
            &parts,
            &records,
            &flags,
            state.cell_width,
            state.column_step,
            spacing,
        ) else {
            return false;
        };
        let state = self.graph91_landscapes.get_mut(&object).unwrap();
        state.source_bitmap = bitmap;
        state.part_spacing = spacing;
        state.parts = flags;
        state.columns = records;
        // Native success releases the old map together with the old columns.
        state.cells.clear();
        state.map_width = 0;
        state.map_height = 0;
        let raster = self.native_landscapes.entry(object).or_default();
        raster.parts = parts;
        raster.columns = columns;
        raster.cells.clear();
        self.refresh_native_landscape(object);
        true
    }
    pub(super) fn configure_native_landscape_guides(
        &mut self,
        object: i32,
        bitmap: i32,
        count: i32,
        words: &[i32],
    ) -> bool {
        if count <= 0
            || words.len() != count as usize * 5
            || !self.graph91_landscapes.contains_key(&object)
        {
            return false;
        }
        let Some(source) = self.graph_bitmap_image(bitmap) else {
            return false;
        };
        let format = self.bitmap_formats.get(&bitmap).copied().unwrap_or(2);
        let mut guides = Vec::new();
        for &record in words.as_chunks::<5>().0 {
            let Some(part) = crop_part(&source, format, record) else {
                return false;
            };
            guides.push(part);
        }
        self.native_landscapes.entry(object).or_default().guides = guides;
        let state = self.graph91_landscapes.get_mut(&object).unwrap();
        state.guide_bitmap = bitmap;
        state.guides = words.as_chunks::<5>().0.to_vec();
        // 423780 replaces assets but leaves stored cell guide indices intact.
        self.refresh_native_landscape(object);
        true
    }
    pub(super) fn copy_native_landscape_part(
        &mut self,
        object: i32,
        destination: i32,
        source: i32,
    ) -> bool {
        let Some(raster) = self.native_landscapes.get_mut(&object) else {
            return false;
        };
        let Some(part) = raster.parts.get(source as usize).cloned() else {
            return false;
        };
        let Some(dest) = raster.parts.get_mut(destination as usize) else {
            return false;
        };
        if (part.image.width, part.image.height, part.format)
            != (dest.image.width, dest.image.height, dest.format)
        {
            return false;
        }
        *dest = part;
        let state = &self.graph91_landscapes[&object];
        let Some(columns) = compose_columns(
            &raster.parts,
            &state.columns,
            &state.parts,
            state.cell_width,
            state.column_step,
            state.part_spacing,
        ) else {
            return false;
        };
        // 423AB0 only rebuilds color pixels. Boundary alpha snapshots and
        // column geometry remain the configure-time copies.
        for (old, new) in raster.columns.iter_mut().zip(columns) {
            old.image = new.image;
        }
        self.refresh_native_landscape(object);
        true
    }
    pub(super) fn copy_native_landscape_cell(
        &mut self,
        destination: i32,
        object: i32,
        row: i32,
        column: i32,
    ) -> bool {
        if destination as u32 >= 0x4000 {
            return false;
        }
        let Some(state) = self.graph91_landscapes.get(&object) else {
            return false;
        };
        let Some(index) = state.cell_index(row, column) else {
            return false;
        };
        let id = state.cells[index].column_index;
        let Some(words) = state.columns.get(id as usize) else {
            return false;
        };
        let boundary = words[33] as usize;
        let Some(&part_id) = words.get(boundary) else {
            return false;
        };
        let Some(part) = self
            .native_landscapes
            .get(&object)
            .and_then(|r| r.parts.get(part_id as usize))
            .cloned()
        else {
            return false;
        };
        self.store_runtime_bitmap(destination, part.image, part.format);
        true
    }
    pub(super) fn refresh_native_landscape(&mut self, object: i32) {
        let Some(mut raster) = self.native_landscapes.remove(&object) else {
            return;
        };
        for layer in raster.layers.values() {
            self.graph_layers.remove(layer);
            self.graph_object_properties.remove(layer);
        }
        let old_layers = std::mem::take(&mut raster.layers);
        let Some(state) = self.graph91_landscapes.get(&object).cloned() else {
            return;
        };
        let width = state.map_width;
        let height = state.map_height;
        let base = self.graph90_native_sort_key(object).unwrap_or(0) & 0xffffe01f;
        let occluding = |row: i32, col: i32| -> i32 {
            state
                .cell_index(row, col)
                .and_then(|i| raster.columns.get(state.cells[i].column_index as usize))
                .map_or(0, |c| c.occluding)
        };
        let mut cells = Vec::with_capacity(state.cells.len());
        for row in 0..height {
            for col in 0..width {
                let index = (row * width + col) as usize;
                let id = state.cells[index].column_index;
                let Some(column) = raster.columns.get(id as usize) else {
                    cells.push(None);
                    continue;
                };
                let next = col - if row & 1 == 0 { 1 } else { 0 };
                let a = occluding(row + 1, next) - state.row_step;
                let b = occluding(row + 1, next + 1) - state.row_step;
                let mut crop = if a > 0 && b > 0 { a.min(b) } else { 0 };
                let c = occluding(row + 2, col) - 2 * state.row_step;
                if c > 0 && crop > c {
                    crop = c;
                }
                let visible = (column.image.height as i32 - crop).max(0) as u32;
                let priority = row.wrapping_mul(state.priority_row_factor).wrapping_add(
                    state
                        .priority_frame_factor
                        .wrapping_mul(column.extra + column.boundary - 1),
                );
                let depth = if visible == 0 {
                    0
                } else {
                    base.wrapping_add((row as u32).wrapping_add((priority as u32) << 11) << 5)
                };
                cells.push(Some(CellGeometry {
                    column: id as usize,
                    x: col.wrapping_mul(state.cell_width)
                        + if row & 1 != 0 {
                            state.cell_width / 2
                        } else {
                            0
                        }
                        + column.offset,
                    y: state.baseline + row.wrapping_mul(state.row_step)
                        - column.image.height as i32,
                    height: visible,
                    depth,
                }));
            }
        }
        raster.cells = cells;
        let depths = raster
            .cells
            .iter()
            .flatten()
            .filter_map(|c| (c.depth != 0).then_some(c.depth))
            .collect::<BTreeSet<_>>();
        let (w, h) = snapshot::runtime_frame_size(self);
        for depth in depths {
            let key = format!("runtime:landscape:{object}:{depth}");
            self.store_graph_image(key.clone(), crate::native_map_filter::blank(w, h));
            let layer = old_layers.get(&depth).copied().unwrap_or_else(|| {
                let layer = self.native_landscape_next_layer;
                self.native_landscape_next_layer = self.native_landscape_next_layer.wrapping_sub(1);
                layer
            });
            self.install_native_map_filter_layer(layer, key, w, h);
            if let Some(layer) = self.graph_layers.get_mut(&layer) {
                layer.owner_object = Some(object);
                layer.hit_id = object;
                layer.z = (depth >> 16) as i32;
                layer.x = 0.;
                layer.y = 0.;
            }
            raster.layers.insert(depth, layer);
        }
        if let Some(state) = self.graph91_landscapes.get_mut(&object) {
            for (cell, geometry) in state.cells.iter_mut().zip(&raster.cells) {
                cell.derived_value = geometry.as_ref().map_or(0, |g| (g.depth >> 16) as u16);
            }
        }
        self.native_landscapes.insert(object, raster);
    }
    pub(super) fn render_native_landscape_stage(
        &self,
        object: i32,
        depth: u32,
        source: &DecodedImage,
    ) -> Option<DecodedImage> {
        let state = self.graph91_landscapes.get(&object)?;
        let raster = self.native_landscapes.get(&object)?;
        let properties = self.graph_object_properties.get(&object)?;
        let alpha = properties.transparency_parameter();
        let blend = properties.blend_mode;
        let df = self
            .bitmap_formats
            .get(&NATIVE_SCREEN_BITMAP)
            .copied()
            .unwrap_or(1);
        let mut result = source.clone();
        let global = self.graph_object_global_display_offset(object);
        let x = properties.native.position_x + global.0 as i32;
        let y = properties.native.position_y + global.1 as i32;
        for (cell, geometry) in state.cells.iter().zip(&raster.cells) {
            let Some(g) = geometry.as_ref().filter(|g| g.depth == depth) else {
                continue;
            };
            let column = &raster.columns[g.column];
            let mut image = DecodedImage {
                width: column.image.width,
                height: g.height,
                rgba: column.image.rgba[..(column.image.width * g.height * 4) as usize].to_vec(),
            };
            if cell.silhouette_type != 0 {
                let mut silhouette = image.clone();
                for p in silhouette.rgba.as_chunks_mut::<4>().0 {
                    p[0] = (cell.silhouette_value >> 16) as u8;
                    p[1] = (cell.silhouette_value >> 8) as u8;
                    p[2] = cell.silhouette_value as u8;
                }
                if column.format == 2 {
                    crate::bitmap_blend::blit_mode(
                        &mut image,
                        column.format,
                        &silhouette,
                        column.format,
                        0,
                        0,
                        if cell.silhouette_type == 2 {
                            0x21
                        } else {
                            0x20
                        },
                        256 - cell.silhouette_level,
                    );
                }
            }
            crate::bitmap_blend::blit_mode(
                &mut result,
                df,
                &image,
                column.format,
                x + g.x,
                y + g.y,
                blend,
                alpha,
            );
            for &(id, transparency) in cell.guides.iter().flatten() {
                if let Some(guide) = raster.guides.get(id as usize) {
                    let gx =
                        x + g.x + ((column.image.width as i32 - guide.image.width as i32) >> 1);
                    let gy = y + g.y + column.image.height as i32
                        - column.boundary * state.column_step
                        - guide.image.height as i32;
                    let ga = 256 - ((256 - alpha).wrapping_mul(256 - transparency) >> 8);
                    crate::bitmap_blend::blit_mode(
                        &mut result,
                        df,
                        &guide.image,
                        guide.format,
                        gx,
                        gy,
                        blend,
                        ga,
                    );
                }
            }
        }
        if df == 1 {
            for p in result.rgba.as_chunks_mut::<4>().0 {
                p[3] = 255;
            }
        }
        Some(result)
    }
    pub(super) fn native_landscape_hit(&self, object: i32, mode: i32) -> Option<[i32; 2]> {
        let state = self.graph91_landscapes.get(&object)?;
        let raster = self.native_landscapes.get(&object)?;
        let properties = self.graph_object_properties.get(&object)?;
        let (mx, my) = self.mouse_pos?;
        let x = mx as i32 - properties.native.position_x;
        let y = my as i32 - properties.native.position_y;
        if state.cell_width == 0 || state.row_step == 0 {
            return None;
        }
        let even = x / state.cell_width;
        let odd =
            even - i32::from((x as u32 % state.cell_width as u32) < (state.cell_width as u32 / 2));
        let begin = (i32::from(y >= state.baseline) + (y - state.baseline) / state.row_step).max(0);
        let end = (i32::from(y >= state.baseline)
            + (y - state.baseline) / state.row_step
            + 32 * state.column_step / state.row_step)
            .min(state.map_height - 1);
        for row in (begin..=end).rev() {
            let col = if row & 1 != 0 { odd } else { even };
            let Some(index) = state.cell_index(row, col) else {
                continue;
            };
            let Some(g) = raster.cells.get(index)?.as_ref() else {
                continue;
            };
            let (px, py) = (x - g.x, y - g.y);
            let column = &raster.columns[g.column];
            if px < 0 || py < 0 || px >= column.image.width as i32 || py >= g.height as i32 {
                continue;
            }
            let index = py as usize * column.image.width as usize + px as usize;
            let alpha = if mode == 1 {
                column
                    .hit
                    .as_ref()
                    .and_then(|m| m.get(index))
                    .copied()
                    .unwrap_or_else(|| {
                        if column.format == 2 {
                            column.image.rgba[index * 4 + 3]
                        } else {
                            0
                        }
                    })
            } else if column.format == 2 {
                column.image.rgba[index * 4 + 3]
            } else {
                0
            };
            if alpha != 0 {
                return Some([col, row]);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethornell_vm::{GraphApi, NativeCallFrame, NativeOpcode, Value};
    #[test]
    fn landscape_copies_assets_draws_depth_slices_and_hits_actual_alpha() {
        let manager = ResourceManager::open_game(env!("CARGO_MANIFEST_DIR")).unwrap();
        let mut api = RuntimeTraceApi::new(manager);
        api.screen_width = 4;
        api.screen_height = 4;
        let call = |api: &mut RuntimeTraceApi, id, args: &[i32]| {
            GraphApi::call_graph(
                api,
                &mut NativeCallFrame::new(
                    NativeOpcode { group: 0x91, id },
                    args.iter().copied().map(Value::Int).collect(),
                ),
            )
            .unwrap()
        };
        api.store_runtime_bitmap(
            100,
            DecodedImage {
                width: 4,
                height: 2,
                rgba: [[200, 0, 0, 255].repeat(2), [0, 0, 100, 255].repeat(2)]
                    .concat()
                    .repeat(2),
            },
            2,
        );
        let object = call(&mut api, 0x70, &[2, 1, 1, 2, 1, 0]).as_i32();
        let mut columns = vec![0; 68];
        columns[0] = 1;
        columns[1] = 0;
        columns[33] = 1;
        columns[34] = 1;
        columns[35] = 1;
        columns[67] = 1;
        assert!(GraphApi::configure_graph91_landscape_parts(
            &mut api,
            object,
            100,
            2,
            &[0, 0, 2, 2, 0, 2, 0, 2, 2, 0],
            1,
            2,
            &columns
        ));
        api.graph_resources.remove(&100);
        api.graph_images.remove("runtime:bitmap:100");
        assert!(GraphApi::configure_graph91_landscape_map(
            &mut api,
            object,
            2,
            1,
            &[0, 1]
        ));
        call(&mut api, 0x75, &[object, 0, 0, 1, 0, 5]);
        api.refresh_native_filters();
        let frame = snapshot::compose_runtime_frame(&api, false);
        assert_eq!(
            &frame.rgba[..16],
            &[
                200, 0, 0, 255, 200, 0, 0, 255, 0, 0, 100, 255, 0, 0, 100, 255
            ]
        );
        api.mouse_pos = Some((3., 1.));
        assert_eq!(api.native_landscape_hit(object, 0), Some([1, 0]));
        assert_eq!(api.graph91_landscape_cell_value(object, 0, 0), Some(5));
        call(&mut api, 0x7c, &[object, 0, 1]);
        api.refresh_native_filters();
        assert_eq!(
            &snapshot::compose_runtime_frame(&api, false).rgba[..4],
            &[0, 0, 100, 255]
        );
        assert_eq!(call(&mut api, 0x7f, &[200, object, 0, 0]), Value::Int(1));
        assert_eq!(
            &api.graph_bitmap_image(200).unwrap().rgba[..4],
            &[0, 0, 100, 255]
        );
        api.store_runtime_bitmap(
            101,
            DecodedImage {
                width: 1,
                height: 1,
                rgba: vec![255, 255, 255, 255],
            },
            2,
        );
        assert!(GraphApi::configure_graph91_landscape_guides(
            &mut api,
            object,
            101,
            1,
            &[0, 0, 1, 1, 0]
        ));
        assert!(GraphApi::set_graph91_landscape_cell_guides(
            &mut api,
            object,
            &[0, 0],
            0,
            0,
            0
        ));
        api.graph_resources.remove(&101);
        api.refresh_native_filters();
        assert_eq!(
            &snapshot::compose_runtime_frame(&api, false).rgba[..4],
            &[255, 255, 255, 255]
        );
        call(&mut api, 0x7d, &[object, 0, 0, 1]);
        api.refresh_native_filters();
        assert_eq!(
            &snapshot::compose_runtime_frame(&api, false).rgba[..4],
            &[0, 0, 100, 255]
        );
    }
}
