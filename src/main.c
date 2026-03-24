#include "./main.h"
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

struct RegVM create_reg_vm() {
  struct RegVM result;
  result.r0 = 0;
  result.r1 = 0;
  result.r2 = 0;
  result.r3 = 0;

  return result;
}
