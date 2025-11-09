#![allow(unused)]

use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent, menu::{Menu, MenuEvent, MenuItem}};
use std::env;
use image;
use winit::{application::ApplicationHandler, event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder}, platform::windows::EventLoopBuilderExtWindows};
use winit::event::WindowEvent;
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>
}

enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            },
            _ => (),
        }
    }
}

fn main() {

    unsafe {env::set_var("RUST_BACKTRACE", "1");}

    let path = std::path::Path::new("D:/projects/rust/small-tray-app/src/pixil-frame-0.png");

    // let icon_size: Option<(u32, u32)> = Some((11, 11));
    // let icon = Icon::from_path(path, icon_size).unwrap(); 

    let icon = load_icon(&path);

     let tray_menu = Menu::with_items(&[
            &MenuItem::new("reset", true, None),
            &MenuItem::new("open dustbin", true, None),

            // TODO settings altough...
            // &MenuItem::new("settings", true, None),
            &MenuItem::new("exit", true, None)
        ]).expect("Couldn't make the menu");

    let tray_icon = TrayIconBuilder::new().with_menu(Box::new(tray_menu)).with_icon(icon).with_tooltip("dustbin").build().unwrap();

    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();

    let proxy = event_loop.create_proxy();
    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::MenuEvent(event));
    }));
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