extern crate reqwest;

use std::fs::{OpenOptions,File};
use std::io::prelude::*;
use std::process::Command;

use ::BingPath;

#[derive(Deserialize)]
pub struct Bing  {
    images: [Image;1] 
}
#[derive(Deserialize, Clone)]
struct Image {
    url: String,
    hsh: String,
    copyright: String,
    startdate: String,
}

impl Bing {

    pub fn image_request (idx:usize) -> reqwest::Result<Bing> {
        let url = &format!("http://www.bing.com/HPImageArchive.aspx?format=js&idx={}&n=1",idx);
        reqwest::get(url)?.json()
    }

    fn fallback_image(&self) -> String {
        let bing_url ="https://www.bing.com";
        format!("{}{}",bing_url,self.images[0].clone().url)
    }

    fn hd_image(&self) -> String {
        let bing_url = "https://www.bing.com/hpwp/";
        format!("{}{}",bing_url,self.images[0].clone().hsh)

    }

    pub fn image_name(&self) -> String {
        let img = self.images[0].clone().url;
        let mut img = img.replace("/az/hprichbg/rb/","");
        let cut_idx = img.find('_').unwrap();
        //println!("{}",cut_idx );
        let _ = img.split_off(cut_idx);
        format!("{}.jpg",img)
    }
    pub fn date(&self) -> String {
        let date = self.images[0].clone().startdate;
        format!("{}-{}-{}",&date[0..4],&date[4..6],&date[6..])
    }

    

    pub fn image_data(&self) -> Result<reqwest::Response,reqwest::Error> {
        let client = reqwest::Client::new();

        match client.get(&self.hd_image()).send() {
            Ok(img) => Ok(img),
            Err(_) => reqwest::get(&self.fallback_image()),
        }
    }

    pub fn image_description(&self) -> String {
        self.images[0].clone().copyright
    }

    pub fn cache(&self) {
        let mut data_path = BingPath.clone();
        data_path.push("data");
        let mut data =   OpenOptions::new()
                        .append(true)
                        .read(true)
                        .open(data_path.as_path())
                        .expect("database not found");
        if read_file(&mut data).find(&self.image_name()).is_some() {
            return ()
        }
        if let Err(e) = writeln!(data,"{} {}",self.image_name(),self.date()) {
            panic!("Error while writing to database: {}",e);
        }
    }

}

//helper functions

pub fn read_file(f:&mut File) -> String {
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("Error while reading data to buffer");

    buffer
}

pub fn image_dir(img_name: &str) -> String {
        format!("{}/{}",BingPath.to_str().unwrap(),img_name)
    }

pub fn current_image() -> String {
    let get_img = Command::new("gsettings")
                    .args(&["get",
                    "org.gnome.desktop.background",
                    "picture-uri"])
                    .output()
                    .expect("Getting current wallpaper faield");
    let  img_path = 
        String::from_utf8_lossy(&get_img.stdout).to_string();
    
    let img_name: Vec<&str> = img_path.split("Bing/")
                    .collect();
    
    img_name[1][..img_name[1].len()-2].to_string()
    
}