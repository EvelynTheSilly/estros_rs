#include "syscalls.h"

void sys_exit(void) {
    __asm__ volatile("svc #2");
}
