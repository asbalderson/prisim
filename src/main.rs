extern crate clap;
extern crate colored;

use clap::{value_t_or_exit, App, Arg};
use colored::*;
use itertools::Itertools;
use std::cmp;
use std::fmt;

#[derive(Copy, Clone)]
pub struct ColorBytes {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorBytes {
    fn new(r: u8, g: u8, b: u8) -> ColorBytes {
        ColorBytes { r: r, g: g, b: b }
    }

    pub fn complement(self) -> ColorBytes {
        ColorBytes { r: u8::MAX - self.r, g: u8::MAX - self.g, b: u8::MAX - self.b }
    }

    pub fn from_tuple(rgb_tuple: (u8, u8, u8)) -> ColorBytes {
        ColorBytes::new(rgb_tuple.0, rgb_tuple.1, rgb_tuple.2)
    }

    pub fn from_vec(vec: Vec<u8>) -> ColorBytes {
        match vec.len() {
            3 => ColorBytes::from_tuple(vec.into_iter().collect_tuple().unwrap()),
            4 => {
                let tuple: (u8, u8, u8, u8) = vec.into_iter().collect_tuple().unwrap();
                ColorBytes::from_cmyk(tuple.0, tuple.1, tuple.2, tuple.3)
            }
            _ => panic!("invalid vector length for color: {:?}", vec),
        }
    }

    pub fn from_hex(hex_string: String) -> ColorBytes {
        let rgb_tuple: (u8, u8, u8) = hex::decode(hex_string)
            .unwrap()
            .into_iter()
            .collect_tuple()
            .unwrap();
        ColorBytes::from_tuple(rgb_tuple)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> ColorBytes {
        ColorBytes::new(r, g, b)
    }

    pub fn from_cmyk(c: u8, m: u8, y: u8, k: u8) -> ColorBytes {
        let one_minus_k: f32 = 1f32 - (k as f32 / 100f32);
        let r = (u8::MAX as f32 * (1f32 - (c as f32 / 100f32)) * one_minus_k).round() as u8;
        let g = (u8::MAX as f32 * (1f32 - (m as f32 / 100f32)) * one_minus_k).round() as u8;
        let b = (u8::MAX as f32 * (1f32 - (y as f32 / 100f32)) * one_minus_k).round() as u8;
        ColorBytes::new(r, g, b)
    }

    fn max(self) -> u8 {
        cmp::max(self.r, cmp::max(self.g, self.b))
    }

    fn min(self) -> u8 {
        cmp::min(self.r, cmp::min(self.g, self.b))
    }

    fn delta(self) -> f32 {
        (self.max() - self.min()) as f32 / u8::MAX as f32
    }

    fn h(self) -> u8 {
        let r_prime = self.r as f32 / u8::MAX as f32;
        let g_prime = self.g as f32 / u8::MAX as f32;
        let b_prime = self.b as f32 / u8::MAX as f32;

        if self.max() == self.r {
            60 * ((g_prime - b_prime) / self.delta()) as u8 % 6 as u8
        } else if self.max() == self.g {
            60 * ((b_prime - r_prime) / self.delta() + 2 as f32) as u8
        } else {
            60 * ((r_prime - g_prime) / self.delta() + 4 as f32) as u8
        }
    }

    fn s(self) -> f32 {
        if self.max() == self.min() {
            0f32
        } else {
            self.delta() / (1f32 - (2f32 * self.l() - 1f32).abs())
        }
    }

    fn l(self) -> f32 {
        ((self.max() as u16 + self.min() as u16) as f32 / u8::MAX as f32) / 2f32
    }

    pub fn as_rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub fn as_hex(self) -> String {
        hex::encode_upper(vec![self.r, self.g, self.b])
    }

    pub fn as_hsl(self) -> (u8, f32, f32) {
        (
            self.h(),
            round_ten(self.s() * 100f32),
            round_ten(self.l() * 100f32),
        )
    }

    pub fn as_hsv(self) -> (u8, f32, f32) {
        (
            self.h(),
            round_ten(self.s() * 100f32),
            round_ten((self.max() as f32 / u8::MAX as f32) * 100f32),
        )
    }

    pub fn as_cmyk(self) -> (u8, u8, u8, u8) {
        let k: f32 = 1f32 - (self.max() as f32 / u8::MAX as f32);
        let c: f32 = (1f32 - (self.r as f32 / u8::MAX as f32) - k) / (1f32 - k);
        let m: f32 = (1f32 - (self.g as f32 / u8::MAX as f32) - k) / (1f32 - k);
        let y: f32 = (1f32 - (self.b as f32 / u8::MAX as f32) - k) / (1f32 - k);

        (
            (c * 100f32).round() as u8,
            (m * 100f32).round() as u8,
            (y * 100f32).round() as u8,
            (k * 100f32).round() as u8,
        )
    }
}

fn round_ten(val: f32) -> f32 {
    (val * 10f32).round() / 10f32
}

pub enum Color {
    Hex(ColorBytes),
    RGB(ColorBytes),
    HSL(ColorBytes),
    HSV(ColorBytes),
    CMYK(ColorBytes),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Hex(cb) => write!(f, "Hex: 0x{}", &cb.as_hex()),
            Color::RGB(cb) => write!(f, "RGB: {:?}", &cb.as_rgb()),
            Color::HSL(cb) => write!(f, "HSL: {:?}", &cb.as_hsl()),
            Color::HSV(cb) => write!(f, "HSV: {:?}", &cb.as_hsv()),
            Color::CMYK(cb) => write!(f, "CMYK: {:?}", &cb.as_cmyk()),
        }
    }
}

