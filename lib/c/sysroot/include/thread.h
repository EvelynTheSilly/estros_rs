#ifndef __THREAD_H__
#define __THREAD_H__
#include <stdint.h>
#include <stddef.h>

void thread_exit(void);
uint64_t spawn_thread(
    typeof(void *(void *_Nullable)) *start_routine,
    void *arg,
    size_t stack_size
);

#endif
