#include <stdio.h>
#include <syscalls.h>

void putc(int c) {
  sys_write(&c, 1);
}
