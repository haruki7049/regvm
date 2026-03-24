#include "./main.h"
#include "./vm.h"
#include <argtable3.h>
#include <stdio.h>

struct arg_lit *help, *version;
struct arg_end *end;

const char program_name[] = "regvm";

int main(int argc, char *argv[]) {
  void *argtable[] = {
      help = arg_lit0("h", "help", "display this help and exit"),
      version = arg_lit0("V", "version", "display version info and exit"),

      end = arg_end(20),
  };
  int argtable_size = sizeof(argtable) / sizeof(argtable[0]);
  int nerrors = arg_parse(argc, argv, argtable);

  if (help->count > 0) {
    return display_help_message(argtable, argtable_size);
  } else if (nerrors > 0) {
    return display_error_message(argtable, argtable_size, program_name);
  }

  RegVM vm = create_reg_vm();
  printf("&vm: %p\n", &vm);
  printf("vm.r0: %d\n", vm.r0);
  printf("vm.r1: %d\n", vm.r1);
  printf("vm.r2: %d\n", vm.r2);
  printf("vm.r3: %d\n", vm.r3);
  printf("\n");
  printf("Hello, it's regvm!!\n");

  arg_freetable(argtable, argtable_size);
  return 0;
}

int display_help_message(void **argtable, int argtable_size) {
  printf("Usage: %s", program_name);
  arg_print_syntax(stdout, argtable, "\n");
  printf("A virtual machine program for my practice to write C-language.\n\n");
  arg_print_glossary(stdout, argtable, "  %-25s %s\n");

  arg_freetable(argtable, argtable_size);
  return 0;
}

int display_error_message(void **argtable, int argtable_size,
                          const char program_name[]) {
  /* Display the error details contained in the arg_end struct.*/
  arg_print_errors(stdout, end, program_name);
  printf("Try '%s --help' for more information.\n", program_name);

  arg_freetable(argtable, argtable_size);
  return 1;
}
