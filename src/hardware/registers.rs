use std::convert::TryInto;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Registers
{
	pub a: u8, // acc/arg
	b: u8,
	c: u8,
	d: u8,
	e: u8,
	h: u8, // addr
	l: u8, // addr
	f: u8, // flags
	pub sp: u16, // stack pointer
	pub pc: u16, // program counter
}

impl Default for Registers {
    fn default() -> Registers
    {
        Registers {
            a: u8::default(),
            b: u8::default(),
            c: u8::default(),
            d: u8::default(),
            e: u8::default(),
            f: u8::default(),
            h: u8::default(),
            l: u8::default(),
            sp: u16::default(),
            pc: 0x100,
        }
    }
}

impl Registers
{
    #[allow(dead_code)]
    pub fn set_af(&mut self, af: u16)
    {
        self.a = ((af >> 8)).try_into().unwrap();
        self.f = ((((af << 8)) >> 8)).try_into().unwrap();
    }

    #[allow(dead_code)]
    pub fn get_af(&self) -> u16
    {
        let mut af: u16 = self.a as u16;
        af = ((af << 8) + (self.f as u16)).into();
        af
    }

    #[allow(dead_code)]
    pub fn set_bc(&mut self, bc: u16)
    {
        self.b = ((bc >> 8)).try_into().unwrap();
        self.c = ((((bc << 8)) >> 8)).try_into().unwrap();
    }

    #[allow(dead_code)]
    pub fn get_bc(&self) -> u16
    {
        let mut bc: u16 = self.b as u16;
        bc = ((bc << 8) + (self.c as u16)).into();
        bc
    }

    #[allow(dead_code)]
    pub fn set_de(&mut self, de: u16)
    {
        self.d = ((de >> 8)).try_into().unwrap();
        self.e = ((((de << 8)) >> 8)).try_into().unwrap();
    }

    #[allow(dead_code)]
    pub fn get_de(&self) -> u16
    {
        let mut de: u16 = self.d as u16;
        de = ((de << 8) + (self.e as u16)).into();
        de
    }
    
    #[allow(dead_code)]
    pub fn set_hl(&mut self, hl: u16)
    {
        self.h = ((hl >> 8)).try_into().unwrap();
        self.l = ((((hl << 8)) >> 8)).try_into().unwrap();
    }

    #[allow(dead_code)]
    pub fn get_hl(&self) -> u16
    {
        let mut hl: u16 = self.h as u16;
        hl = ((hl << 8) + (self.l as u16)).into();
        hl
    }

    #[allow(dead_code)]
    pub fn set_zero_flag(&mut self)
    {
        self.f |= 0x80; // 0b1000_0000
    }

    #[allow(dead_code)]
    pub fn unset_zero_flag(&mut self)
    {
        self.f &= !0x80; // 0b0111_1111
    }

    #[allow(dead_code)]
    pub fn is_zero_flag_set(&self) -> bool
    {
        (self.f & 0x80) > 0
    }

    #[allow(dead_code)]
    pub fn set_subtraction_flag(&mut self)
    {
        self.f |= 0x40; // 0b0100_0000
    }

    #[allow(dead_code)]
    pub fn unset_subtraction_flag(&mut self)
    {
        self.f &= !0x40; // 0b1011_1111
    }

    #[allow(dead_code)]
    pub fn is_subtraction_flag_set(&self) -> bool
    {
        (self.f & 0x40) > 0
    }

    #[allow(dead_code)]
    pub fn set_halfcarry_flag(&mut self)
    {
        self.f |= 0x20; // 0b0010_0000
    }

    #[allow(dead_code)]
    pub fn unset_halfcarry_flag(&mut self)
    {
        self.f &= !0x20; // 0b1101_1111
    }

    #[allow(dead_code)]
    pub fn is_halfcarry_flag_set(&self) -> bool
    {
        (self.f & 0x20) > 0
    }

    #[allow(dead_code)]
    pub fn set_carry_flag(&mut self)
    {
        self.f |= 0x10; // 0b0001_0000
    }

    #[allow(dead_code)]
    pub fn unset_carry_flag(&mut self)
    {
        self.f &= !0x10; // 0b1110_1111
    }

