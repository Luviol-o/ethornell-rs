use super::{Value, Vm, VmError, VmResult};
use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub(super) struct ExtendedOpcodeState {
    memory_handle_mode: bool,
    clipboard_text: String,
    last_modal_list: Option<(i32, String)>,
    last_resource_transform: [i32; 7],
    last_resource_blend: [i32; 5],
    next_legacy_handle: i32,
    legacy_devices: BTreeSet<i32>,
    legacy_meshes: BTreeSet<i32>,
    legacy_workers: BTreeSet<i32>,
    last_legacy_3d_call: Option<(u8, Vec<i32>)>,
    last_debug_inspection: Option<u8>,
}

impl Vm {
    pub(super) fn execute_qword_arithmetic(&mut self, opcode: u8) -> VmResult<()> {
        let right = self.pop_ptr()?;
        let left = self.pop_ptr()?;
        let destination = self.pop_ptr()?;
        let right = self.read_i64(right)?;
        let left = self.read_i64(left)?;
        let result = match opcode {
            0x50 => left.wrapping_add(right),
            0x51 => left.wrapping_sub(right),
            0x52 => left.wrapping_mul(right),
            0x53 if right != 0 => left.wrapping_div(right),
            0x54 if right != 0 => left.wrapping_rem(right),
            0x53 | 0x54 => -1,
            _ => return Err(VmError::UnsupportedInstruction(opcode)),
        };
        self.write_i64(destination, result)
    }

    /// 0x56/0x57 (sub_4747C0, sub_4747F0): the first value popped is the
    /// right operand.
    pub(super) fn execute_fixed_arithmetic(&mut self, opcode: u8) -> VmResult<i32> {
        let right = self.pop_int()?;
        let left = self.pop_int()?;
        Ok(match opcode {
            0x56 => fixed_mul(left, right),
            0x57 if right == 0 => 0,
            0x57 => float_to_fixed(fixed_to_float(left) / fixed_to_float(right)),
            _ => return Err(VmError::UnsupportedInstruction(opcode)),
        })
    }

    /// 0x58..=0x5F (sub_474890..sub_474B40): SSE helpers over four 16.16
    /// lanes. Pointers pop right operand first and the destination last.
    pub(super) fn execute_fixed_vector(&mut self, opcode: u8) -> VmResult<()> {
        let result = match opcode {
            0x58..=0x5b => {
                let right = self.pop_ptr()?;
                let left = self.pop_ptr()?;
                let destination = self.pop_ptr()?;
                let (left, right) = (self.read_fixed4(left)?, self.read_fixed4(right)?);
                let lanes = std::array::from_fn(|lane| {
                    let (left, right) = (left[lane], right[lane]);
                    match opcode {
                        0x58 => left.wrapping_add(right),
                        0x59 => left.wrapping_sub(right),
                        0x5a => fixed_mul(left, right),
                        _ if right == 0 => 0,
                        _ => float_to_fixed(fixed_to_float(left) / fixed_to_float(right)),
                    }
                });
                (destination, lanes)
            }
            0x5d => {
                let source = self.pop_ptr()?;
                let destination = self.pop_ptr()?;
                let [x, y, z, _] = self.read_fixed4(source)?.map(fixed_to_float);
                // sub_4558C0 sums z² + y² + x² and returns zero for a zero
                // vector; w becomes the length.
                let squared = z * z + y * y + x * x;
                let lanes = if squared > 0.0 {
                    let inverse = 1.0 / squared.sqrt();
                    [x * inverse, y * inverse, z * inverse, squared * inverse].map(float_to_fixed)
                } else {
                    [0; 4]
                };
                (destination, lanes)
            }
            0x5e | 0x5f => {
                let scalar = self.pop_int()?;
                let source = self.pop_ptr()?;
                let destination = self.pop_ptr()?;
                let source = self.read_fixed4(source)?;
                let lanes = match opcode {
                    0x5e => source.map(|lane| fixed_mul(lane, scalar)),
                    _ if scalar == 0 => [0; 4],
                    _ => source
                        .map(|lane| float_to_fixed(fixed_to_float(lane) / fixed_to_float(scalar))),
                };
                (destination, lanes)
            }
            _ => return Err(VmError::UnsupportedInstruction(opcode)),
        };
        let (destination, lanes) = result;
        for (index, lane) in lanes.into_iter().enumerate() {
            self.write_int(destination.wrapping_add(index as u32 * 4), 2, lane as u32)?;
        }
        Ok(())
    }

