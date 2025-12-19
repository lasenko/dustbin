// work in progress
// TODO: autorun, support of other buttons, maybe support of other OS, change icon


#![allow(unused)]

use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent, menu::{Menu, MenuEvent, MenuId, MenuItem}};
use std::env;
use image;
use winit::{application::ApplicationHandler, event, event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder}, platform::windows::EventLoopBuilderExtWindows};
use winit::event::WindowEvent;
use winit::window::{Window, WindowId};

mod utility;

#[derive(Default)]
struct App {
    tray_icon: Option<TrayIcon>,
}

#[derive(Debug)]
enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, _event: WindowEvent) {}

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        println!("{event:?}");
        match event {
            UserEvent::MenuEvent(event) => {
                if event.id == MenuId::new("1001") {
                    std::process::exit(0);
                }else if event.id == MenuId::new("1000") {
                    std::process::Command::new("Explorer.exe")
                        .arg("shell:RecycleBinFolder")
                        .status()
                        .expect("Failed to open Windows Explorer at path");
                }else if event.id == MenuId::new("1002") {
                    utility::autorun();
                }/*else if event.id == MenuId::new("1000") { // if i do it need to change the id of other menu buttons
                    
                }*/
            },
            UserEvent::TrayIconEvent(tray_icon_event) => (),
        }
    }
}

fn main() {

    unsafe {env::set_var("RUST_BACKTRACE", "1");}

    // TODO: fix the absolute path
    let path = std::path::Path::new("D:/projects/rust/small-tray-app/src/pixil-frame-1.png");

    // let icon_size: Option<(u32, u32)> = Some((11, 11));
    // let icon = Icon::from_path(path, icon_size).unwrap(); 

    let icon = load_icon(&path);

    let tray_menu = Menu::with_items(&[
            // TODO:
            //&MenuItem::new("reset", true, None),
            &MenuItem::new("open dustbin", true, None),

            // TODO settings altough...
            // &MenuItem::new("settings", true, None),
            &MenuItem::new("exit", true, None),
            &MenuItem::new("enable/disable autorun", true, None)
        ]).expect("Couldn't make the menu");

    let tray_icon = TrayIconBuilder::new().with_menu(Box::new(tray_menu)).with_icon(icon).with_tooltip("dustbin").build().unwrap();

    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    let mut app = App::default();

    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::TrayIconEvent(event));
    }));
    
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::MenuEvent(event));
    }));

    event_loop.run_app(&mut app).unwrap();
}


fn load_icon(path: &std::path::Path) -> Icon {
    let (rgba, width, height) = {
        let image = image::open(path).expect("Failed to open the icon from path.").into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Icon::from_rgba(rgba, width, height)
        .expect("Failed to open the icon.")
}