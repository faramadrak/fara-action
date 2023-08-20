use crossterm::style::{self, Stylize};


mod console;
// mod menu;
// mod menu_list;
// mod  menu_item;
mod menu_list;
mod menu;
mod site;
// mod menu;
// mod menu_list;
// mod menu_base;
// mod menu_status;mod

use console::Console;
// use menu::Menu;
// use menu_status::MenuStatus;
use menu_list::MenuList;
use menu_list::MenuItem;
use menu::Menu;


fn main() {

    let mut menu = Menu::new();


    let mut menu_list = MenuList::new("main", "Site manage");
    menu_list.add("HLS",false, "","");
    menu_list.add("Settings",false,"","site_settings");
    menu_list.add("Sites",false,"","sites");
    menu_list.add("Exit",true,"close_app","sites");

    menu.add(menu_list);

    let mut menu_list = MenuList::new("site_settings", "Site settings");
    menu_list.add("FTP",false,"","");
    menu_list.add("HLS",false,"","");
    menu_list.add("Watermark",false,"","");
    menu_list.add("Back",false,"","");
    
    menu.add(menu_list);

    let mut menu_list = MenuList::new("sites", "Add a site or show list");
    menu_list.add("Add site",true,"add_site","");
    menu_list.add("List",false,"","");
    menu_list.add("Back",false,"","");

    menu.add(menu_list);
    
    
    menu.show_current_menu(true);

    
}
