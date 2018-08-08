extern crate clap;
use clap::ArgMatches;

extern crate failure;

extern crate rand;
use self::rand::prelude::*;

extern crate chrono;
use self::chrono::prelude::*;
use self::chrono::Utc;

use bing::*;
use BingPath;

use std::fs::{create_dir, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn delete(matches: &ArgMatches) -> Result<(), failure::Error> {
    let current_image = current_image();
    Bing::remove_entry(&current_image);
    get_random(matches)?;
    Command::new("rm")
        .arg(image_dir(&current_image))
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to rm current image (Maybe its not in Bing folder?)");
    Ok(())
}

pub fn recall(matches: &ArgMatches) -> Result<(), failure::Error> {
    let mut last_path = BingPath.clone();
    last_path.push("last");
    match File::open(last_path) {
        Ok(mut last) => set_wallpaper(&read_file(&mut last).trim(), None),
        Err(_) => get_random(matches)?,
    }
    Ok(())
}

//get methods
pub fn get_previous(matches: &ArgMatches) -> Result<(), failure::Error> {
    let n: usize = (matches.value_of("previous").unwrap_or("1")).parse()?;

    if matches.is_present("local") {
        let table = get_table()?;
        let current_image = current_image();

        //If the current wallpaper isn't from the Bing folder:
        //  return  a random one
        if current_image == "" {
            get_random(matches)?;
            return Ok(());
        }

        let current_idx = find_index(&table, &current_image);
        let previous_index = previous_index(current_idx, n, table.len());

        let (requested_image, _) = &table[previous_index];

        set_wallpaper(requested_image, None);

        Ok(())
    } else {
        get(n)?;

        Ok(())
    }
}

pub fn get_next(matches: &ArgMatches) -> Result<(), failure::Error> {
    let n: usize = (matches.value_of("next").unwrap_or("1")).parse()?;
    //printprintln!("{}",n );
    if matches.is_present("local") {
        let table = get_table()?;
        let current_image = current_image();

        //If the current wallpaper isn't from the Bing folder:
        //  return  a random one
        if current_image == "" {
            get_random(matches)?;
            return Ok(());
        }

        let current_idx = find_index(&table, &current_image);
        let next_index = next_index(current_idx, n, table.len());

        let (requested_image, _) = &table[next_index];

        set_wallpaper(requested_image, None);

        Ok(())
    } else {
        get(n)?;

        Ok(())
    }
}

pub fn get_today() -> Result<(), failure::Error> {
    get(0)?;

    Ok(())
}

pub fn get_random(matches: &ArgMatches) -> Result<(), failure::Error> {
    if matches.is_present("local") {
        let table = get_table()?;
        let (img_name, _) = thread_rng().choose(&table).unwrap();
        set_wallpaper(img_name, None);
        Ok(())
    } else {
        let n: usize = thread_rng().gen_range(0, 8);

        get(n)?;
        //println!("{}", n);
        Ok(())
    }
}

fn get(n: usize) -> Result<(), failure::Error> {
    let img = Bing::image_request(n)?;
    img.cache();
    let img_desc = img.image_description();

    let img_name = img.image_name();
    let img_dir = &image_dir(&img_name);
    let img_dir = Path::new(img_dir);

    if img_dir.exists() {
        set_wallpaper(&img_name, None);
        return Ok(());
    }

    let mut img_data = img.image_data()?;

    let mut img_file = File::create(&img_dir)?;
    //println!("{}",img.image_name());
    io::copy(&mut img_data, &mut img_file)?;
    set_wallpaper(&img_name, Some(&img_desc));

    Ok(())
}

fn get_table() -> Result<Vec<(String, Date<Utc>)>, failure::Error> {
    let mut table: Vec<(String, String)> = Vec::new();
    let mut data_path = BingPath.clone();
    data_path.push("data");
    let mut data_file = File::open(data_path).expect("Error while reading database");
    let data = read_file(&mut data_file);

    for line in data.lines() {
        let l: Vec<&str> = line.split(' ').collect();
        table.push((String::from(l[0]), String::from(l[1])));
    }

    if table.is_empty() {
        panic!("Bing folder is empty\nYou have not fetched any Image from bing yet");
    }

    let mut table: Vec<(_, _)> = table
        .into_iter()
        .map(|(n, d)| {
            let dd = Date::<Utc>::from_utc(NaiveDate::parse_from_str(&d, "%Y-%m-%d").unwrap(), Utc);
            (n, dd)
        }).collect();
    table.sort();
    //println!("{:?}",&table);
    Ok(table)
}

fn notify(img_desc: Option<&str>) {
    let img_desc = match img_desc {
        Some(desc) => desc,
        None => return (),
    };
    Command::new("notify-send")
        .arg(img_desc)
        .spawn()
        .expect("Failed to invoke notify command");
}

fn set_wallpaper(img_name: &str, img_desc: Option<&str>) {
    let img_dir = image_dir(&img_name);
    let img_dir = Path::new(&img_dir);
    Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.background", "picture-uri"])
        .arg(format!("file://{}", img_dir.to_str().unwrap()))
        .spawn()
        .expect("Failed to set wallpaper");

    //save last wallpaper name
    let mut last_path = BingPath.clone();
    last_path.push("last");

    let mut last = File::create(last_path).expect("error while creating last file");
    if let Err(e) = writeln!(last, "{}", current_image()) {
        panic!("error while writing last wallpaper: {}", e);
    }

    notify(img_desc);
}

//helper methods
fn find_index(table: &[(String, Date<Utc>)], img_name: &str) -> usize {
    for (i, j) in table.iter().enumerate() {
        let (name, _) = j;
        //println!("{} {} {} {}",&name,&img_name,&name.len(), img_name.len() );
        if name == img_name {
            return i;
        }
    }
    thread_rng().gen_range(0, table.len())
}

fn previous_index(current_idx: usize, prev_arg: usize, table_len: usize) -> usize {
    let (table_len, current_idx, prev_arg) =
        (table_len as i32, current_idx as i32, prev_arg as i32);
    match current_idx - prev_arg {
        x if x >= 0 => x as usize,
        x => (x + table_len * (-x / table_len) + table_len) as usize,
    }
}
fn next_index(current_idx: usize, prev_arg: usize, table_len: usize) -> usize {
    match current_idx + prev_arg {
        x if x >= table_len => x - (x / table_len) * table_len,
        x => x,
    }
}

//check if dir exists and check database
pub fn check_dir() -> Result<(), failure::Error> {
    let dir = BingPath.as_path();
    //println!("{:?}",&dir);
    if !dir.exists() {
        create_dir(dir)?;
    }
    Ok(())
}

pub fn check_data() -> Result<(), failure::Error> {
    let mut dir = BingPath.clone();
    dir.push("data");
    //println!("{:?}",&dir);
    if !dir.exists() {
        File::create(dir)?;
    }
    Ok(())
}
