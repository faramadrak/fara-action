use crossterm::style::{self, Stylize};

mod console;
mod hls;
mod menu;
mod menu_list;
mod site;
mod watermark;

use console::Console;
use hls::HLS;
use menu::Menu;
use menu_list::MenuItem;
use menu_list::MenuList;
use site::Site;
use watermark::Watermark;

fn main() {
    let mut menu = Menu::new();

    let mut menu_list = MenuList::new("main", "Site manage");
    menu_list.add("HLS", false, "", "hls");
    menu_list.add("Settings", false, "", "site_settings");
    menu_list.add("Sites", false, "", "sites");
    menu_list.add("Exit", true, "close_app", "sites");

    menu.add(menu_list);

    let mut menu_list = MenuList::new("hls", "HLS Action");
    menu_list.add("Start Connvert", true, "hls_all", "");
    menu_list.add("Show List", true, "show_all_video_list", "");
    menu_list.add("Add Video", true, "import_new_video", "site_settings");
    menu_list.add("History", true, "how_hls_history", "");
    menu_list.add(
        "Remove all original videos",
        true,
        "remove_all_org_videos",
        "",
    );
    menu_list.add("Remove all hls videos", true, "remove_all_hls_videos", "");
    menu_list.add("Back", true, "close_app", "sites");

    menu.add(menu_list);

    let mut menu_list = MenuList::new("site_settings", "Site settings");
    // menu_list.add("FTP",false,"","");
    menu_list.add("HLS", false, "", "hls_settings");
    menu_list.add("Watermark", false, "", "watermark_settings");
    menu_list.add("Back", false, "", "");

    menu.add(menu_list);

    let mut menu_list = MenuList::new("watermark_settings", "Site settings");
    menu_list.add("Add Image", true, "add_watermark_image", "");
    menu_list.add(
        "360p size (d: 0.020)",
        true,
        "set_size_w_360",
        "hls_settings",
    );
    menu_list.add("480p size (d: 0.022)", true, "set_size_w_480", "");
    menu_list.add("720p size (d: 0.025)", true, "set_size_w_720", "");
    menu_list.add("1080p size (d: 0.028)", true, "set_size_w_1080", "");
    menu_list.add("Remove Watermark", true, "remove_watermark", "");
    menu_list.add("Back", false, "", "");

    menu.add(menu_list);

    let mut menu_list = MenuList::new("sites", "Add a site or show list");
    menu_list.add("Add site", true, "add_site", "");
    menu_list.add("List", true, "select_a_site", "");
    menu_list.add("Back", false, "", "");

    menu.add(menu_list);

    let mut menu_list = MenuList::new("hls_settings", "Site HLS settings");
    menu_list.add(
        "Select Config Files (enc.keyinfo, enc.key)",
        true,
        "select_hls_key_file",
        "",
    );
    menu_list.add("Create New keyInfo", true, "create_new_hls_key", "");
    menu_list.add("Custom m3u8 path", true, "set_custom_path_hls", "");
    menu_list.add("Resolutions", true, "set_resolutions", "");
    menu_list.add("Back", false, "", "");

    menu.add(menu_list);

    HLS::init();
    // return;

    let current_site = Site::get_current();
    let count = Site::count();

    match current_site {
        Some(site) => {
            menu.show_current_menu(true);
        }
        None => {
            if count == 0 {
                Site::process_save_new();
                Site::set_first_to_currect();
                menu.show_current_menu(true);
            } else if count == 1 {
                Site::set_first_to_currect();
                menu.show_current_menu(true);
            } else {
                Site::show_site_list_and_select();
                menu.show_current_menu(true);
            }
        }
    }
}
