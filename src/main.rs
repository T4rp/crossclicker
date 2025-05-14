#[allow(clippy::all)]
mod uinput {
    include!(concat!(env!("OUT_DIR"), "/uinput.rs"));
}

mod virtual_input;

use std::{sync::mpsc::RecvTimeoutError, thread, time::Duration};

use gtk4::{
    Application, ApplicationWindow, Button,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::{BoxExt, ButtonExt, GtkWindowExt},
};

enum VirtualDeviceCmd {
    EnableAutoclick,
    DisableAutoclick,
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let mouse = virtual_input::VirtualMouse::new().expect("Failed to create virtual mouse");
        let mut is_autoclicking = false;

        loop {
            if is_autoclicking {
                mouse.left_click().expect("Click event failed");
            }

            match rx.recv_timeout(Duration::from_millis(50)) {
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
                Ok(VirtualDeviceCmd::EnableAutoclick) => is_autoclicking = true,
                Ok(VirtualDeviceCmd::DisableAutoclick) => is_autoclicking = false,
            };
        }
    });

    let app = Application::builder()
        .application_id("com.t4rp.autoclickd")
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(500)
            .default_height(400)
            .maximized(false)
            .title("autoclickd")
            .build();

        window.present();

        let start_button = Button::builder().label("start").build();
        let stop_button = Button::builder().label("stop").build();

        let sender = tx.clone();
        start_button.connect_clicked(move |_| {
            sender
                .send(VirtualDeviceCmd::EnableAutoclick)
                .expect("Failed to send message");
        });

        let sender = tx.clone();
        stop_button.connect_clicked(move |_| {
            sender
                .send(VirtualDeviceCmd::DisableAutoclick)
                .expect("Failed to send message");
        });

        let gtk_box = gtk4::Box::builder()
            .margin_start(12)
            .margin_end(12)
            .margin_top(12)
            .margin_bottom(12)
            .orientation(gtk4::Orientation::Vertical)
            .halign(gtk4::Align::Center)
            .build();

        gtk_box.append(&start_button);
        gtk_box.append(&stop_button);

        window.set_child(Some(&gtk_box));
    });

    app.run();
}
