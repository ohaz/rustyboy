use log::{info, error};

use super::gameboy::GameBoy;

fn error_unknown_opcode(opcode: u8, registers: &super::registers::Registers)
{
    error!("Unknown opcode {opcode:#X}. Registers: {registers:?}",
                opcode=opcode,
                registers=registers
          );
    panic!("Unknown opcode");
}

fn increment_pc(hardware: &mut GameBoy)
{
    increment_pc_by(hardware, 1)
}

fn increment_pc_by(hardware: &mut GameBoy, amount: u16)
{
    hardware.registers.pc += amount
}

fn get_16_bit_value(hardware: &mut GameBoy, start_index: usize) -> u16
{
    let l = hardware.memory_map[start_index] as u16;
    let h = (hardware.memory_map[start_index + 1] as u16) << 8;
    h + l
}

fn get_8_bit_value(hardware: &mut GameBoy, start_index: usize) -> u8
{
    hardware.memory_map[start_index]
}

fn jump_absolute_16_bit(hardware: &mut GameBoy)
{
    hardware.registers.pc = get_16_bit_value(hardware, (hardware.registers.pc + 1) as usize);
    info!("Absolute 16bit jump to {destination:#X}", destination=hardware.registers.pc);
}

fn disable_interrupts(hardware: &mut GameBoy)
{
    // TODO implement
    increment_pc(hardware);
}

fn load_to_sp(hardware: &mut GameBoy)
{
    hardware.registers.sp = get_16_bit_value(hardware, (hardware.registers.pc + 1) as usize);
    info!("Loading {value:#X} to SP", value = hardware.registers.sp);
    increment_pc_by(hardware, 3);
}

fn save_a_to_address(hardware: &mut GameBoy)
{
    let location = get_16_bit_value(hardware, (hardware.registers.pc + 1) as usize);
    hardware.memory_map[location as usize] = hardware.registers.a;
    info!("Loading register A: {a:#X} into memory[{location:#X}]", a=hardware.registers.a, location=location);
    increment_pc_by(hardware, 3);
}

fn load_8bit_intermediate_to_a(hardware: &mut GameBoy)
{
    let intermediate = get_8_bit_value(hardware, (hardware.registers.pc + 1) as usize);
    hardware.registers.a = intermediate;
    info!("Loading {value:#X} into register A", value=intermediate);
    increment_pc_by(hardware, 2);
}

fn save_a_to_ff00_plus_intermediate(hardware: &mut GameBoy)
{
    let location = 0xFF00u16 + get_8_bit_value(hardware, (hardware.registers.pc + 1) as usize) as u16;
    hardware.memory_map[location as usize] = hardware.registers.a;
    info!("Saving register A: {a:#X} to memory[{location:#X}]", a=hardware.registers.a, location=location);
    increment_pc_by(hardware, 2);
}

fn load_16bit_intermediate_to_hl(hardware: &mut GameBoy)
{
    let value = get_16_bit_value(hardware, (hardware.registers.pc + 1) as usize);
    hardware.registers.set_hl(value);
    info!("Setting HL to {value:#X}", value=value);
    increment_pc_by(hardware, 3);
}

fn call(hardware: &mut GameBoy)
{
    hardware.memory_map[(hardware.registers.sp - 1) as usize] = (hardware.registers.pc >> 8) as u8;
    hardware.memory_map[(hardware.registers.sp - 2) as usize] = ((hardware.registers.pc << 8) >> 8) as u8;
    hardware.registers.pc = get_16_bit_value(hardware, (hardware.registers.pc + 1) as usize);
    hardware.registers.sp -= 2;
    info!("Calling {pc:#X}", pc=hardware.registers.pc);
}

fn return_from_call(hardware: &mut GameBoy)
{
    let pc_lower = hardware.memory_map[hardware.registers.sp as usize];
    let pc_higher = hardware.memory_map[(hardware.registers.sp + 1) as usize];
    hardware.registers.pc = ((pc_higher as u16) << 8) + pc_lower as u16;
    hardware.registers.sp += 2;
    info!("Returning to {pc:#X}", pc=hardware.registers.pc);
    increment_pc(hardware);
}

fn copy_l_to_a(hardware: &mut GameBoy)
{
    hardware.registers.a = hardware.registers.l;
    increment_pc(hardware);
}

fn copy_h_to_a(hardware: &mut GameBoy)
{
    hardware.registers.a = hardware.registers.h;
    increment_pc(hardware);
}

fn jump_signed_immediate(hardware: &mut GameBoy)
{
    let jump_size: i8 = get_8_bit_value(hardware, (hardware.registers.pc + 1) as usize) as i8;
    hardware.registers.pc = hardware.registers.pc.wrapping_add(jump_size as u16);
    info!("Jumped by {jump_size} to {pc:#X}", jump_size=jump_size, pc=hardware.registers.pc);
}

