//! The 0xD0 native table: the strategy-game ("logic") extension of BGI 1.622.
//! See `src/adr/0002-d0-logic-extension.md` for the map of the three classes
//! and `src/engine/logic_grid_field.c` for the cleaned target code.

mod grid;
mod spline;

use super::{Value, Vm, VmResult};
use grid::{Cell, GridFieldManager, Tile};

/// The target's internal statuses; `status_to_script` (sub_497120) turns them
/// into what the script sees.
pub(crate) mod status {
    pub const NO_HANDLE: u32 = 0x8000_0000;
    pub const BAD_ARGUMENT: u32 = 0x8000_0001;
    pub const NO_MAP: u32 = 0x8000_0002;
    pub const NO_UNIT: u32 = 0x8000_0003;
    pub const OUT_OF_MAP: u32 = 0x8000_0004;
    pub const NO_TABLE: u32 = 0x8000_0005;
    pub const NO_SEARCH: u32 = 0x8000_0006;
    pub const UNREACHABLE: u32 = 0x8000_0007;
    pub const BAD_ORIGIN: u32 = 0x8000_0008;
    pub const BAD_HEIGHT: u32 = 0x8000_0009;
    pub const BAD_RADIUS: u32 = 0x8000_000A;
    pub const BAD_DIRECTION: u32 = 0x8000_000B;
    pub const FAILED: u32 = 0xFFFF_FFFF;
    pub const BROKEN_PATH: u32 = 0xFFFF_FFFE;
    pub const SEARCH_FAILED: u32 = FAILED;
    // DCTELogicalField's own codes, remapped by the manager.
    pub const NO_CELLS: u32 = 0x8000_0003;
    pub const NOT_REACHED: u32 = 0x8000_0004;
    pub const OUT_OF_FIELD: u32 = 0x8000_0005;
    pub const PRUNED: u32 = 0x8000_0006;
}

/// sub_497120.
pub(crate) fn status_to_script(status: u32) -> i32 {
    match status {
        0 => 0,
        0x8000_0000..=0x8000_000B => (status - 0x8000_0000 + 1) as i32,
        0x9000_0002 => 16,
        0x9000_0003 => 17,
        0xA000_0000 => 1,
        0xA000_0001..=0xA000_0003 => (status - 0xA000_0001 + 18) as i32,
        0xA000_0004 => 8,
        0xA000_0005 => 21,
        0xA000_0006 => 22,
        0xFFFF_FFFE => -2,
        _ => -1,
    }
}

/// Largest map the port accepts (16 bytes a tile). The target only rejects 0
/// and dies in `operator new` on absurd sizes.
const MAX_TILES: i64 = 1 << 22;

#[derive(Debug, Default)]
pub(crate) struct LogicState {
    /// dword_5669EC, newest first when searched; ids from dword_5669E4++ (0..).
    grids: Vec<(i32, GridFieldManager)>,
    next_grid: i32,
}

impl LogicState {
    fn grid(&mut self, handle: i32) -> Result<&mut GridFieldManager, u32> {
        self.grids
            .iter_mut()
            .rev()
            .find(|(id, _)| *id == handle)
            .map(|(_, grid)| grid)
            .ok_or(status::NO_HANDLE)
    }
}

/// Fold a manager result into the status the wrapper returns.
fn done(result: Result<u32, u32>) -> u32 {
    result.unwrap_or_else(|status| status)
}

