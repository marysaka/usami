        /*0000*/                   IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x58] ;
        /*0010*/                   I2F.F16 R6, 0xb1 ;
        /*0020*/                   IMAD.MOV.U32 R16, RZ, RZ, c[0x0][0x48] ;
        /*0030*/                   STL.128 [RZ], RZ ;
        /*0040*/                   ISETP.GT.U32.AND P0, PT, R2, 0x3, PT ;
        /*0050*/                   ISETP.GT.U32.AND P1, PT, R16, 0x3, PT ;
        /*0060*/                   STL.128 [0x10], RZ ;
        /*0070*/                   I2F.F16 R7, 0xb2 ;
        /*0080*/                   I2F.F16 R9, 0xa1 ;
        /*0090*/                   I2F.F16 R10, 0xa2 ;
        /*00a0*/                   PRMT R6, R6, 0x5410, R7 ;
        /*00b0*/                   I2F.F16 R11, 0xa3 ;
        /*00c0*/                   I2F.F16 R12, 0xa4 ;
        /*00d0*/                   PRMT R16, R9, 0x5410, R10 ;
        /*00e0*/                   I2F.F16 R8, 0xb3 ;
        /*00f0*/                   I2F.F16 R1, 0xb4 ;
        /*0100*/                   PRMT R17, R11, 0x5410, R12 ;
        /*0110*/                   I2F.F16 R15, 0xa7 ;
        /*0120*/                   STL.64 [RZ], R16 ;
        /*0130*/                   I2F.F16 R0, 0xa8 ;
        /*0140*/                   PRMT R7, R8, 0x5410, R1 ;
        /*0150*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x54] ;
        /*0160*/                   I2F.F16 R3, 0xc1 ;
        /*0170*/                   STL.64 [0x10], R6 ;
        /*0180*/                   I2F.F16 R4, 0xc2 ;
        /*0190*/                   PRMT R11, R15, 0x5410, R0 ;
        /*01a0*/                   @P0 IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x50] ;
        /*01b0*/                   I2F.F16 R5, 0xc3 ;
        /*01c0*/                   I2F.F16 R2, 0xc4 ;
        /*01d0*/                   PRMT R8, R3, 0x5410, R4 ;
        /*01e0*/                   @P1 IMAD.MOV.U32 R3, RZ, RZ, c[0x0][0x44] ;
        /*01f0*/                   @P0 LDG.E.STRONG.CTA R4, [R0] ;
        /*0200*/                   I2F.F16 R13, 0xa5 ;
        /*0210*/                   I2F.F16 R14, 0xa6 ;
        /*0220*/                   PRMT R9, R5, 0x5410, R2 ;
        /*0230*/                   @P1 IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x40] ;
        /*0240*/                   STL.64 [0x18], R8 ;
        /*0250*/                   @P1 LDG.E.STRONG.CTA R3, [R2] ;
        /*0260*/                   PRMT R10, R13, 0x5410, R14 ;
        /*0270*/                   STL.64 [0x8], R10 ;
        /*0280*/                   NOP ;
        /*0290*/                   NOP ;
        /*02a0*/                   LDL.128 R12, [0x10] ;
        /*02b0*/                   LDL.128 R8, [RZ] ;
        /*02c0*/                   @!P1 IMAD.MOV.U32 R2, RZ, RZ, RZ ;
        /*02d0*/                   S2R R0, SR_LANEID ;
        /*02e0*/                   @P1 IMAD.SHL.U32 R2, R3, 0x2, RZ ;
        /*02f0*/                   SHF.R.U32.HI R1, RZ, 0x2, R0 ;
        /*0300*/                   LOP3.LUT R3, R0, 0x3, RZ, 0xc0, !PT ;
        /*0310*/                   IMAD R6, R1, R2, RZ ;
        /*0320*/                   @!P0 CS2R R0, SRZ ;
        /*0330*/                   IMAD R3, R3, 0x4, R6 ;
        /*0340*/                   @P0 IMAD.SHL.U32 R0, R4, 0x2, RZ ;
        /*0350*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, RZ ;
        /*0360*/                   IMAD R2, R2, 0x8, R3 ;
        /*0370*/                   IADD3 R4, P0, R0, c[0x0][0x30], RZ ;
        /*0380*/                   IADD3.X R5, R1, c[0x0][0x34], RZ, P0, !PT ;
        /*0390*/                   IADD3 R0, P0, R3, R4.reuse, RZ ;
        /*03a0*/                   IADD3 R2, P1, R2, R4, RZ ;
        /*03b0*/                   IMAD.X R1, RZ, RZ, R5.reuse, P0 ;
        /*03c0*/                   IMAD.X R3, RZ, RZ, R5, P1 ;
        /*03d0*/                   NOP ;
        /*03e0*/                   MOVM.16.MT88 R12, R12 ;
        /*03f0*/                   MOVM.16.MT88 R13, R13 ;
        /*0400*/                   HMMA.16816.F16 R14, R8, R12, R14 ;
        /*0410*/                   NOP ;
        /*0420*/                   NOP ;
        /*0430*/                   STG.E.STRONG.CTA [R0], R14 ;
        /*0440*/                   STG.E.STRONG.CTA [R2], R15 ;
        /*0450*/                   EXIT ;
        /*0460*/                   BRA 0x460;
        /*0470*/                   NOP;
