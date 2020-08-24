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

pub fn step(hardware: &mut GameBoy)
{
    let opcode = hardware.memory_map[hardware.registers.pc as usize];
    info!("Parsed opcode: {opcode:#X}", opcode=opcode);
    match opcode{
        0x00 => increment_pc(hardware), // NOP
        0xC3 => jump_absolute_16_bit(hardware),
        0xF3 => disable_interrupts(hardware),
        0x31 => load_to_sp(hardware),
        x => error_unknown_opcode(x, &hardware.registers)
    };
}

#[cfg(test)]
mod tests
{
    use super::*;

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
