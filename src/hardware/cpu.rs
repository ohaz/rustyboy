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
    hardware.registers.pc += 1;
}

fn jump_absolute_16_bit(hardware: &mut super::gameboy::GameBoy)
{
    let l = hardware.memory_map[(hardware.registers.pc + 1) as usize] as u16;
    let h = (hardware.memory_map[(hardware.registers.pc + 2) as usize] as u16) << 8;
    let destination: u16 = h + l;
    hardware.registers.pc = destination;
    info!("Absolute 16bit jump to {destination:#X}", destination=destination);
}

fn disable_interrupts(hardware: &mut super::gameboy::GameBoy)
{
    // TODO implement
    increment_pc(hardware);
}

pub fn step(hardware: &mut super::gameboy::GameBoy)
{
    let opcode = hardware.memory_map[hardware.registers.pc as usize];
    info!("Parsed opcode: {opcode:#X}", opcode=opcode);
    match opcode{
        0x00 => increment_pc(hardware), // NOP
        0xC3 => jump_absolute_16_bit(hardware),
        0xF3 => disable_interrupts(hardware),
        x => error_unknown_opcode(x, &hardware.registers)
    };
}