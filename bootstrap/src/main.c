#include <stdio.h>
#include <stdint.h>

extern uint64_t add(uint64_t, uint64_t);

int main() {
  printf("Hello");
  printf("Hello %lu", add(2, 7));
}
