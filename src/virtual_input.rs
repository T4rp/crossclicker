use std::{io, os::raw};

use crate::uinput::{
    self, __s32, __suseconds_t, __time_t, __u16, __u32, EV_KEY, EV_SYN, SYN_REPORT, UI_DEV_CREATE,
    UI_DEV_SETUP, input_event,
};
use libc::{O_NONBLOCK, O_WRONLY};

fn emit_key_event(fd: libc::c_int, code: __u16, val: __s32) -> io::Result<()> {
    let key_event = uinput::input_event {
        time: uinput::timeval {
            tv_sec: 0 as __time_t,
            tv_usec: 0 as __suseconds_t,
        },
        type_: EV_KEY as __u16,
        code: code as __u16,
        value: val as __s32,
    };

    let sync_event = uinput::input_event {
        time: uinput::timeval {
            tv_sec: 0 as __time_t,
            tv_usec: 0 as __suseconds_t,
        },
        type_: EV_SYN as __u16,
        code: SYN_REPORT as __u16,
        value: 0 as __s32,
    };

    unsafe {
        let res = libc::write(
            fd,
            &key_event as *const _ as *const libc::c_void,
            std::mem::size_of::<input_event>() as libc::size_t,
        );

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        let res = libc::write(
            fd,
            &sync_event as *const _ as *const libc::c_void,
            std::mem::size_of::<input_event>() as libc::size_t,
        );

        if res < 0 {
            return Err(io::Error::last_os_error());
        }
    };

    Ok(())
}

fn ioctl_int_arg(fd: raw::c_int, op: raw::c_ulong, arg: raw::c_int) -> io::Result<()> {
    unsafe {
        if uinput::safe_ioctl_int(fd, op, arg) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn ioctl_no_arg(fd: raw::c_int, op: raw::c_ulong) -> io::Result<()> {
    unsafe {
        if uinput::safe_ioctl_no_arg(fd, op) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn ioctl_ptr_arg(fd: raw::c_int, op: raw::c_ulong, arg: *const raw::c_void) -> io::Result<()> {
    unsafe {
        if uinput::safe_ioctl_ptr_arg(fd, op, arg) == -1 {
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
        emit_key_event(
            self.fd as raw::c_int,
            uinput::BTN_LEFT as __u16,
            1 as raw::c_int,
        )?;

        emit_key_event(
            self.fd as raw::c_int,
            uinput::BTN_LEFT as __u16,
            0 as raw::c_int,
        )?;

        Ok(())
    }

    pub fn right_click(&self) -> io::Result<()> {
        emit_key_event(
            self.fd as raw::c_int,
            uinput::BTN_RIGHT as __u16,
            1 as raw::c_int,
        )?;

        emit_key_event(
            self.fd as raw::c_int,
            uinput::BTN_RIGHT as __u16,
            0 as raw::c_int,
        )?;

        Ok(())
    }
}