pub fn step(hardware: &mut GameBoy)
{
    let opcode = hardware.memory_map[hardware.registers.pc as usize];
    info!("Parsed opcode: {opcode:#X}", opcode=opcode);
    match opcode {
        0x00 => increment_pc(hardware), // NOP
        0xC3 => jump_absolute_16_bit(hardware),
        0xF3 => disable_interrupts(hardware),
        0x31 => load_to_sp(hardware),
        0xEA => save_a_to_address(hardware),
        0x3E => load_8bit_intermediate_to_a(hardware),
        0xE0 => save_a_to_ff00_plus_intermediate(hardware),
        0x21 => load_16bit_intermediate_to_hl(hardware),
        0xCD => call(hardware),
        0x7D => copy_l_to_a(hardware),
        0x7C => copy_h_to_a(hardware),
        0x18 => jump_signed_immediate(hardware),
        0xC9 => return_from_call(hardware),
        x => error_unknown_opcode(x, &hardware.registers)
    };
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn return_from_call_to_0x1000()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.sp = 0x2000;
        gameboy.memory_map[0x2000] = 0x44;
        gameboy.memory_map[0x2001] = 0x55;

        return_from_call(&mut gameboy);

        assert_eq!(0x5545, gameboy.registers.pc);
        assert_eq!(0x2002, gameboy.registers.sp);
    }

    #[test]
    fn jump_signed_immediate_by_minus5()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.pc = 0x1000;
        let jump_size: i8 = -5;
        gameboy.memory_map[0x1001] = jump_size as u8;

        jump_signed_immediate(&mut gameboy);

        assert_eq!(0x0FFB, gameboy.registers.pc);
    }

    #[test]
    fn jump_signed_immediate_by_5()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.pc = 0x1000;
        gameboy.memory_map[0x1001] = 5;

        jump_signed_immediate(&mut gameboy);

        assert_eq!(0x1005, gameboy.registers.pc);
    }

    #[test]
    fn copy_h_to_a_0xfa()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.h = 0xFA;

        copy_h_to_a(&mut gameboy);

        assert_eq!(0xFA, gameboy.registers.a);
    }

    #[test]
    fn copy_l_to_a_0xfa()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.l = 0xFA;

        copy_l_to_a(&mut gameboy);

        assert_eq!(0xFA, gameboy.registers.a);
    }

    #[test]
    fn call_check_stack_and_pc()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.sp = 0x2000;
        gameboy.registers.pc = 0x1234;
        gameboy.memory_map[0x1235] = 0x33;
        gameboy.memory_map[0x1236] = 0x44;

        call(&mut gameboy);

        assert_eq!(0x4433, gameboy.registers.pc);
        assert_eq!(0x1FFE, gameboy.registers.sp);
        assert_eq!(0x34, gameboy.memory_map[0x1FFE]);
        assert_eq!(0x12, gameboy.memory_map[0x1FFF]);

    }

    #[test]
    fn load_16bit_intermediate_to_hl_0xffee()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.pc = 0x1000;
        gameboy.memory_map[0x1001] = 0xEE;
        gameboy.memory_map[0x1002] = 0xFF;

        load_16bit_intermediate_to_hl(&mut gameboy);

        assert_eq!(0xFFEE, gameboy.registers.get_hl());
    }

    #[test]
    fn save_a_to_ff00_plus_intermediate_5_to_ff04()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.pc = 0x1000;
        gameboy.memory_map[0x1001] = 0x4;
        gameboy.registers.a = 0x5;

        save_a_to_ff00_plus_intermediate(&mut gameboy);

        assert_eq!(0x5, gameboy.memory_map[0xff04]);
    }

    #[test]
    fn load_8bit_intermediate_to_a_0x34()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.pc = 0x1000;
        gameboy.memory_map[0x1001] = 0x34;

        load_8bit_intermediate_to_a(&mut gameboy);

        assert_eq!(0x34, gameboy.registers.a)
    }

    #[test]
    fn save_a_to_address_5_to_1234()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.a = 5;
        gameboy.registers.pc = 0x1000;

        gameboy.memory_map[0x1001] = 0x34;
        gameboy.memory_map[0x1002] = 0x12;

        save_a_to_address(&mut gameboy);

        assert_eq!(0x5, gameboy.memory_map[0x1234]);
    }

    #[test]
    fn increment_program_counter_by_5()
    {
        let mut gameboy = GameBoy::default();
        gameboy.registers.pc = 0x1000;

        increment_pc_by(&mut gameboy, 5);

        assert_eq!(0x1005, gameboy.registers.pc);
    }

    #[test]
    fn get_16_bit_value_0xcdab_returns_0xabcd()
    {
        let mut gameboy = GameBoy::default();
        gameboy.memory_map[5] = 0xCD;
        gameboy.memory_map[6] = 0xAB;

        assert_eq!(0xABCD, get_16_bit_value(&mut gameboy, 5));
    }

    #[test]
    fn jump_absolute_16_bit_jump_to_0x1234_pc_is_set()
    {
        let mut gameboy = GameBoy::default();
        gameboy.memory_map[5] = 0x34;
        gameboy.memory_map[6] = 0x12;

        gameboy.registers.pc = 4;

        jump_absolute_16_bit(&mut gameboy);

        assert_eq!(0x1234, gameboy.registers.pc);
    }

    #[test]
    fn load_to_sp_0x1234_sp_is_set_and_pc_increased()
    {
        let mut gameboy = GameBoy::default();
        gameboy.memory_map[5] = 0x34;
        gameboy.memory_map[6] = 0x12;

        gameboy.registers.pc = 4;

        load_to_sp(&mut gameboy);

        assert_eq!(0x1234, gameboy.registers.sp);
    }
}
