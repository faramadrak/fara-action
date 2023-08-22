use crossterm::style::Stylize;

use crate::MenuList;
use crate::MenuItem;
use crate::Console;
use crate::site::Site;
use crate::hls::HLS;

pub struct Menu{
    list:Vec<MenuList>,
    current:String,
    history:Vec<String>
}


impl Menu {

    pub fn new()->Menu{
        Menu { list: vec![], current:String::from("main"), history:vec!["main".to_string()] }
    }

    pub fn add(&mut self, menu_list:MenuList){
        self.list.push(menu_list);
    }

    pub fn get_current_menu(&self)->Option<&MenuList>{
        let mut current_menu :Option<&MenuList> = None;

        for menu in self.list.iter(){
            if menu.name == self.current{
                current_menu = Some(menu);
            }
        }

        current_menu
    }

    pub fn get_current_menu_text(&self, current:Option<&MenuList>)->String{

        let mut current_menu = current;


        let mut str = String::new();

        match current_menu {
            Some(menu)=>{
                let mut index = 0;

                for item in &menu.list{
                    
                    if item.name == "Back"{
                        println!("{}","0) Back".yellow());
                        println!();
                    }
                    else{
                        index +=1;
                        str += &format!("🔹 {}) {}\n", index, item.name).to_string();
                    }

                }

            }
            None=>{
                str = "Not Found Menu".to_string();
            }
        }
        str
        // "".to_string();
    }

    pub fn show_current_menu(&mut self, clear:bool){
        if clear{
            Console::clear();
        }

        let current = self.get_current_menu();
        
        let text = self.get_current_menu_text(current);

        match current {
            Some(menu)=>{
                if Site::get_current().is_some(){
                    println!("🌐 {} 🔻", Site::get_current().unwrap().title.blue().bold());
                    println!();
                }
                println!(" 💬 {}\n",menu.description.clone().dark_cyan());
                println!("{}", text);
        
                self.get_input();
            }
            None=>{
                println!("Menu not found");
            }
        }


    }

    pub fn get_input(&mut self){
        Console::print_color("Enter the menu number: ".blue());
        let select = Console::input();

        if select == "0"{
            self.back();
            self.show_current_menu(true);
        }
        else{
            self.check_action(select);
        }
    }

    pub fn check_action(&mut self, select:String)
    {

        let current = self.get_current_menu().expect("Menu not found in 'check_action' in menu.rs");
        
        let number_convert = select.parse::<usize>();
        let mut number_select:usize = 0;
        let mut valid_convert = false;

        match number_convert {
            Ok(number)=>{
                number_select = number-1;
                valid_convert = true;
            }
            Err(_)=>{}
        }


        
        if valid_convert && current.list.get(number_select).is_some() && current.list[number_select].has_action == false{

            let item = &current.list[number_select];
            self.current = item.show_menu.clone();
            self.show_current_menu(true);
        }
        else if current.list.get(number_select).is_some() && current.list[number_select].has_action == true{
            let item = &current.list[number_select];

            if item.action == "add_site" {
                Site::process_save_new();
                self.show_current_menu(false);
            }
            else if item.action == "close_app"{
                Console::println_color("Goodbay 👋".green());
                std::process::exit(0);
            }
            else if item.action == "select_a_site"{
                Site::show_site_list_and_select();
                self.clear_history();
                self.show_current_menu(true);
            }
            else if item.action == "select_hls_key_file"{
                HLS::select_key_file();
            }
            else if item.action == "create_new_hls_key"{
                HLS::create_new_key();
            }
            else if item.action == "hls_all"{
                HLS::start_all_video();
            }
        }
        else{
            Console::clear();
            Console::warning("Menu not found");
            println!();
            self.show_current_menu(false);
        }


    }

    pub fn back(&mut self){

        if self.history.len()> 1 {
            self.history.pop();
        }
        
        self.current = self.history.last().unwrap().to_string();

    }

    pub fn clear_history(&mut self){

        self.history = vec!["main".to_string()];
        
        self.current = self.history.last().unwrap().to_string();

    }

    // pub fn set_to_current_menu(){

    // }
    
}