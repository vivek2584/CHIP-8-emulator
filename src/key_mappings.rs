use minifb::Key;

pub fn get_hex(key: Key) -> Option<u8> {
    match key {
        Key::Key1 => Some(0x1),
        Key::Key2 => Some(0x2),
        Key::Key3 => Some(0x3),
        Key::Key4 => Some(0xC),
        Key::Q => Some(0x4),
        Key::W => Some(0x5),
        Key::E => Some(0x6),
        Key::R => Some(0xD),
        Key::A => Some(0x7),
        Key::S => Some(0x8),
        Key::D => Some(0x9),
        Key::F => Some(0xE),
        Key::Z => Some(0xA),
        Key::X => Some(0x0),
        Key::C => Some(0xB),
        Key::V => Some(0xF),
        _ => None,
    }
}

pub fn get_key(hex: u8) -> Option<Key> {
    match hex {
        0x00 => Some(Key::X),
        0x01 => Some(Key::Key1),
        0x02 => Some(Key::Key2),
        0x03 => Some(Key::Key3),
        0x04 => Some(Key::Q),
        0x05 => Some(Key::W),
        0x06 => Some(Key::E),
        0x07 => Some(Key::A),
        0x08 => Some(Key::S),
        0x09 => Some(Key::D),
        0x0A => Some(Key::Z),
        0x0B => Some(Key::C),
        0x0C => Some(Key::Key4),
        0x0D => Some(Key::R),
        0x0E => Some(Key::F),
        0x0F => Some(Key::V),
        _ => None,
    }
}
