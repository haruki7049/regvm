#include "./vm.h"
#include <argtable3.h>
#include <stdio.h>

const int exitcode = 0;
const char program_name[] = "regvm";

typedef struct {
  arg_lit_t *help;
} Cli;

int main() {
  Cli cli = {
      arg_litn("h", "help", 0, 1, "display this help and exit"),
  };

  RegVM vm = create_reg_vm();
  printf("&vm: %p\n", &vm);
  printf("vm.r0: %d\n", vm.r0);
  printf("vm.r1: %d\n", vm.r1);
  printf("vm.r2: %d\n", vm.r2);
  printf("vm.r3: %d\n", vm.r3);
  printf("\n");

  printf("Hello, it's regvm!!\n");
  return 0;
}
