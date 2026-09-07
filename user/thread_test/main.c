#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <syscalls.h>
#include <thread.h>
#include <stddef.h>

void* poopoopeepee(void* arg) {
    puts("wee woo yayyyyy");
    for (int i = 0; i < 100; i++) {
        sys_noop();
    };
    sys_exit();
}

int main(){
    puts("hello from threadding");
    puts("the goal is to make a second thread, exit the main thread, then exit the process from the second thread");

    spawn_thread(*poopoopeepee, NULL, 4096);
    spawn_thread(*poopoopeepee, NULL, 4096);
    spawn_thread(*poopoopeepee, NULL, 4096);
    spawn_thread(*poopoopeepee, NULL, 4096);
    spawn_thread(*poopoopeepee, NULL, 4096);
    spawn_thread(*poopoopeepee, NULL, 4096);
    spawn_thread(*poopoopeepee, NULL, 4096);
    spawn_thread(*poopoopeepee, NULL, 4096);
    
    thread_exit();
}
