use std::ffi::*;

use sdl2::pixels;
use sdl2::video::*;
use windows::core::*;
use windows::w;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    // let window = Window::new();

    //    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    // canvas.set_draw_color(pixels::Color::RGB(255, 0, 0));
    // canvas.fill_rect(None).unwrap();
    // canvas.present();

    // let mut event_pump = sdl.event_pump().unwrap();
    // loop {
    //     for _event in event_pump.poll_iter() {}
    // }
}