impl Vm {
    /// Runs a ported 0xD0 selector with its arguments in call order. `None`
    /// means the selector is not ported yet.
    pub(super) fn execute_logic(&mut self, id: u8, args: &[Value]) -> VmResult<Option<i32>> {
        let int = |index: usize| args.get(index).map(Value::as_i32).unwrap_or_default();
        let ptr = |index: usize| int(index) as u32;
        let result = match id {
            // (out*, kind, scale): only kind 0 exists; raw 1 / 0.
            0x00 => {
                if int(1) != 0 {
                    return Ok(Some(0));
                }
                let handle = self.logic.next_grid;
                self.logic.next_grid = handle.wrapping_add(1);
                self.logic
                    .grids
                    .push((handle, GridFieldManager::new(int(2))));
                self.logic_write_i32(ptr(0), handle)?;
                return Ok(Some(1));
            }
            // (h): raw 1 found / 0.
            0x01 => {
                let handle = int(0);
                let found = self.logic.grids.iter().rposition(|(id, _)| *id == handle);
                if let Some(index) = found {
                    self.logic.grids.remove(index);
                }
                return Ok(Some(i32::from(found.is_some())));
            }
            0x04 => {
                let (w, h) = (int(1), int(2));
                let cells = i64::from(w) * i64::from(h);
                if let Err(status) = self.logic.grid(int(0)) {
                    status
                } else if w != 0 && h != 0 && !(1..=MAX_TILES).contains(&cells) {
                    status::BAD_ARGUMENT
                } else {
                    let bytes = self.logic_read(ptr(3), cells.max(0) as usize * Tile::SIZE)?;
                    let tiles = bytes
                        .chunks_exact(Tile::SIZE)
                        .map(Tile::from_bytes)
                        .collect();
                    done(
                        self.logic
                            .grid(int(0))
                            .map(|grid| grid.set_map(w, h, tiles)),
                    )
                }
            }
            0x05 => {
                let count = match self.logic.grid(int(1)) {
                    Ok(grid) => grid.map_cells(),
                    Err(status) => return Ok(Some(status_to_script(status))),
                };
                let bytes = self.logic_read(ptr(2), count * 4)?;
                let cost = bytes
                    .chunks_exact(4)
                    .map(|word| i32::from_le_bytes(word.try_into().unwrap()))
                    .collect();
                match self
                    .logic
                    .grid(int(1))
                    .and_then(|grid| grid.add_cost_table(cost))
                {
                    Ok(table) => {
                        self.logic_write_i32(ptr(0), table)?;
                        0
                    }
                    Err(status) => status,
                }
            }
            0x10 => match self.logic.grid(int(1)).and_then(GridFieldManager::add_unit) {
                Ok(unit) => {
                    self.logic_write_i32(ptr(0), unit)?;
                    0
                }
                Err(status) => status,
            },
            0x11 => done(self.logic.grid(int(0)).map(|grid| grid.remove_unit(int(1)))),
            0x12 => done(self.logic.grid(int(0)).map(|grid| grid.drop_search(int(1)))),
            0x14 => done(
                self.logic
                    .grid(int(0))
                    .map(|grid| grid.set_position(int(1), int(2), int(3))),
            ),
            0x15 => done(
                self.logic
                    .grid(int(0))
                    .map(|grid| grid.set_size(int(1), int(2), int(3))),
            ),
            0x16 => {
                match self
                    .logic
                    .grid(int(0))
                    .and_then(|grid| grid.unit_mut(int(1)).map(|_| ()))
                {
                    Ok(()) => {}
                    Err(status) => return Ok(Some(status_to_script(status))),
                }
                let pair = self.logic_read(ptr(2), 8)?;
                let flags = u32::from_le_bytes(pair[0..4].try_into().unwrap());
                let team = u32::from_le_bytes(pair[4..8].try_into().unwrap());
                done(
                    self.logic
                        .grid(int(0))
                        .map(|grid| grid.set_flags(int(1), flags, team)),
                )
            }
            0x17 => done(
                self.logic
                    .grid(int(0))
                    .map(|grid| grid.set_table(int(1), int(2))),
            ),
            0x18 => done(
                self.logic
                    .grid(int(0))
                    .map(|grid| grid.set_dir(int(1), int(2))),
            ),
            0x20 => done(
                self.logic
                    .grid(int(0))
                    .map(|grid| grid.search(int(1), int(2), int(3), (int(4), int(5)))),
            ),
            // (dirs*, count*, h, unit, x, y)
            0x21 => {
                let want_dirs = ptr(0) != 0;
                match self
                    .logic
                    .grid(int(2))
                    .and_then(|grid| grid.path(int(3), int(4), int(5), want_dirs))
                {
                    Ok((steps, dirs)) => {
                        self.logic_write_i32(ptr(1), steps)?;
                        self.logic_write_i32s(ptr(0), &dirs)?;
                        0
                    }
                    Err(status) => status,
                }
            }
            // (cells*, h, unit)
            0x22 => {
                let bytes = match self.logic.grid(int(1)).and_then(|grid| grid.cells(int(2))) {
                    Ok(cells) => cells
                        .iter()
                        .flat_map(|cell| Cell::to_bytes(*cell))
                        .collect::<Vec<_>>(),
                    Err(status) => return Ok(Some(status_to_script(status))),
                };
                self.logic_write(ptr(0), &bytes)?;
                0
            }
            // (xy*, count*, h, unit)
            0x23 => match self
                .logic
                .grid(int(2))
                .and_then(|grid| grid.reachable(int(3)))
            {
                Ok(tiles) => {
                    let flat: Vec<i32> = tiles.iter().flat_map(|&(x, y)| [x, y]).collect();
                    self.logic_write_i32s(ptr(0), &flat)?;
                    self.logic_write_i32(ptr(1), tiles.len() as i32)?;
                    0
                }
                Err(status) => status,
            },
            // (xy*, dist*, count*, h, x, y, range, bonus, arc, units_only, ignore_units)
            0x28 => match self.logic.grid(int(3)).and_then(|grid| {
                grid.targets(
                    int(4),
                    int(5),
                    int(6),
                    int(7),
                    int(8),
                    int(9) != 0,
                    int(10) != 0,
                )
            }) {
                Ok(found) => {
                    let flat: Vec<i32> = found.iter().flat_map(|&(x, y, _)| [x, y]).collect();
                    let distances: Vec<i32> = found.iter().map(|&(_, _, d)| d).collect();
                    self.logic_write_i32s(ptr(0), &flat)?;
                    self.logic_write_i32s(ptr(1), &distances)?;
                    self.logic_write_i32(ptr(2), found.len() as i32)?;
                    0
                }
                Err(status) => status,
            },
            // (out*, tx, ty, h, unit)
            0x2C => match self
                .logic
                .grid(int(3))
                .and_then(|grid| grid.unit_facing_delta(int(4), int(1), int(2)))
            {
                Ok(delta) => {
                    self.logic_write_i32(ptr(0), delta)?;
                    0
                }
                Err(status) => status,
            },
            // (out*, h, tx, ty, fx, fy)
            0x2D => match self.logic.grid(int(1)) {
                Ok(_) => {
                    let dir = grid::direction_to(int(2), int(3), int(4), int(5));
                    self.logic_write_i32(ptr(0), dir)?;
                    0
                }
                Err(status) => status,
            },
            _ => return Ok(None),
        };
        Ok(Some(status_to_script(result)))
    }

