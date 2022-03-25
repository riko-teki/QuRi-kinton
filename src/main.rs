struct QR {
    version: u8,
    pixel_size: u8,
}

enum ErrCorrectionLevel {
    L,
    M,
    Q,
    H,
}

impl QR {
    fn new(version: u8) -> Self {
        match version {
            v @ 2_u8..=40_u8 => QR { 
                version: v,
                pixel_size: version * 4 + 17
            },
            _ => unreachable!(),
        } 
    }
}

fn main() {
    let qr = QR::new(40);
    println!("{}",qr.pixel_size);
}
