#include "syscalls.h"

void sys_noop(void) {
    __asm__ volatile("svc #0");
}