fn main() {
    let args = App::new("color_conversion")
        .version("v1.0-alpha")
        .author("Alex Balderson")
        .about("Convert a color from hex to RGB, HSL, HSV, and CMYK")
        .arg(
            Arg::with_name("hex")
                .long("hex")
                .value_name("hex")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rgb")
                .long("rgb")
                .value_name("rgb")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cmyk")
                .long("cmyk")
                .value_name("cmyk")
                .takes_value(true),
        ).arg(
            Arg::with_name("color")
                .long("color")
                .value_name("color")
                .takes_value(false)
        ).arg(
            Arg::with_name("complement")
                .long("complement")
                .value_name("complement")
                .takes_value(false))
        .get_matches();

    let mut color;

    if args.is_present("hex") {
        let hex_str = value_t_or_exit!(args, "hex", String);
        color = ColorBytes::from_hex(hex_str);
    } else if args.is_present("rgb") {
        let vec: Vec<u8> = value_t_or_exit!(args, "rgb", String)
            .split(",")
            .map(|v| {
                v.to_string()
                    .parse::<u8>()
                    .expect("Could not parse into u8")
            })
            .collect::<Vec<u8>>();
        color = ColorBytes::from_vec(vec);
    } else if args.is_present("cmyk") {
        let vec: Vec<u8> = value_t_or_exit!(args, "cmyk", String)
            .split(",")
            .map(|v| {
                v.to_string()
                    .parse::<u8>()
                    .expect("Could not parse into u8")
            })
            .collect::<Vec<u8>>();
        color = ColorBytes::from_vec(vec);
    } else {
        panic!("Need argument `hex`, `rgb`, or `cmyk`");
    }

    if args.is_present("complement") {
        color = color.complement();
    }

    let mut hex: ColoredString = format!("{}", color.as_hex()).normal();
    let mut rgb: ColoredString = format!("{:?}", color.as_rgb()).normal();
    let mut hsl: ColoredString = format!("{:?}", color.as_hsl()).normal();
    let mut hsv: ColoredString = format!("{:?}", color.as_hsv()).normal();
    let mut cmyk: ColoredString = format!("{:?}", color.as_cmyk()).normal();

    if args.is_present("color") {
        hex = hex.truecolor(color.r, color.g, color.b);
        rgb = rgb.truecolor(color.r, color.g, color.b);
        hsl = hsl.truecolor(color.r, color.g, color.b);
        hsv = hsv.truecolor(color.r, color.g, color.b);
        cmyk = cmyk.truecolor(color.r, color.g, color.b);
    }

    println!("Hex: {}", hex);
    println!("RGB: {}", rgb);
    println!("HSL: {}", hsl);
    println!("HSV: {}", hsv);
    println!("CYMK: {}", cmyk);
}
