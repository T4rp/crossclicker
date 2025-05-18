#[allow(warnings)]
mod uinput {
    include!(concat!(env!("OUT_DIR"), "/uinput.rs"));
}

mod virtual_input;

use std::{sync::mpsc::RecvTimeoutError, thread, time::Duration};

use gtk4::{
    Application, ApplicationWindow, Button,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::{BoxExt, ButtonExt, EditableExt, GtkWindowExt},
};

enum VirtualDeviceCmd {
    Ping, // make sure thread is alive for debugging purposes
    EnableAutoclick(Duration),
    DisableAutoclick,
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let mouse = virtual_input::VirtualMouse::new().expect("Failed to create virtual mouse");

        let mut is_autoclicking = false;
        let mut speed = Duration::from_millis(50);

        loop {
            if is_autoclicking {
                mouse.left_click().expect("Click event failed");
            }

            match rx.recv_timeout(speed) {
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
                Ok(VirtualDeviceCmd::Ping) => {}
                Ok(VirtualDeviceCmd::EnableAutoclick(s)) => {
                    speed = s;
                    is_autoclicking = true;
                }
                Ok(VirtualDeviceCmd::DisableAutoclick) => is_autoclicking = false,
            };
        }
    });

    tx.send(VirtualDeviceCmd::Ping).unwrap();

    let app = Application::builder()
        .application_id("com.t4rp.crossclicker")
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(500)
            .default_height(400)
            .maximized(false)
            .title("CrossClicker")
            .build();

        window.present();

        let speed_entry = gtk4::Entry::builder()
            .placeholder_text("speed (ms)")
            .input_purpose(gtk4::InputPurpose::Digits)
            .text("50")
            .build();

        let start_button = Button::builder().label("start").build();
        let stop_button = Button::builder().label("stop").build();

        let sender = tx.clone();

        let se = speed_entry.clone();
        start_button.connect_clicked(move |_| {
            let text = se.text();
            let speed = text.as_str().trim().parse::<u64>().unwrap_or(50).max(50);

            sender
                .send(VirtualDeviceCmd::EnableAutoclick(Duration::from_millis(
                    speed,
                )))
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

        gtk_box.append(&speed_entry);
        gtk_box.append(&start_button);
        gtk_box.append(&stop_button);

        window.set_child(Some(&gtk_box));
    });

    app.run();
}
