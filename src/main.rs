// https://www.rapidtables.com/convert/color/how-hex-to-rgb.html
// RGB: 34, 139, 34 (3 8bit integers)
// Hex: 228B22      (3 8bit integers as hex)
// HSL: 120, 67%, 33.9% (look it up)
// HSV: 120, 75.5%, 54.5% (also look it up)
// CMYK: 76, 0, 76, 45 (again look it up

use std::fmt;
use std::marker::PhantomData;


// doing this with new is dumb
// it should probably be done with a
// bunch of from_whatever functions
// colorbytes::from_hex(<hex>)
// like how string is done
// new can do its things somehow, maybe no new
// 

#[derive(Copy, Clone)]
pub struct ColorBytes <T> {
    r: u8,
    g: u8,
    b: u8,
    cb_type: PhantomData<T>,
}

impl ColorBytes <(u8, u8, u8)> {
    pub fn new(r: u8, g: u8, b: u8) -> ColorBytes <(u8, u8, u8)> {
        ColorBytes { r: r, g: g, b: b, cb_type: PhantomData}   
    }
}

impl ColorBytes <Vec<u8>> {
    pub fn new(rgb_vec: Vec<u8>) -> ColorBytes <Vec<u8>> {
        ColorBytes { r: rgb_vec[0], g: rgb_vec[1], b: rgb_vec[2], cb_type: PhantomData}
    }
}

impl ColorBytes <String> {
    pub fn new(hex_str: String) -> ColorBytes <Vec<u8>> {
        ColorBytes::<Vec<u8>>::new(hex::decode(hex_str).unwrap())
    }
}


pub enum Color <T>{
    Hex(ColorBytes<T>),
    RGB(ColorBytes<T>),
}

impl fmt::Display for Color <T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::<T>::Hex(cb) => fmt::Display::fmt(&hex::encode_upper(vec![cb.r, cb.g, cb.b]), f),
            Color::<T>::RGB(cb) => f.debug_struct("RGB").field("r", &cb.r).field("g", &cb.g).field("b", &cb.b).finish(),
        }
    }
}

fn main() {
    let green: ColorBytes<(u8, u8, u8)> = ColorBytes::<(u8, u8, u8)>::new(34u8, 139u8, 34u8);
    println!("{}", Color::<(u8, u8, u8)>::RGB(green));
    println!("{}", Color::Hex(green));

    println!("{:?}", String::from("228B22").chars());
    println!("{:?}", hex::decode(String::from("228B22")));

}
