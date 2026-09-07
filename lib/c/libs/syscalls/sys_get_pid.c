#include "syscalls.h"

uint64_t sys_get_pid(void) {
    register uint64_t r0 __asm__("x0");

    __asm__ volatile("svc #4"
                     : "+r"(r0)
                     :
                     : "memory");

    return r0;
}
