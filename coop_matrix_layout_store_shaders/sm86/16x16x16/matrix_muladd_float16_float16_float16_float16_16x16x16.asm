        /*0000*/                   I2F.F16 R17, 0xa3 ;
        /*0010*/                   IMAD.MOV.U32 R7, RZ, RZ, c[0x0][0x58] ;
        /*0020*/                   STL.128 [RZ], RZ ;
        /*0030*/                   IMAD.MOV.U32 R22, RZ, RZ, c[0x0][0x48] ;
        /*0040*/                   ISETP.GT.U32.AND P0, PT, R7, 0x3, PT ;
        /*0050*/                   STL.128 [0x10], RZ ;
        /*0060*/                   I2F.F16 R18, 0xa4 ;
        /*0070*/                   ISETP.GT.U32.AND P1, PT, R22, 0x3, PT ;
        /*0080*/                   STL.128 [0x20], RZ ;
        /*0090*/                   I2F.F16 R8, 0xa1 ;
        /*00a0*/                   I2F.F16 R16, 0xa2 ;
        /*00b0*/                   PRMT R23, R17, 0x5410, R18 ;
        /*00c0*/                   I2F.F16 R21, 0xa7 ;
        /*00d0*/                   I2F.F16 R0, 0xa8 ;
        /*00e0*/                   PRMT R22, R8, 0x5410, R16 ;
        /*00f0*/                   STL.64 [RZ], R22 ;
        /*0100*/                   I2F.F16 R19, 0xa5 ;
        /*0110*/                   I2F.F16 R20, 0xa6 ;
        /*0120*/                   PRMT R17, R21, 0x5410, R0 ;
        /*0130*/                   I2F.F16 R11, 0xb3 ;
        /*0140*/                   I2F.F16 R12, 0xb4 ;
        /*0150*/                   PRMT R16, R19, 0x5410, R20 ;
        /*0160*/                   STL.64 [0x8], R16 ;
        /*0170*/                   I2F.F16 R2, 0xc1 ;
        /*0180*/                   I2F.F16 R3, 0xc2 ;
        /*0190*/                   PRMT R19, R11, 0x5410, R12 ;
        /*01a0*/                   I2F.F16 R6, 0xc5 ;
        /*01b0*/                   I2F.F16 R7, 0xc6 ;
        /*01c0*/                   PRMT R12, R2, 0x5410, R3 ;
        /*01d0*/                   @P1 IMAD.MOV.U32 R3, RZ, RZ, c[0x0][0x44] ;
        /*01e0*/                   @P1 IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x40] ;
        /*01f0*/                   I2F.F16 R9, 0xb1 ;
        /*0200*/                   @P1 LDG.E.STRONG.SM R3, [R2] ;
        /*0210*/                   I2F.F16 R10, 0xb2 ;
        /*0220*/                   PRMT R6, R6, 0x5410, R7 ;
        /*0230*/                   I2F.F16 R15, 0xb7 ;
        /*0240*/                   I2F.F16 R1, 0xb8 ;
        /*0250*/                   PRMT R18, R9, 0x5410, R10 ;
        /*0260*/                   STL.64 [0x10], R18 ;
        /*0270*/                   I2F.F16 R8, 0xc7 ;
        /*0280*/                   I2F.F16 R0, 0xc8 ;
        /*0290*/                   PRMT R11, R15, 0x5410, R1 ;
        /*02a0*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x54] ;
        /*02b0*/                   I2F.F16 R13, 0xb5 ;
        /*02c0*/                   I2F.F16 R14, 0xb6 ;
        /*02d0*/                   PRMT R7, R8, 0x5410, R0 ;
        /*02e0*/                   @P0 IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x50] ;
        /*02f0*/                   STL.64 [0x28], R6 ;
        /*0300*/                   I2F.F16 R4, 0xc3 ;
        /*0310*/                   @P0 LDG.E.STRONG.SM R8, [R0] ;
        /*0320*/                   I2F.F16 R5, 0xc4 ;
        /*0330*/                   PRMT R10, R13, 0x5410, R14 ;
        /*0340*/                   STL.64 [0x18], R10 ;
        /*0350*/                   PRMT R13, R4, 0x5410, R5 ;
        /*0360*/                   STL.64 [0x20], R12 ;
        /*0370*/                   NOP ;
        /*0380*/                   NOP ;
        /*0390*/                   LDL.128 R12, [0x10] ;
        /*03a0*/                   LDL.128 R4, [RZ] ;
        /*03b0*/                   LDL.128 R16, [0x20] ;
        /*03c0*/                   @!P1 IMAD.MOV.U32 R2, RZ, RZ, RZ ;
        /*03d0*/                   @!P0 CS2R R0, SRZ ;
        /*03e0*/                   @P1 IMAD.SHL.U32 R2, R3, 0x2, RZ ;
        /*03f0*/                   S2R R10, SR_LANEID ;
        /*0400*/                   @P0 IMAD.SHL.U32 R0, R8, 0x2, RZ ;
        /*0410*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, RZ ;
        /*0420*/                   NOP ;
        /*0430*/                   MOVM.16.MT88 R14, R14 ;
        /*0440*/                   MOVM.16.MT88 R15, R15 ;
        /*0450*/                   SHF.R.U64 R21, R4.reuse, 0x10, R5.reuse ;
        /*0460*/                   LOP3.LUT R3, R4, 0xffff, RZ, 0xc0, !PT ;
        /*0470*/                   MOVM.16.MT88 R12, R12 ;
        /*0480*/                   PRMT R4, R5, 0x7710, RZ ;
        /*0490*/                   SHF.R.U32.HI R9, RZ, 0x10, R5 ;
        /*04a0*/                   MOVM.16.MT88 R13, R13 ;
        /*04b0*/                   SHF.R.U64 R20, R6, 0x10, R7 ;
        /*04c0*/                   LOP3.LUT R5, R6, 0xffff, RZ, 0xc0, !PT ;
        /*04d0*/                   PRMT R8, R7, 0x7710, RZ ;
        /*04e0*/                   SHF.R.U32.HI R11, RZ, 0x10, R7 ;
        /*04f0*/                   SHF.R.U32.HI R7, RZ, 0x2, R10 ;
        /*0500*/                   PRMT R9, R4, 0x5410, R9 ;
        /*0510*/                   LOP3.LUT R4, R21, 0xffff, RZ, 0xc0, !PT ;
        /*0520*/                   LOP3.LUT R6, R20, 0xffff, RZ, 0xc0, !PT ;
        /*0530*/                   IMAD R20, R7, R2, RZ ;
        /*0540*/                   PRMT R11, R8, 0x5410, R11 ;
        /*0550*/                   LOP3.LUT R7, R10, 0x3, RZ, 0xc0, !PT ;
        /*0560*/                   PRMT R8, R3, 0x5410, R4 ;
        /*0570*/                   PRMT R10, R5, 0x5410, R6 ;
        /*0580*/                   IMAD R7, R7, 0x4, R20 ;
        /*0590*/                   IADD3 R3, P0, R0, c[0x0][0x30], RZ ;
        /*05a0*/                   IMAD R2, R2, 0x8, R7 ;
        /*05b0*/                   IADD3 R4, R7, 0x10, RZ ;
        /*05c0*/                   HMMA.16816.F16 R14, R8.reuse, R14, R18 ;
        /*05d0*/                   IADD3 R6, R2, 0x10, RZ ;
        /*05e0*/                   IADD3 R4, P2, R4, R3.reuse, RZ ;
        /*05f0*/                   IADD3 R2, P1, R2, R3.reuse, RZ ;
        /*0600*/                   HMMA.16816.F16 R12, R8, R12, R16 ;
        /*0610*/                   IADD3 R6, P3, R6, R3, RZ ;
        /*0620*/                   IADD3.X R16, R1, c[0x0][0x34], RZ, P0, !PT ;
        /*0630*/                   IADD3 R0, P0, R7, R3, RZ ;
        /*0640*/                   IMAD.X R3, RZ, RZ, R16.reuse, P1 ;
        /*0650*/                   IMAD.X R1, RZ, RZ, R16.reuse, P0 ;
        /*0660*/                   IMAD.X R5, RZ, RZ, R16.reuse, P2 ;
        /*0670*/                   IMAD.X R7, RZ, RZ, R16, P3 ;
        /*0680*/                   NOP ;
        /*0690*/                   STG.E.STRONG.SM [R0], R12 ;
        /*06a0*/                   STG.E.STRONG.SM [R2], R13 ;
        /*06b0*/                   STG.E.STRONG.SM [R4], R14 ;
        /*06c0*/                   STG.E.STRONG.SM [R6], R15 ;
        /*06d0*/                   EXIT ;
        /*06e0*/                   BRA 0x6e0;
        /*06f0*/                   NOP;
        /*0700*/                   NOP;
        /*0710*/                   NOP;
        /*0720*/                   NOP;
        /*0730*/                   NOP;
        /*0740*/                   NOP;
        /*0750*/                   NOP;
        /*0760*/                   NOP;
        /*0770*/                   NOP;
