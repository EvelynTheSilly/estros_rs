#include <stdio.h>
#include <syscalls.h>

void putc(int c) {
  write(&c, 1);
}
