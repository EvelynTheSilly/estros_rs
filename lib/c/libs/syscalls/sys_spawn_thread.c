#include "syscalls.h"

uint64_t sys_spawn_thread(const void *location, void *arg) {
    register const void *r0 __asm__("x0") = location;
    register void *r1 __asm__("x1") = arg;

    __asm__ volatile("svc #6"
                     : "+r"(r0)
                     : "r"(r1)
                     : "memory");

    return (uint64_t)r0;
}
