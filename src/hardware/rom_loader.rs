use log::{error};

pub fn read_rom(path: &str) -> Vec<u8>
{
    std::fs::read(path).unwrap()
}

fn check_nintendo_logo(logo: &[u8]) -> bool
{
    let nintendo_logo = [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 
        0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 
        0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
    ];

    for index in 0..nintendo_logo.len()
    {
        if nintendo_logo[index] != logo[index]
        {
            error!("Logo incorrect @ {index}: {nlogo:#X} == {romlogo:#X}", index=index, nlogo=nintendo_logo[index], romlogo=logo[index]);
            return false;
        }
    }
    true
}

pub fn check_valid(rom: &Vec<u8>) -> bool
{
    // TODO: Also check checksums here
    let slice = &rom[0x0104..0x0134];
    check_nintendo_logo(slice)
}

pub fn get_rom_name(rom: &Vec<u8>) -> String
{
    let chars_raw = &rom[0x0134..0x0144];
    let mut chars: Vec<char> = vec![];
    for character in chars_raw
    {
        chars.push(*character as char);
    }
    chars.into_iter().collect()
}