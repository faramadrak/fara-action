use crossterm::style::Stylize;

use crate::console::Console;
use crate::site;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs;
use std::sync::{Once};



static mut DATA: String = String::new();
static ONCE: Once = Once::new();

#[derive(Serialize, Deserialize)]
pub struct SiteJson {
   pub title: String,
   pub watermark: String,
   pub hls: String,
}

#[derive(Serialize, Deserialize)]
pub struct SiteListJson {
    pub sites: Vec<SiteJson>,
    current: String,
}

pub struct Site {
    pub sites: SiteListJson,
}

impl Site {


   pub fn get_config_file_path() -> PathBuf {
        let mut path = env::current_exe().expect("Unable to get current executable path");
        path.set_file_name("sites.conf");

        path
    }

    fn get_config_string() -> String {

        let config_path = Site::get_config_file_path();

        if config_path.exists() == false{
            Site::save_config(SiteListJson { sites: vec![], current: "".to_string() })
        }

        // open config file
        let mut config_file = File::open(&config_path).expect("Can not open config file");

        // make config_str variable
        let mut config_str = String::new();

        // set config text to config_str
        config_file.read_to_string(&mut config_str).unwrap();

        config_str
    }


    pub fn set_current(title:String){
        let mut config = Site::get_config();
        config.current = title;

        Site::save_config(config);
    }

    pub fn get_config() -> SiteListJson {
        let config_str = Site::get_config_string();
        
        serde_json::from_str(&config_str)
            .expect(&format!("Error in parse json: {}", config_str).to_string())
    }

    pub fn save_config(sites: SiteListJson) {
        let result = serde_json::to_string(&sites);
        let config_file_path = Site::get_config_file_path();

        match result {
            Ok(text) => {
                // write string to config file
                let mut new_file = File::create(config_file_path).expect("Unable to create config file");
                new_file
                    .write_all(text.as_bytes())
                    .expect("Unable to write to file");
            }
            Err(_) => {
                println!("The configuration file could not be saved");
            }
        }
    }

    pub fn get_current()->Option<SiteJson>{

        let config = Site::get_config();

        let current_site_title = config.current;

        for site in config.sites{
            if(site.title == current_site_title){
                return Some(site);
            }
        }

        None
    }

    pub fn count()->usize{
        let config = Site::get_config();
        config.sites.len()
    }

    fn save_new_site(title: String) {
        // make site object
        let site: SiteJson = SiteJson {
            title: title,
            watermark: "{}".to_string(),
            hls: "{}".to_string(),
        };

        // parse json
        let mut config: SiteListJson = Site::get_config();

        // add new site config to sites
        config.sites.push(site);

        // convert config object to string
        Site::save_config(config);
    }


    pub fn process_save_new() {
        Console::clear();

        println!("0) Cancel");
        println!("You are creating a new site, enter the requested information");

        Console::print_color("Enter site name:".blue());
        let title = Console::input();

        if title != "0" {
            Site::save_new_site(title.clone());
            Console::clear();
            Console::success(format!("The site '{}' has been added", title).as_str());
        } else {
            Console::clear();
        }
    }

    pub fn show_site_list_and_select(){
        let mut text = String::new();
        let mut index = 0;


        for site in Site::get_config().sites{
            index += 1;
            text += &format!("{}) {}\n",index, site.title).to_string();
        }

        Console::clear();
        println!("{}", text);
        Console::print("Enter site number:");

        let site_number = Console::input();

        let (status,number) = Console::to_number(site_number);
        
        if status == false{
            Console::clear();
            Console::error("Site number not currect.");
            Site::show_site_list_and_select();
            return;
        }

        let config = Site::get_config();

        if config.sites.get(number-1).is_some() ==false {
            Console::clear();
            Console::error("Site number not currect.");
            Site::show_site_list_and_select();
            return;
        }

        Site::set_current(config.sites[number-1].title.clone());
        

    }


    pub fn set_first_to_currect(){
        let config = Site::get_config();
        Site::set_current(config.sites[0].title.clone());
    }


}
