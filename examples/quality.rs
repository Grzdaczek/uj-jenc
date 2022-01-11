use std::fs;
use std::io::{Cursor, Result};
use pixlib::codec::*;
use pixlib::color::Lab8;
use pixlib::image::Image;

fn main() -> Result<()> {
    let file = fs::read("./examples/in_192x192.ppm")?;
    let img: Image<Lab8> = ppm::decode(Cursor::new(file)).into();
    
    for q in 0..8 {
        let mut rcr_data = Vec::new();
        rcr::encode(Cursor::new(&mut rcr_data), rcr::Settings::quality(q), &img)?;
        
        let img = rcr::decode(Cursor::new(&rcr_data))?;
        
        let mut ppm_data = Vec::new();
        ppm::encode(Cursor::new(&mut ppm_data), &img.into());
        
        fs::write(format!("./examples/tmp/out_q{}.ppm", q), ppm_data)?;
        fs::write(format!("./examples/tmp/out_q{}.rcr", q), rcr_data)?;
    }

    Ok(())
}