    fn read_fixed4(&self, pointer: u32) -> VmResult<[i32; 4]> {
        let mut lanes = [0; 4];
        for (index, lane) in lanes.iter_mut().enumerate() {
            *lane = self.read_int(pointer.wrapping_add(index as u32 * 4), 2)? as i32;
        }
        Ok(lanes)
    }

    pub(super) fn execute_quote_string(&mut self) -> VmResult<()> {
        let delimiter = char::from(self.pop_int()? as u8);
        let source = self.pop_string_lossy()?;
        let destination = self.pop_ptr()?;
        self.write_c_string(destination, &format!("{delimiter}{source}{delimiter}"))
    }

    pub(super) fn execute_set_memory_mode(&mut self) -> VmResult<()> {
        self.extended_opcodes.memory_handle_mode = self.pop_int()? != 0;
        Ok(())
    }

    pub(super) fn execute_modal_list(&mut self) -> VmResult<i32> {
        let text = self.pop_string_lossy()?;
        let owner = self.pop_int()?;
        self.extended_opcodes.last_modal_list = Some((owner, text));
        // This opcode is an engine diagnostics dialog. Windowless validation
        // models the native cancel result instead of opening a second window.
        Ok(0)
    }

    pub(super) fn execute_resource_transform(&mut self) -> VmResult<()> {
        let mut values = [0; 7];
        for value in &mut values {
            *value = self.pop_int()?;
        }
        self.extended_opcodes.last_resource_transform = values;
        Ok(())
    }

    pub(super) fn execute_clipboard_set(&mut self) -> VmResult<i32> {
        self.extended_opcodes.clipboard_text = self.pop_string_lossy()?;
        Ok(1)
    }

    pub(super) fn execute_resource_blend(&mut self) -> VmResult<()> {
        let mut values = [0; 5];
        for value in &mut values {
            *value = self.pop_int()?;
        }
        self.extended_opcodes.last_resource_blend = values;
        Ok(())
    }

    pub(super) fn execute_legacy_3d(&mut self, id: u8) -> VmResult<Option<Value>> {
        let Some((argc, returns_value)) = legacy_3d_abi(id) else {
            return Err(VmError::UnknownDispatch {
                group: 0xd0,
                id: u16::from(id),
            });
        };
        let mut args = Vec::with_capacity(argc);
        for _ in 0..argc {
            args.push(self.pop_value()?);
        }
        let values = args.iter().map(Value::as_i32).collect::<Vec<_>>();
        self.extended_opcodes.last_legacy_3d_call = Some((id, values.clone()));

        let result = match id {
            0x00 => {
                let handle = self.alloc_legacy_handle();
                self.extended_opcodes.legacy_devices.insert(handle);
                self.write_legacy_output(values.get(2).copied(), handle)?;
                1
            }
            0x01 => i32::from(
                values
                    .first()
                    .is_some_and(|handle| self.extended_opcodes.legacy_devices.remove(handle)),
            ),
            0x40 => {
                let handle = self.alloc_legacy_handle();
                self.extended_opcodes.legacy_meshes.insert(handle);
                self.write_legacy_output(values.first().copied(), handle)?;
                0
            }
            0x41 => i32::from(
                !values
                    .first()
                    .is_some_and(|handle| self.extended_opcodes.legacy_meshes.remove(handle)),
            ),
            0x80 => {
                let handle = self.alloc_legacy_handle();
                self.extended_opcodes.legacy_workers.insert(handle);
                self.write_legacy_output(values.first().copied(), handle)?;
                0
            }
            0x81 => i32::from(
                !values
                    .first()
                    .is_some_and(|handle| self.extended_opcodes.legacy_workers.remove(handle)),
            ),
            0x2d => {
                self.write_legacy_output(values.get(5).copied(), 0)?;
                0
            }
            0x65 | 0x67 => {
                if let Some(pointer) = values.get(2).copied().filter(|pointer| *pointer != 0) {
                    let pointer = Self::translate_system_descriptor(pointer as u32);
                    for offset in [0, 4, 8] {
                        self.write_int(pointer.wrapping_add(offset), 2, 0)?;
                    }
                }
                0
            }
            0x87 => {
                self.write_legacy_output(values.get(1).copied(), -65_536)?;
                0
            }
            _ => 0,
        };

        Ok(returns_value.then_some(Value::Int(result)))
    }

