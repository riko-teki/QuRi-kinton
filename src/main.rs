use std::{fmt, error::Error};

struct QR {
    version: u8,
    pixel_size: u8,
    err_correction_level: ErrCorrectionLevel,
}

impl QR {
    fn new(version: u8) -> Result<Self, InvaridVersionError> {
        match version {
            v @ 2_u8..=40_u8 => Ok(QR { 
                version: v,
                pixel_size: version * 4 + 17,
                err_correction_level: ErrCorrectionLevel::L,
            }),
            _ => Err(InvaridVersionError),
        } 
    }
}

enum ErrCorrectionLevel {
    L = 7, // default
    M = 15,
    Q = 25,
    H = 30,
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
    let qr = QR::new(40).unwrap();
    println!("{}",qr.pixel_size);
}
