use std::{fmt, error::Error};
use encoding_rs::SHIFT_JIS;
use encoding::all::ISO_8859_1;
use encoding::{Encoding,EncoderTrap};

enum ErrCorrectionLevel {
    L = 7, // default
    M = 15,
    Q = 25,
    H = 30,
}


#[derive(Debug, Clone,)]
enum Mode {
   numeric,
   alphanumeric,
   byte,
   kanji,
   Undifined,
}

struct QR {
    version: u8,
    pixel_size: u8,
    err_correction_level: ErrCorrectionLevel,
    mode: Mode,
}

impl QR {
    fn new(version: u8) -> Result<Self, InvaridVersionError> {
        match version {
            v @ 2_u8..=40_u8 => Ok(QR { 
                version: v,
                pixel_size: version * 4 + 17,
                err_correction_level: ErrCorrectionLevel::L,
                mode: Mode::numeric,
            }),
            _ => Err(InvaridVersionError),
        } 
    }

    fn determine_mode(input: String) -> Mode {
        let digit = input.chars().filter(|c| !c.is_digit(10));
        if digit.count() == 0 { return Mode::numeric; }

        let alphanum = input.chars().filter(|c| !is_qr_alphanum(c));
        if alphanum.count() == 0 { return Mode::alphanumeric; }

        if ISO_8859_1.encode(input.as_str(), EncoderTrap::Strict).is_ok() { return Mode::byte; }

        let (_,_,f) = SHIFT_JIS.encode(input.as_str());
        if !f { return Mode::kanji; }

        return Mode::Undifined;
    }
}

#[derive(Debug, Clone)]
struct InvaridVersionError;

impl fmt::Display for InvaridVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid qr version was specified. Only numbers from 2 to 40 are valid.")
    }
}

impl Error for InvaridVersionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn main() {
    println!("{:?}", QR::determine_mode("SDFSDKJH".to_string()));
    println!("{:?}", QR::determine_mode("092384".to_string()));
    println!("{:?}", QR::determine_mode("ca\n".to_string()));
    println!("{:?}", QR::determine_mode("おはよう御座います".to_string()));
}

fn is_qr_alphanum(c:&char) -> bool {
    match c {
        '0'..='9' => return true,
        'A'..='Z' => return true,
        '$'|'%'|'*'|'+'|'-'|'.'|'/'|' ' => return true,
        _ => return false,
    }
}
