use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::process::Command;

use crate::{Result, BINGPATH};

#[derive(Deserialize)]
pub struct Bing {
    images: [Image; 1],
}
#[derive(Deserialize, Clone)]
struct Image {
    url: String,
    hsh: String,
    copyright: String,
    startdate: String,
}

impl Bing {
    pub fn image_request(idx: usize) -> Result<Bing> {
        let url = &format!(
            "http://www.bing.com/HPImageArchive.aspx?format=js&idx={}&n=1",
            idx
        );
        Ok(ureq::get(url).call().into_json_deserialize()?)
    }

    fn fallback_image(&self) -> String {
        let bing_url = "https://www.bing.com";
        format!("{}{}", bing_url, self.images[0].clone().url)
    }

    fn hd_image(&self) -> String {
        let bing_url = "https://www.bing.com/hpwp/";
        format!("{}{}", bing_url, self.images[0].clone().hsh)
    }

    pub fn image_name(&self) -> String {
        let img = self.images[0].clone().url;
        let mut img = img.replace("/az/hprichbg/rb/", "");
        let cut_idx = img.find('_').unwrap();

        let _ = img.split_off(cut_idx);

        //remove th?id=
        if img.contains("th?id=OHR.") {
            img = img.split("/th?id=OHR.").nth(1).unwrap().to_string();
        }

        format!("{}.jpg", img)
    }
    pub fn date(&self) -> String {
        let date = self.images[0].clone().startdate;
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..])
    }

    pub fn image_data(&self) -> impl std::io::Read + Send {
        let resp = ureq::get(&self.hd_image()).call();
        if resp.ok() {
            resp.into_reader()
        } else {
            ureq::get(&self.fallback_image()).call().into_reader()
        }
    }

    pub fn image_description(&self) -> String {
        self.images[0].clone().copyright
    }

    pub fn cache(&self) {
        let mut data_path = BINGPATH.clone();
        data_path.push("data");
        let mut data = OpenOptions::new()
            .append(true)
            .read(true)
            .open(data_path.as_path())
            .expect("database not found");
        if read_file(&mut data).find(&self.image_name()).is_some() {
            return;
        }
        if let Err(e) = writeln!(data, "{} {}", self.image_name(), self.date()) {
            panic!("Error while writing to database: {}", e);
        }
    }
    pub fn remove_entry(img: &str) {
        let mut data_path = BINGPATH.clone();
        data_path.push("data");
        let mut data_file = File::open(&data_path).expect("Error while reading database");
        let data = read_file(&mut data_file);
        let mut data: String = data
            .lines()
            .filter(|line| !line.contains(img))
            .map(|line| format!("{}\n", line))
            .collect();
        // remove last \n
        data.pop().unwrap();
        let mut data_file =
            File::create(&data_path).expect("Error while removing entry from database");
        writeln!(data_file, "{}", data).unwrap();
    }
}

//helper functions

pub fn read_file(f: &mut File) -> String {
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Error while reading data to buffer");

    buffer
}

pub fn image_dir(img_name: &str) -> String {
    format!("{}/{}", BINGPATH.to_str().unwrap(), img_name)
}

pub fn current_image() -> String {
    let get_img = Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.background", "picture-uri"])
        .output()
        .expect("Getting current wallpaper faield");
    let img_path = String::from_utf8_lossy(&get_img.stdout).to_string();

    let img_name: Vec<&str> = img_path.split("Bing/").collect();
    //If the current wallpaper isn't from the Bing folder:
    //  return  an empty string
    if img_name.len() < 2 {
        return "".to_string();
    }
    img_name[1][..img_name[1].len() - 2].to_string()
}
