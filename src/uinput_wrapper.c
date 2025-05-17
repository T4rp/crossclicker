#include <linux/uinput.h>
#include "uinput_wrapper.h"

int safe_ioctl_int(int fd, unsigned long request, int arg) {
  return ioctl(fd, request, arg);
}

int safe_ioctl_no_arg(int fd, unsigned long request) {
  return ioctl(fd, request);
}

int safe_ioctl_ptr_arg(int fd, unsigned long request, const void *arg) {
  return ioctl(fd, request, arg);
}
