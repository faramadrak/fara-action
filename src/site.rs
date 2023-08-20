use crossterm::style::Stylize;

use crate::console::Console;
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct SiteJson{
    title:String,

    watermark:bool,
    watermark_icon:String,
    watermark_size:i16,

}

#[derive(Serialize, Deserialize)]
struct SiteListJson{
    sites:Vec<SiteJson>
}

pub struct Site{
    // title: String,

}

impl Site{




    fn save_new_site(title:String){
        let site: SiteJson = SiteJson{title:title, watermark:false, watermark_icon:"".to_string(), watermark_size:80};
        let current_exe_path = env::current_exe().expect("Unable to get current executable path");
        
        let mut new_file_path = current_exe_path.clone();
        new_file_path.set_file_name("sites.conf");
        let mut config_file = File::open(&new_file_path).expect("Can not open config file");

        let mut config_str = String::new();
        config_file.read_to_string(&mut config_str).unwrap();
        
        let mut config:SiteListJson = serde_json::from_str(&config_str).expect(&format!("Error in parse json: {}", config_str).to_string());

        config.sites.push(site);
        
        let result = serde_json::to_string(&config);

        match result {
             Ok(text)=>{

                // تغییر نام فایل و ایجاد فایل متنی جدید
                
                let mut new_file = File::create(new_file_path).expect("Unable to create file");
                
                new_file.write_all(text.as_bytes()).expect("Unable to write to file");
             }
             Err(_)=>{
                println!("Probbllleeeemmm")
             }
        }

    }

    pub fn process_save_new(){
        Console::clear();

        println!("0) Cancel");
        println!("You are creating a new site, enter the requested information");

        Console::print_color("Enter site name:".blue());
        let title = Console::input();

        if title != "0"{            
            Site::save_new_site(title.clone());
            Console::clear();
            Console::success(format!("The site '{}' has been added", title).as_str());
        }
        else{
            Console::clear();
        }

    }
}