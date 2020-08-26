/*

The memory map is as follows:
0000 - 3FFF: 16KB ROM Bank 00, Cartridge / ROM is saved here
4000 - 7FFF: 16KB ROM Bank 01~7F, Cartridge if MBC
8000 - 9FFF: 8KB Video RAM:
  * 8000 - 87FF: Tileset 1
  * 8800 - 9000: Shared content
  * 9000 - 97FF: Tileset 2
A000 - BFFF: 8KB External RAM, in cartridge
C000 - CFFF: 4KB Work RAM, bank 0
D000 - DFFF: 4KB Work Ram, bank 1~N
E000 - FDFF: Mirror of C000 - DDFF
FE00 - FE9F: Sprite Attribute Table
FEA0 - FEFF: Unusable
FF00 - FF7F: I/O Registers
FF80 - FFFE: High RAM
FFFF - FFFF: Interruptes Enable Register

*/

pub struct GameBoy {
    pub registers: super::registers::Registers,
    pub memory_map: [u8; 0x10000],
}

impl Default for GameBoy {
    fn default() -> GameBoy
    {
        GameBoy {
            registers: super::registers::Registers::default(),
            memory_map: [0; 0x10000],
        }
    }
}

impl GameBoy {
    pub fn map_cartridge(&mut self, rom: &Vec<u8>)
    {
        // Map till 0x3FFF
        for index in 0..0x3FFF+1
        {
            self.memory_map[index] = rom[index]
        }
    }
}

