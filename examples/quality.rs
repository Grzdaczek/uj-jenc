use std::fs;
use std::io::Result;
use uj_jenc::codec::*;
use uj_jenc::color::Lab8;
use uj_jenc::image::Image;

fn main() -> Result<()> {
    let file = fs::read("./examples/in_192x192.ppm")?;
    let img: Image<Lab8> = ppm::decode(&mut &file[..]).into();
    
    for q in 0..8 {
        let mut rcr_data = Vec::new();
        let mut ppm_data = Vec::new();
        
        rcr::encode(&mut rcr_data, rcr::Settings::quality(q), &img)?;
        let img = rcr::decode(&mut &rcr_data[..])?;
        ppm::encode(&mut ppm_data, &img.into());
        
        fs::write(format!("./examples/tmp/out_q{}.ppm", q), ppm_data)?;
        fs::write(format!("./examples/tmp/out_q{}.rcr", q), rcr_data)?;
    }

    Ok(())
}