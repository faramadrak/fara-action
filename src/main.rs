use crossterm::style::{self, Stylize};


mod console;
mod menu_list;
mod menu;
mod site;
mod hls;


use console::Console;
use menu_list::MenuList;
use menu_list::MenuItem;
use menu::Menu;
use site::Site;
use site::SiteJson;
use site::SiteListJson;
use hls::HLS;

fn main() {

    let mut menu = Menu::new();


    let mut menu_list = MenuList::new("main", "Site manage");
    menu_list.add("HLS",false, "","hls");
    menu_list.add("Settings",false,"","site_settings");
    menu_list.add("Sites",false,"","sites");
    menu_list.add("Exit",true,"close_app","sites");

    menu.add(menu_list);


    let mut menu_list = MenuList::new("hls", "HLS Action");
    menu_list.add("Start Connvert",true, "hls_all","");
    menu_list.add("Show List",true, "show_all_video_list","");
    menu_list.add("Add Video",true,"import_new_video","site_settings");
    menu_list.add("Remove all original videos",true,"remove_all_org_videos","");
    menu_list.add("Remove all hls videos",true,"remove_all_hls_videos","");
    menu_list.add("Back",true,"close_app","sites");

    menu.add(menu_list);

    let mut menu_list = MenuList::new("site_settings", "Site settings");
    menu_list.add("FTP",false,"","");
    menu_list.add("HLS",false,"","hls_settings");
    menu_list.add("Watermark",false,"","");
    menu_list.add("Back",false,"","");
    
    menu.add(menu_list);

    let mut menu_list = MenuList::new("sites", "Add a site or show list");
    menu_list.add("Add site",true,"add_site","");
    menu_list.add("List",true,"select_a_site","");
    menu_list.add("Back",false,"","");

    menu.add(menu_list);


    let mut menu_list = MenuList::new("hls_settings", "Site HLS settings");
    menu_list.add("Select Config Files (enc.keyinfo, enc.key)",true,"select_hls_key_file","");
    menu_list.add("Create New keyInfo",true,"create_new_hls_key","");
    // menu_list.add("HLS",false,"","");
    // menu_list.add("Watermark",false,"","");
    menu_list.add("Back",false,"","");
    
    menu.add(menu_list);


    HLS::init();
    // return;


    let current_site = Site::get_current();
    let count = Site::count();

    match current_site{
        Some(site)=>{
            menu.show_current_menu(true);
        }
        None=>{
            if count == 0{
                Site::process_save_new();
                Site::set_first_to_currect();
                menu.show_current_menu(true);
            }
            else if count == 1 {
                Site::set_first_to_currect();
                menu.show_current_menu(true);
            }
            else{
                Site::show_site_list_and_select();
                menu.show_current_menu(true);
            }
        }
    }
    
    

    
}