    pub(super) fn execute_debug_inspect(&mut self, id: u8) -> VmResult<()> {
        if !matches!(id, 0x00 | 0x40 | 0x80) {
            return Err(VmError::UnknownDispatch {
                group: 0xe0,
                id: u16::from(id),
            });
        }
        self.extended_opcodes.last_debug_inspection = Some(id);
        Ok(())
    }

    fn read_i64(&self, pointer: u32) -> VmResult<i64> {
        let range = self.resolve_range(pointer, 8)?;
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&self.memory[range]);
        Ok(i64::from_le_bytes(bytes))
    }

    fn write_i64(&mut self, pointer: u32, value: i64) -> VmResult<()> {
        let range = self.resolve_write_range(pointer, 8)?;
        self.memory[range].copy_from_slice(&value.to_le_bytes());
        self.clear_shadow_values(pointer, 8);
        Ok(())
    }

    fn alloc_legacy_handle(&mut self) -> i32 {
        self.extended_opcodes.next_legacy_handle = self
            .extended_opcodes
            .next_legacy_handle
            .wrapping_add(1)
            .max(1);
        self.extended_opcodes.next_legacy_handle
    }

    fn write_legacy_output(&mut self, pointer: Option<i32>, value: i32) -> VmResult<()> {
        let Some(pointer) = pointer.filter(|pointer| *pointer != 0) else {
            return Ok(());
        };
        self.write_int(
            Self::translate_system_descriptor(pointer as u32),
            2,
            value as u32,
        )
    }
}

fn fixed_mul(left: i32, right: i32) -> i32 {
    ((i64::from(left) * i64::from(right)) >> 16) as i32
}

fn fixed_to_float(value: i32) -> f32 {
    value as f32 * (1.0 / 65_536.0)
}

/// sub_455940: round to the nearest 16.16 value (ties to even, MXCSR
/// rounding cleared) and snap a fraction within 0x40 of an integer to it.
/// Out-of-range and NaN lanes become the cvtps2dq indefinite value.
fn float_to_fixed(value: f32) -> i32 {
    let scaled = (value * 65_536.0).round_ties_even();
    let raw = if scaled >= -2_147_483_648.0 && scaled < 2_147_483_648.0 {
        scaled as i32
    } else {
        i32::MIN
    };
    let whole = raw & !0xffff;
    let fraction = raw.wrapping_sub(whole);
    if fraction > 0xffc0 {
        whole.wrapping_add(0x10000)
    } else if fraction < 0x40 {
        whole
    } else {
        raw
    }
}

