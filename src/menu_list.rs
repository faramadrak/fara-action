

pub struct MenuItem{
    pub name:String,
    pub show_menu:String,
    pub has_action:bool,
    pub action:String,
}

pub struct MenuList{
    pub list:Vec<MenuItem>,
    pub name:String,
    pub description:String,
}

impl MenuList{

    pub fn new(name:&str, description:&str)->MenuList{
        MenuList{list:vec![], name:name.to_string(), description:description.to_string()}
    }

    pub fn add(&mut self, name:&str,has_action:bool,action:&str, show_menu:&str){
        self.list.push(MenuItem{name:name.to_string(),has_action,action:action.to_string(), show_menu:show_menu.to_string()});
    }
}