
mod bing;

mod data;
use data::*;

extern crate dirs;
use self::dirs::home_dir;

use std::path::PathBuf;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate clap;
use clap::{Arg, App, AppSettings};

extern crate failure;

lazy_static! {
    pub static ref BingPath: PathBuf = {
        
        [home_dir().unwrap(),PathBuf::from("Pictures/Bing")]
                            .iter().collect()

    };
}

fn main() -> Result<(),failure::Error> {
    check_dir()?;
    check_data()?;

    let matches = App::new("Bing")
                        .version("1.0")
                        .about("fetch Bing wallpaper of the day")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .arg(Arg::with_name("previous")
                            .short("p")
                            .long("previous")
                            .help("get the nth previous image\n example: bing-rs -p ")
                            .min_values(0)
                            .conflicts_with_all(&["random","next","today"]))
                        .arg(Arg::with_name("next")
                            .short("n")
                            .long("next")
                            .help("get the nth next image (works on local mode only)\n example: bing-rs -n 3")
                            .min_values(0)
                            .requires("local")
                            .conflicts_with_all(&["random","previous","today"]))
                        .arg(Arg::with_name("random")
                            .short("r")
                            .long("random")
                            .help("fetch a random image from Bing images or local wallpapers\n example: bing -l -r")
                            .conflicts_with_all(&["previous","next","today"])
                            //min_values(0) just to make random appear as option instead of a flag
                            .min_values(0))
                        .arg(Arg::with_name("today")
                            .short("t")
                            .long("today")
                            .help("fetch today's image from bing (needs network)")
                            .conflicts_with_all(&["previous","next","random","local"]))
                        .arg(Arg::with_name("local")
                            .short("l")
                            .long("local")
                            .help("fetch image from saved wallpapers\n  you must have at least one saved local Bing image "))
                        .get_matches();

                        

    if matches.is_present("today") {
        get_today()?;
        return Ok(());
    };
    if matches.is_present("previous") {
        get_previous(&matches)?;
        return Ok(());
    };
    if matches.is_present("next") {
        get_next(&matches)?;
        return Ok(());
    };
    if matches.is_present("random") {
        get_random(&matches)?;
        return Ok(());
    };

       

    Ok(())    

}
