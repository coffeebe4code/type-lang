#include <stdint.h>
#include <stdio.h>

extern uint64_t add(uint64_t, uint64_t);

int main() {
  uint64_t val = add(2, 7);
  printf("Hello %lu\n", val);
  return val;
}
