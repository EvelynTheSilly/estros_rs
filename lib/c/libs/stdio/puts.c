#include <stdio.h>
#include <syscalls.h>
#include <string.h>

void puts(const char* str) {
  write(str, strlen(str));
}

