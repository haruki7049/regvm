#include "./vm.h"
#include <stdio.h>

int main() {
  struct RegVM vm = create_reg_vm();
  printf("&vm: %p\n", &vm);
  printf("vm.r0: %d\n", vm.r0);
  printf("vm.r1: %d\n", vm.r1);
  printf("vm.r2: %d\n", vm.r2);
  printf("vm.r3: %d\n", vm.r3);
  printf("\n");

  printf("Hello, it's regvm!!\n");
  return 0;
}
