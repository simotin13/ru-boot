    .section    .fvectors
    .arm
    .extern  _PowerON_Reset

vector_table:
    LDR pc, =_PowerON_Reset 

