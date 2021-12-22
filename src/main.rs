use std::u8;
use std::fmt;

// https://www.rapidtables.com/convert/color/how-hex-to-rgb.html


pub enum Colors {
    Hex(String),
    RGB(u8, u8, u8),
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colors::Hex(val) => f.debug_struct("Hex").field("hex", val).finish(),
            Colors::RGB(r, g, b) => f.debug_struct("RGB").field("r", r).field("g", g).field("b", b).finish(),
        }
    }
}

impl From<String> for Colors::Hex {
    fn from(color_str: String) -> Self {
        Colors::Hex(color_str)
    }
}

impl From<String> for Colors::RGB {
    fn from(color_str: String) -> Self {
        let tmp: Vec<u8> = color_str.split(",").map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        Colors::RGB(tmp[0], tmp[1], tmp[2])
    }
}


/*impl From<String> for Colors::Hex {
    fn from(hex_str: String) -> Self {
        Colors::Hex{val: hex_str}
    }
}*/

/*impl From<Colors::Hex> for Colors::RGB {
    fn from(hex_color: Colors::Hex) -> Self {
        let hex_chars: Vec<char> = hex_color.chars().collect();
        RGB{u8::from_str_radix(hex_color[0..2].iter().collect()), u8::from_str_radix(hex_color[2..4].iter().collect()), u8::from_str_radix(hex_color[4..6].iter().collect())}
    }
}*/


fn main() {
    let hex = Colors::Hex(String::from("FF0000"));

    println!("{}", hex);
}
