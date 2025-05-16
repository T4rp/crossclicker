use std::{io, os::raw};

use crate::uinput::{
    self, __s32, __suseconds_t, __time_t, __u16, __u32, UI_DEV_CREATE, UI_DEV_SETUP, input_event,
};
use libc::{O_NONBLOCK, O_WRONLY};

fn emit_input_event(
    fd: libc::c_int,
    t: __u16,
    code: __u16,
    val: __s32,
) -> io::Result<libc::ssize_t> {
    let ie = uinput::input_event {
        time: uinput::timeval {
            tv_sec: 0 as __time_t,
            tv_usec: 0 as __suseconds_t,
        },
        type_: t,
        code,
        value: val,
    };

    unsafe {
        match libc::write(
            fd,
            &ie as *const uinput::input_event as *const libc::c_void,
            std::mem::size_of::<input_event>(),
        ) {
            -1 => Err(io::Error::last_os_error()),
            r => Ok(r),
        }
    }
}

fn ioctl_int_arg(fd: raw::c_int, op: raw::c_ulong, arg: raw::c_int) -> io::Result<()> {
    unsafe {
        if uinput::ioctl(fd, op, arg) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn ioctl_no_arg(fd: raw::c_int, op: raw::c_ulong) -> io::Result<()> {
    unsafe {
        if uinput::ioctl(fd, op) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn ioctl_ptr_arg(fd: raw::c_int, op: raw::c_ulong, arg: *const raw::c_void) -> io::Result<()> {
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
    fd: libc::c_int,
}

#[allow(unused)]
impl VirtualMouse {
    pub fn new() -> Result<Self, io::Error> {
        let fd = unsafe {
            let fd = open_uinput()?;

            ioctl_int_arg(
                fd,
                uinput::UI_SET_EVBIT as raw::c_ulong,
                uinput::EV_KEY as raw::c_int,
            )?;

            ioctl_int_arg(
                fd,
                uinput::UI_SET_KEYBIT as raw::c_ulong,
                uinput::BTN_LEFT as raw::c_int,
            )?;

            ioctl_int_arg(
                fd,
                uinput::UI_SET_KEYBIT as raw::c_ulong,
                uinput::BTN_RIGHT as raw::c_int,
            )?;

            ioctl_int_arg(
                fd,
                uinput::UI_SET_KEYBIT as raw::c_ulong,
                uinput::BTN_MIDDLE as raw::c_int,
            )?;

            let mut name: [raw::c_char; 80] = [0; 80];

            for (i, &b) in b"VirtualMouse".iter().enumerate() {
                name[i] = b as raw::c_char;
            }

            let usetup = uinput::uinput_setup {
                id: uinput::input_id {
                    bustype: uinput::BUS_USB as __u16,
                    vendor: 0x1337 as __u16,
                    product: 0x7331 as __u16,
                    version: 0 as __u16,
                },
                name,
                ff_effects_max: 0 as __u32,
            };

            ioctl_ptr_arg(
                fd as raw::c_int,
                UI_DEV_SETUP as raw::c_ulong,
                &usetup as *const _ as *const raw::c_void,
            )?;

            ioctl_no_arg(fd, UI_DEV_CREATE as raw::c_ulong)?;

            fd
        };

        Ok(Self { fd })
    }

    pub fn left_click(&self) -> io::Result<()> {
        emit_input_event(
            self.fd as raw::c_int,
            uinput::EV_KEY as __u16,
            uinput::BTN_LEFT as __u16,
            1 as raw::c_int,
        )?;

        emit_input_event(
            self.fd as raw::c_int,
            uinput::EV_SYN as __u16,
            uinput::SYN_REPORT as __u16,
            0 as raw::c_int,
        )?;

        emit_input_event(
            self.fd as raw::c_int,
            uinput::EV_KEY as __u16,
            uinput::BTN_LEFT as __u16,
            0 as raw::c_int,
        )?;

        emit_input_event(
            self.fd as raw::c_int,
            uinput::EV_SYN as __u16,
            uinput::SYN_REPORT as __u16,
            0 as raw::c_int,
        )?;

        Ok(())
    }

    pub fn right_click(&self) -> io::Result<()> {
        emit_input_event(
            self.fd as raw::c_int,
            uinput::EV_KEY as __u16,
            uinput::BTN_RIGHT as __u16,
            1 as raw::c_int,
        )?;

        emit_input_event(
            self.fd as raw::c_int,
            uinput::EV_SYN as __u16,
            uinput::SYN_REPORT as __u16,
            0 as raw::c_int,
        )?;

        emit_input_event(
            self.fd as raw::c_int,
            uinput::EV_KEY as __u16,
            uinput::BTN_RIGHT as __u16,
            0 as raw::c_int,
        )?;

        emit_input_event(
            self.fd as raw::c_int,
            uinput::EV_SYN as __u16,
            uinput::SYN_REPORT as __u16,
            0 as raw::c_int,
        )?;

        Ok(())
    }
}
