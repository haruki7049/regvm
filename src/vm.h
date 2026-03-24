typedef struct {
  unsigned int r0;
  unsigned int r1;
  unsigned int r2;
  unsigned int r3;
} RegVM;

RegVM create_reg_vm();
int interpreter(void *argtable, int argtable_size);
