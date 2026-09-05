#include <stdio.h>
#include <syscalls.h>
#include <string.h>

void puts(const char* str) {
  sys_write(str, strlen(str));
}

