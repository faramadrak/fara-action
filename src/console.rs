use crossterm::{cursor, style::{self, Stylize, StyledContent}, terminal, ExecutableCommand, QueueableCommand};
use std::io::{self, Write};


pub struct Console{
    
}



impl Console{

    pub fn clear(){
        io::stdout()
        .execute(terminal::Clear(terminal::ClearType::All)).unwrap()
        .execute(cursor::MoveTo(0, 0)).unwrap();
    }


    pub fn print(text :&str){
        print!("{}", text);
        io::stdout().flush().unwrap();
    }

    pub fn print_color(text :StyledContent<&str>){
        print!("{}", text);
        io::stdout().flush().unwrap();
    }

    pub fn println_color(text :StyledContent<&str>){
        println!("{}", text);
        io::stdout().flush().unwrap();
    }

    pub fn success(text :&str){
        Console::println_color(format!("✅ {}", text).as_str().green());
    }


    pub fn input()->String{
        

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        return input.trim().to_string();
    }

    
}