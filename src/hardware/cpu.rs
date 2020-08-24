use log::{info, error};

fn error_unknown_opcode(opcode: u8, registers: &super::registers::Registers)
{
    error!("Unknown opcode {opcode:#X}. Registers: {registers:?}",
                opcode=opcode,
                registers=registers
          );
    panic!("Unknown opcode");
}

fn increment_pc(hardware: &mut super::gameboy::GameBoy)
{
    increment_pc_by(hardware, 1)
}

fn increment_pc_by(hardware: &mut super::gameboy::GameBoy, amount: u16)
{
    hardware.registers.pc += amount
}

fn get_16_bit_value(hardware: &mut super::gameboy::GameBoy, start_index: usize) -> u16
{
    let l = hardware.memory_map[start_index] as u16;
    let h = (hardware.memory_map[start_index + 1] as u16) << 8;
    h + l
}

fn jump_absolute_16_bit(hardware: &mut super::gameboy::GameBoy)
{
    hardware.registers.pc = get_16_bit_value(hardware, (hardware.registers.pc + 1) as usize);
    info!("Absolute 16bit jump to {destination:#X}", destination=hardware.registers.pc);
}

fn disable_interrupts(hardware: &mut super::gameboy::GameBoy)
{
    // TODO implement
    increment_pc(hardware);
}

fn load_to_sp(hardware: &mut super::gameboy::GameBoy)
{
    hardware.registers.sp = get_16_bit_value(hardware, (hardware.registers.pc + 1) as usize);
    info!("Loading {value:#X} to SP", value = hardware.registers.sp);
    increment_pc_by(hardware, 3);
}

pub fn step(hardware: &mut super::gameboy::GameBoy)
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