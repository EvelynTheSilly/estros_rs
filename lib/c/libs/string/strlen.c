#include <string.h>
#include <stddef.h>

size_t strlen(const char* str) {
  size_t cnt = 0;
  while (str[cnt] != 0) {
    cnt++;
  }
  return cnt;
}
