use std::fs;

use native_dialog::Error;
use native_dialog::FileDialog;
use crate::site::SiteJson;
use crate::site::SiteListJson;

use crate::{hls::HLS, console::Console, site::Site};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
pub struct Watermark{
    pub image:String,
    pub size360p:String,
    pub size480p:String,
    pub size720p:String,


}


impl Watermark{

    pub fn get_config()->Watermark{
        let current = Site::get_current().unwrap();

        let mut watermark_str = current.watermark;


        let watermark_ob= serde_json::from_str(&watermark_str);

        match watermark_ob {
            Ok(ob)=>{
                let ob_str:Watermark = ob;

                return ob_str;
            }
            Err(_)=>{
                return Watermark{image:"".to_string(), size360p:"0.020".to_string(), size480p:"0.022".to_string(),size720p:"0.025".to_string()}
            }
        }

    }


    pub fn set_size(size:String){

        // let current = Site::get_current().unwrap();


        Console::clear();
        Console::print("Enter size (sample: 0.021): ");
        let get = Console::input();


        let mut w_config = Watermark::get_config();


        if size == "360"{
            w_config.size360p = get.clone();
        }
        else if size == "480"{
            w_config.size480p = get.clone();
        }
        else if size == "720"{
            w_config.size720p = get.clone();
        }

        let site_current_config = Site::get_current().unwrap();
        let mut sites = Site::get_config();

        for site in sites.sites.iter_mut(){
            if site.title == site_current_config.title{
                site.watermark = serde_json::to_string(&w_config).unwrap();
            }
        }

        Site::save_config(sites);

        Console::clear();
        Console::success(&format!("Updated to : {}", get));
        println!();
    }

    pub fn remove_image(){

        Console::clear();

        Console::print("Can the watermark be removed? (enter 'yes' to delete)? ");
        let answer = Console::input();

        if answer != "yes"{
            return;
        }

        let mut w_config = Watermark::get_config();
        w_config.image = "".to_string();


        let site_current_config = Site::get_current().unwrap();
        let mut sites = Site::get_config();

        for site in sites.sites.iter_mut(){
            if site.title == site_current_config.title{
                site.watermark = serde_json::to_string(&w_config).unwrap();
            }
        }

        Site::save_config(sites);

        Console::clear();
        Console::success("Image removed");
        println!();
    }

    pub fn add_image(){
        let site_path = HLS::site_dir_path();
        
        let files_path = FileDialog::new()
            .set_location("~/Desktop")
            .add_filter("Select image", &["png", "jpg"])
            .show_open_single_file()
            .unwrap();


        match files_path {
            Some(file)=>{
                let file_name = &file.file_name().unwrap().to_str().unwrap();
                let _ = fs::copy(&file, &site_path.join("config").join(&file_name));


                let mut w_config = Watermark::get_config();
                w_config.image = file_name.to_string().clone();


                let site_current_config = Site::get_current().unwrap();
                let mut sites = Site::get_config();

                for site in sites.sites.iter_mut(){
                    if site.title == site_current_config.title{
                        site.watermark = serde_json::to_string(&w_config).unwrap();
                    }
                }

                Site::save_config(sites);

                Console::clear();
                Console::success("Image added");
                println!();
            }
            None=>{
                Console::clear();
            }
        }
    }
    
}