use std::{ffi::c_void, io};

use crate::uinput::{self, UI_DEV_CREATE, UI_DEV_SETUP, input_event};
use libc::{O_NONBLOCK, O_WRONLY};

fn emit_input_event(fd: i32, t: u16, code: u16, val: i32) -> io::Result<libc::ssize_t> {
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
        match libc::write(
            fd,
            &ie as *const uinput::input_event as *const c_void,
            std::mem::size_of::<input_event>(),
        ) {
            -1 => Err(io::Error::last_os_error()),
            r => Ok(r),
        }
    }
}

fn ioctl_int_arg(fd: libc::c_int, op: libc::c_ulong, arg: libc::c_int) -> io::Result<()> {
    unsafe {
        if uinput::ioctl(fd, op, arg) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn ioctl_no_arg(fd: libc::c_int, op: libc::c_ulong) -> io::Result<()> {
    unsafe {
        if uinput::ioctl(fd, op) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn ioctl_ptr_arg(fd: libc::c_int, op: libc::c_ulong, arg: *const c_void) -> io::Result<()> {
    unsafe {
        if uinput::ioctl(fd, op, arg) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn open_uinput() -> Result<libc::c_int, io::Error> {
    unsafe {
        let fd = libc::open(c"/dev/uinput".as_ptr(), O_WRONLY | O_NONBLOCK);

        if fd < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(fd)
        }
    }
}

pub struct VirtualMouse {
    fd: i32,
}

#[allow(unused)]
impl VirtualMouse {
    pub fn new() -> Result<Self, io::Error> {
        let fd = unsafe {
            let fd = open_uinput()?;

            ioctl_int_arg(fd, uinput::UI_SET_EVBIT as u64, uinput::EV_KEY as i32)?;
            ioctl_int_arg(fd, uinput::UI_SET_KEYBIT as u64, uinput::BTN_LEFT as i32)?;
            ioctl_int_arg(fd, uinput::UI_SET_KEYBIT as u64, uinput::BTN_RIGHT as i32)?;
            ioctl_int_arg(fd, uinput::UI_SET_KEYBIT as u64, uinput::BTN_MIDDLE as i32)?;

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

            ioctl_ptr_arg(
                fd,
                UI_DEV_SETUP as u64,
                &usetup as *const _ as *const c_void,
            )?;

            ioctl_no_arg(fd, UI_DEV_CREATE as u64)?;

            fd
        };

        Ok(Self { fd })
    }

    pub fn left_click(&self) -> io::Result<()> {
        emit_input_event(self.fd, uinput::EV_KEY as u16, uinput::BTN_LEFT as u16, 1)?;
        emit_input_event(self.fd, uinput::EV_SYN as u16, uinput::SYN_REPORT as u16, 0)?;
        emit_input_event(self.fd, uinput::EV_KEY as u16, uinput::BTN_LEFT as u16, 0)?;
        emit_input_event(self.fd, uinput::EV_SYN as u16, uinput::SYN_REPORT as u16, 0)?;
        Ok(())
    }

    pub fn right_click(&self) -> io::Result<()> {
        emit_input_event(self.fd, uinput::EV_KEY as u16, uinput::BTN_RIGHT as u16, 1)?;
        emit_input_event(self.fd, uinput::EV_SYN as u16, uinput::SYN_REPORT as u16, 0)?;
        emit_input_event(self.fd, uinput::EV_KEY as u16, uinput::BTN_RIGHT as u16, 0)?;
        emit_input_event(self.fd, uinput::EV_SYN as u16, uinput::SYN_REPORT as u16, 0)?;

        Ok(())
    }
}
