extern crate image;
extern crate clap;

use clap::{Arg, App};
use std::error::Error;
use std::fs;

// list of all dimensions
const DIMENSIONS: [u32; 13] = [20, 29, 40, 58, 60, 76, 87, 80, 120, 152, 167, 180, 1024];

fn main() -> Result<(), Box<dyn Error>> {

    // taking params
    let matches = App::new("square")
        .version("0.1.0")
        .author("Benoit PASQUIER <b.pasquier69@gmail.com>")
        .about("An iOS app icon resizer written in rust")
        .arg(Arg::with_name("image")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("Image to resize"))
        .get_matches();

    let url = matches.value_of("image").unwrap();
    println!("{}", url);

    // Do the job
    let img = image::open(&url)?;
    
    fs::create_dir("./output/")?;
    for dimension in &DIMENSIONS {
        let thumbnail = img.thumbnail_exact(*dimension, *dimension);
        let path = format!("./output/{}.png", dimension);
        thumbnail.save(path).unwrap();
    }
    
    // All was ok
    Ok(())
}