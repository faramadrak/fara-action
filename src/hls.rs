use std::fs;
use std::env;
use std::path::PathBuf;

use native_dialog::FileDialog;

use crate::Site;
use crate::console::Console;

pub struct HLS{
    
}

impl HLS{

    fn root_path()->PathBuf{
       let mut path = env::current_exe().expect("Not access in hls");
       path.pop();

       path
    }

    fn site_dir_path()->PathBuf{
        let mut path = HLS::root_path();
        let mut current = Site::get_current();
        path = path.join(current.unwrap().title);

        path
    }

    

    pub fn init(){
        let mut path = HLS::root_path();
        let mut current = Site::get_current();

        if current.is_some(){

            path = path.join(current.unwrap().title);

            fs::create_dir(&path);
    
            let video = fs::create_dir(&path.join("videos"));
            let _ = fs::create_dir(&path.join("hls_videos"));
            let _ = fs::create_dir(&path.join("config"));

            match video {
                Ok(())=>{

                }
                Err(err)=>{
                    println!("{}", path.display());
                    println!("{}", err)
                }
            }
        }
    }

    
    pub fn start_all_video(){

    }


    pub fn select_key_file(){
        let to = HLS::site_dir_path();
        let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("keyinfo", &["keyinfo"])
        .show_open_single_file()
        .unwrap();

        match path {
            Some(path)=>{
                let c = fs::copy(path, to.join("config/enc.keyinfo"));

                match c {
                    Ok(cc)=>{

                    }
                    Err(err)=>{
                        println!("{}", err);
                    }
                }
            }
            None=>{}
        }
    }


    pub fn create_new_key(){
        Console::clear();

        Console::warning("HLS settings include two enc.key files and enc.keyinfo files, the content of the enc.key file must be accessible through a secure address.");
        println!();
        Console::print("Enter the url to access the contents of the enc.key file \n Url:");
        let get_url = Console::input();

        
    }

}