fn legacy_3d_abi(id: u8) -> Option<(usize, bool)> {
    Some(match id {
        0x00 => (3, true),
        0x01 => (1, true),
        0x04 => (4, true),
        0x05 => (3, true),
        0x10 | 0x11 | 0x12 => (2, true),
        0x14 | 0x15 => (4, true),
        0x16 | 0x17 | 0x18 => (3, true),
        0x20 | 0x21 => (6, true),
        0x22 => (3, true),
        0x23 => (4, true),
        0x28 => (11, true),
        0x2c => (5, true),
        0x2d => (6, true),
        0x40 | 0x41 => (1, true),
        0x60 => (13, true),
        0x61 => (2, true),
        0x64 => (5, true),
        0x65 => (3, true),
        0x66 => (5, true),
        0x67 | 0x68 | 0x69 => (4, true),
        0x6a => (7, true),
        0x70 => (9, true),
        0x72 => (5, true),
        0x74 => (8, true),
        0x75 => (7, true),
        0x78 => (5, true),
        0x79 => (4, true),
        // sub_4787C0 pops five integers and one pointer.
        0x7a => (6, true),
        0x80 => (1, false),
        0x81 => (1, true),
        0x84 | 0x87 | 0x8c => (2, true),
        0x88 | 0x8a | 0x8d => (5, true),
        0x8e => (3, true),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_fixed4(vm: &mut Vm, pointer: u32, lanes: [i32; 4]) {
        for (index, lane) in lanes.into_iter().enumerate() {
            vm.write_int(pointer + index as u32 * 4, 2, lane as u32)
                .unwrap();
        }
    }

    #[test]
    fn fixed_arithmetic_pops_the_right_operand_first() {
        let mut vm = Vm::new();
        vm.stack.extend([Value::Int(3 << 16), Value::Int(0x8000)]);
        assert_eq!(vm.execute_fixed_arithmetic(0x56).unwrap(), 0x18000);
        vm.stack.extend([Value::Int(3 << 16), Value::Int(2 << 16)]);
        assert_eq!(vm.execute_fixed_arithmetic(0x57).unwrap(), 0x18000);
        vm.stack.extend([Value::Int(3 << 16), Value::Int(0)]);
        assert_eq!(vm.execute_fixed_arithmetic(0x57).unwrap(), 0);
    }

    #[test]
    fn fixed_vectors_follow_the_target_lane_rules() {
        let mut vm = Vm::new();
        let (dst, left, right) = (0x1200_0100, 0x1200_0110, 0x1200_0120);
        write_fixed4(&mut vm, left, [6 << 16, 1 << 16, -(4 << 16), 5]);
        write_fixed4(&mut vm, right, [2 << 16, 0, 2 << 16, 7]);

        vm.stack
            .extend([Value::Ptr(dst), Value::Ptr(left), Value::Ptr(right)]);
        vm.execute_fixed_vector(0x59).unwrap();
        assert_eq!(
            vm.read_fixed4(dst).unwrap(),
            [4 << 16, 1 << 16, -(6 << 16), -2]
        );

        vm.stack
            .extend([Value::Ptr(dst), Value::Ptr(left), Value::Ptr(right)]);
        vm.execute_fixed_vector(0x5b).unwrap();
        assert_eq!(
            vm.read_fixed4(dst).unwrap(),
            [3 << 16, 0, -(2 << 16), float_to_fixed(5.0 / 7.0)]
        );

        vm.stack
            .extend([Value::Ptr(dst), Value::Ptr(left), Value::Int(0x8000)]);
        vm.execute_fixed_vector(0x5e).unwrap();
        assert_eq!(
            vm.read_fixed4(dst).unwrap(),
            [3 << 16, 0x8000, -(2 << 16), 2]
        );

        vm.stack
            .extend([Value::Ptr(dst), Value::Ptr(left), Value::Int(0)]);
        vm.execute_fixed_vector(0x5f).unwrap();
        assert_eq!(vm.read_fixed4(dst).unwrap(), [0; 4]);
    }

    #[test]
    fn vec3_normalize_stores_the_length_in_w() {
        let mut vm = Vm::new();
        let (dst, source) = (0x1200_0100, 0x1200_0110);
        write_fixed4(&mut vm, source, [3 << 16, 0, 4 << 16, 9 << 16]);
        vm.stack.extend([Value::Ptr(dst), Value::Ptr(source)]);
        vm.execute_fixed_vector(0x5d).unwrap();
        assert_eq!(
            vm.read_fixed4(dst).unwrap(),
            [float_to_fixed(0.6), 0, float_to_fixed(0.8), 5 << 16]
        );

        write_fixed4(&mut vm, source, [0, 0, 0, 1 << 16]);
        vm.stack.extend([Value::Ptr(dst), Value::Ptr(source)]);
        vm.execute_fixed_vector(0x5d).unwrap();
        assert_eq!(vm.read_fixed4(dst).unwrap(), [0; 4]);
    }

    #[test]
    fn float_to_fixed_snaps_near_integers() {
        assert_eq!(float_to_fixed(1.0 + 0x3f as f32 / 65_536.0), 1 << 16);
        assert_eq!(float_to_fixed(2.0 - 0x3f as f32 / 65_536.0), 2 << 16);
        assert_eq!(float_to_fixed(1.5), 0x18000);
        assert_eq!(float_to_fixed(f32::NAN), i32::MIN);
    }

    #[test]
    fn qword_arithmetic_uses_native_pointer_operands() {
        let mut vm = Vm::new();
        vm.write_i64(0x1200_0100, 40).unwrap();
        vm.write_i64(0x1200_0108, 2).unwrap();
        vm.stack.extend([
            Value::Ptr(0x1200_0110),
            Value::Ptr(0x1200_0100),
            Value::Ptr(0x1200_0108),
        ]);
        vm.execute_qword_arithmetic(0x50).unwrap();
        assert_eq!(vm.read_i64(0x1200_0110).unwrap(), 42);
    }

    #[test]
    fn legacy_device_create_writes_the_output_handle() {
        let mut vm = Vm::new();
        vm.stack
            .extend([Value::Ptr(0x1200_0100), Value::Int(0), Value::Int(0)]);
        assert_eq!(vm.execute_legacy_3d(0x00).unwrap(), Some(Value::Int(1)));
        assert_eq!(vm.read_int(0x1200_0100, 2).unwrap(), 1);
    }
}
