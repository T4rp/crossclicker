#ifndef UINPUT_WRAPPER_H
#define UINPUT_WRAPPER_H

#include <linux/uinput.h>

int safe_ioctl_int(int fd, unsigned long request, int arg);
int safe_ioctl_no_arg(int fd, unsigned long request);
int safe_ioctl_ptr_arg(int fd, unsigned long request, const void *arg);

#endif

