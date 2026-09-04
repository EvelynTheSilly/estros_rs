#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <syscalls.h>

int main(){
    char hello[] = "hello from memcopy";
    size_t len = strlen(hello);
    char *buf = malloc(len + 1);
    if (!buf) return 1;
    for (size_t i = 0; i <= len; i++) {
        buf[i] = hello[i];
    }
    puts(buf);
    free(buf);
    exit();
}