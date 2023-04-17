pub mod windows;

pub enum MenuItem {
    Divider,
    Action {
        title: String,
        action: Box<dyn Fn() -> ()>,
    },
    SubMenu(Vec<MenuItem>),
}

pub struct Menu(windows::Menu);

pub fn new_menu(items: Vec<MenuItem>) -> Menu {
    Menu(windows::new_menu(items))
}
