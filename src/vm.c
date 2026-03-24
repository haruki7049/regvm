#include "./vm.h"
#include <argtable3.h>
#include <linenoise.h>
#include <stdio.h>
#include <stdlib.h>

RegVM create_reg_vm() {
  RegVM result;
  result.r0 = 0;
  result.r1 = 0;
  result.r2 = 0;
  result.r3 = 0;

  return result;
}

int interpreter(void *argtable, int argtable_size) {
  RegVM vm = create_reg_vm();
  printf("&vm: %p\n", &vm);
  printf("vm.r0: %d\n", vm.r0);
  printf("vm.r1: %d\n", vm.r1);
  printf("vm.r2: %d\n", vm.r2);
  printf("vm.r3: %d\n", vm.r3);
  printf("\n");
  printf("Hello, it's regvm!!\n");

  mainloop(vm);

  arg_freetable(argtable, argtable_size);
  return 0;
}

void mainloop(RegVM vm) {
  char *line;
  while ((line = linenoise("regvm:> ")) != NULL) {
    printf("You wrote: %s\n", line);
    free(line);
  }
}