    fn logic_read(&self, ptr: u32, len: usize) -> VmResult<Vec<u8>> {
        if len == 0 {
            return Ok(Vec::new());
        }
        let range = self.resolve_range(Self::translate_system_descriptor(ptr), len)?;
        Ok(self.memory[range].to_vec())
    }

    /// Writes through a script pointer; a null pointer is skipped (the target
    /// would fault).
    fn logic_write(&mut self, ptr: u32, bytes: &[u8]) -> VmResult<()> {
        if ptr == 0 || bytes.is_empty() {
            return Ok(());
        }
        let ptr = Self::translate_system_descriptor(ptr);
        let range = self.resolve_write_range(ptr, bytes.len())?;
        self.memory[range].copy_from_slice(bytes);
        self.clear_shadow_values(ptr, bytes.len());
        Ok(())
    }

    fn logic_write_i32(&mut self, ptr: u32, value: i32) -> VmResult<()> {
        self.logic_write(ptr, &value.to_le_bytes())
    }

    fn logic_write_i32s(&mut self, ptr: u32, values: &[i32]) -> VmResult<()> {
        let bytes: Vec<u8> = values
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect();
        self.logic_write(ptr, &bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_statuses_follow_sub_497120() {
        assert_eq!(status_to_script(0), 0);
        assert_eq!(status_to_script(status::NO_HANDLE), 1);
        assert_eq!(status_to_script(status::BAD_DIRECTION), 12);
        assert_eq!(status_to_script(0x8000_000C), -1);
        assert_eq!(status_to_script(0x9000_0003), 17);
        assert_eq!(status_to_script(0xA000_0000), 1);
        assert_eq!(status_to_script(0xA000_0003), 20);
        assert_eq!(status_to_script(0xA000_0004), 8);
        assert_eq!(status_to_script(0xA000_0006), 22);
        assert_eq!(status_to_script(status::BROKEN_PATH), -2);
        assert_eq!(status_to_script(status::FAILED), -1);
    }
}
