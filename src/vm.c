#include "./vm.h"

struct RegVM create_reg_vm() {
  struct RegVM result;
  result.r0 = 0;
  result.r1 = 0;
  result.r2 = 0;
  result.r3 = 0;

  return result;
}