    #[allow(dead_code)]
    pub fn is_carry_flag_set(&self) -> bool
    {
        (self.f & 0x10) > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pc_starts_at_0x100()
    {
        let registers: Registers = Registers::default();

        assert_eq!(0x100, registers.pc);
    }

    #[test]
    fn set_a()
    {
        let mut registers: Registers = Registers::default();
        registers.a = 5;
        
        assert_eq!(5, registers.a);
    }

    #[test]
    fn set_af()
    {
        let mut registers: Registers = Registers::default();
        
        registers.set_af(0xFADA);
        
        assert_eq!(0xFA, registers.a);
        assert_eq!(0xDA, registers.f);
    }

    #[test]
    fn get_af()
    {
        let mut registers: Registers = Registers::default();
     
        registers.a = 0xFA;
        registers.f = 0xDA;

        assert_eq!(0xFADA, registers.get_af());
    }

    #[test]
    fn set_and_get_af()
    {
        let mut registers: Registers = Registers::default();
     
        registers.set_af(0xFADA);

        assert_eq!(0xFADA, registers.get_af());
    }

    #[test]
    fn set_bc()
    {
        let mut registers: Registers = Registers::default();
        
        registers.set_bc(0xFADA);
        
        assert_eq!(0xFA, registers.b);
        assert_eq!(0xDA, registers.c);
    }

    #[test]
    fn get_bc()
    {
        let mut registers: Registers = Registers::default();
     
        registers.b = 0xFA;
        registers.c = 0xDA;

        assert_eq!(0xFADA, registers.get_bc());
    }

    #[test]
    fn set_and_get_bc()
    {
        let mut registers: Registers = Registers::default();
     
        registers.set_bc(0xFADA);

        assert_eq!(0xFADA, registers.get_bc());
    }


    #[test]
    fn set_de()
    {
        let mut registers: Registers = Registers::default();
        
        registers.set_de(0xFADA);
        
        assert_eq!(0xFA, registers.d);
        assert_eq!(0xDA, registers.e);
    }

    #[test]
    fn get_de()
    {
        let mut registers: Registers = Registers::default();
     
        registers.d = 0xFA;
        registers.e = 0xDA;

        assert_eq!(0xFADA, registers.get_de());
    }

    #[test]
    fn set_and_get_de()
    {
        let mut registers: Registers = Registers::default();
     
        registers.set_de(0xFADA);

        assert_eq!(0xFADA, registers.get_de());
    }


    #[test]
    fn set_hl()
    {
        let mut registers: Registers = Registers::default();
        
        registers.set_hl(0xFADA);
        
        assert_eq!(0xFA, registers.h);
        assert_eq!(0xDA, registers.l);
    }

    #[test]
    fn get_hl()
    {
        let mut registers: Registers = Registers::default();
     
        registers.h = 0xFA;
        registers.l = 0xDA;

        assert_eq!(0xFADA, registers.get_hl());
    }

    #[test]
    fn set_and_get_hl()
    {
        let mut registers: Registers = Registers::default();
     
        registers.set_hl(0xFADA);

        assert_eq!(0xFADA, registers.get_hl());
    }

    #[test]
    fn set_zero_flag()
    {
        let mut registers: Registers = Registers::default();

        registers.set_zero_flag();

        assert_eq!(true, registers.is_zero_flag_set());
    }

    #[test]
    fn unset_zero_flag()
    {
        let mut registers: Registers = Registers::default();

        registers.f = 0xFF;
        registers.unset_zero_flag();

        assert_eq!(false, registers.is_zero_flag_set());
    }

    #[test]
    fn set_subtraction_flag()
    {
        let mut registers: Registers = Registers::default();
        
        registers.set_subtraction_flag();

        assert_eq!(true, registers.is_subtraction_flag_set());
    }

    #[test]
    fn unset_subtraction_flag()
    {
        let mut registers:Registers = Registers::default();

        registers.f = 0xFF;
        registers.unset_subtraction_flag();

        assert_eq!(false, registers.is_subtraction_flag_set());
    }

    #[test]
    fn set_halfcarry_flag()
    {
        let mut registers: Registers = Registers::default();

        registers.set_halfcarry_flag();

        assert_eq!(true, registers.is_halfcarry_flag_set());
    }

    #[test]
    fn unset_halfcarry_flag()
    {
        let mut registers: Registers = Registers::default();

        registers.f = 0xFF;
        registers.unset_halfcarry_flag();

        assert_eq!(false, registers.is_halfcarry_flag_set());
    }

    #[test]
    fn set_carry_flag()
    {
        let mut registers: Registers = Registers::default();

        registers.set_carry_flag();

        assert_eq!(true, registers.is_carry_flag_set());
    }

    #[test]
    fn unset_carry_flag()
    {
        let mut registers: Registers = Registers::default();

        registers.f = 0xFF;
        registers.unset_carry_flag();

        assert_eq!(false, registers.is_carry_flag_set());
    }
}