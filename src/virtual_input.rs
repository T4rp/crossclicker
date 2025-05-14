use std::ffi::c_void;

use crate::uinput::{self, UI_DEV_CREATE, UI_DEV_SETUP, input_event};
use libc::{O_NONBLOCK, O_WRONLY};

fn emit_input_event(fd: i32, t: u16, code: u16, val: i32) {
    let ie = uinput::input_event {
        time: uinput::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        type_: t,
        code,
        value: val,
    };

    unsafe {
        libc::write(
            fd,
            &ie as *const uinput::input_event as *const c_void,
            std::mem::size_of::<input_event>(),
        );
    }
}

pub struct VirtualMouse {
    fd: i32,
}

#[allow(unused)]
impl VirtualMouse {
    pub fn new() -> Self {
        let fd = unsafe {
            let fd = libc::open(c"/dev/uinput".as_ptr(), O_WRONLY | O_NONBLOCK);

            uinput::ioctl(fd, uinput::UI_SET_EVBIT as u64, uinput::EV_KEY);
            uinput::ioctl(fd, uinput::UI_SET_KEYBIT as u64, uinput::BTN_LEFT);
            uinput::ioctl(fd, uinput::UI_SET_KEYBIT as u64, uinput::BTN_RIGHT);
            uinput::ioctl(fd, uinput::UI_SET_KEYBIT as u64, uinput::BTN_MIDDLE);

            let mut name: [i8; 80] = [0; 80];

            for (i, &b) in b"VirtualMouse".iter().enumerate() {
                name[i] = b as i8;
            }

            let usetup = uinput::uinput_setup {
                id: uinput::input_id {
                    bustype: uinput::BUS_USB as u16,
                    vendor: 0x1337,
                    product: 0x7331,
                    version: 0,
                },
                name,
                ff_effects_max: 0,
            };

            uinput::ioctl(fd, UI_DEV_SETUP as u64, &usetup as *const _);
            uinput::ioctl(fd, UI_DEV_CREATE as u64);

            fd
        };

        Self { fd }
    }

    pub fn left_click(&self) {
        emit_input_event(self.fd, uinput::EV_KEY as u16, uinput::BTN_LEFT as u16, 1);
        emit_input_event(self.fd, uinput::EV_SYN as u16, uinput::SYN_REPORT as u16, 0);
        emit_input_event(self.fd, uinput::EV_KEY as u16, uinput::BTN_LEFT as u16, 0);
        emit_input_event(self.fd, uinput::EV_SYN as u16, uinput::SYN_REPORT as u16, 0);
    }

    pub fn right_click(&self) {
        emit_input_event(self.fd, uinput::EV_KEY as u16, uinput::BTN_RIGHT as u16, 1);
        emit_input_event(self.fd, uinput::EV_SYN as u16, uinput::SYN_REPORT as u16, 0);
        emit_input_event(self.fd, uinput::EV_KEY as u16, uinput::BTN_RIGHT as u16, 0);
        emit_input_event(self.fd, uinput::EV_SYN as u16, uinput::SYN_REPORT as u16, 0);
    }
}
