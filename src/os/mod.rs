pub mod windows;

pub enum MenuItem<'a> {
    Divider,
    Action {
        title: &'a str,
        action: Box<dyn Fn() -> ()>,
    },
    SubMenu(Vec<MenuItem<'a>>),
}

pub struct Menu(windows::Menu);

pub fn new_menu(items: &[MenuItem]) -> Menu {
    Menu(windows::new_menu(items))
}
