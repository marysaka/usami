        /*0000*/                   S2R R0, SR_CTAID.X ;
        /*0010*/                   IADD3 R0, R0, c[0x0][0x20], RZ ;
        /*0020*/                   IMAD.SHL.U32 R3, R0.reuse, 0x4, RZ ;
        /*0030*/                   IMAD.SHL.U32 R0, R0, 0x2, RZ ;
        /*0040*/                   ISETP.GE.U32.AND P2, PT, R3, c[0x0][0x68], PT ;
        /*0050*/                   ISETP.GE.U32.AND P0, PT, R0.reuse, c[0x0][0x48], PT ;
        /*0060*/                   ISETP.GE.U32.AND P1, PT, R0, c[0x0][0x58], PT ;
        /*0070*/                   ULDC.64 UR4, c[0x0][0x40] ;
        /*0080*/                   ULDC.64 UR8, c[0x0][0x60] ;
        /*0090*/                   LDG.E.U16.CONSTANT.SYS R1, [R0.U32+UR4], !P0 ;
        /*00a0*/                   ULDC.64 UR6, c[0x0][0x50] ;
        /*00b0*/                   LDG.E.CONSTANT.SYS R3, [R3.U32+UR8], !P2 ;
        /*00c0*/                   LDG.E.U16.CONSTANT.SYS R2, [R0.U32+UR6], !P1 ;
        /*00d0*/                   S2R R6, SR_LANEID ;
        /*00e0*/                   LOP3.LUT R4, R6, 0x8, RZ, 0xc0, !PT ;
        /*00f0*/                   IMAD.SHL.U32 R5, R4, 0x2, RZ ;
        /*0100*/                   LOP3.LUT R4, R6, 0x10, RZ, 0xc0, !PT ;
        /*0110*/                   LOP3.LUT R7, R5.reuse, 0x7, R6.reuse, 0xf8, !PT ;
        /*0120*/                   SHF.R.U32.HI R8, RZ, 0x2, R6 ;
        /*0130*/                   LEA.HI R4, R4, R7, RZ, 0x1f ;
        /*0140*/                   LOP3.LUT R9, R6.reuse, 0x3, RZ, 0xc0, !PT ;
        /*0150*/                   LOP3.LUT R6, R6, 0x7, R5, 0xc8, !PT ;
        /*0160*/                   LOP3.LUT R7, R5, 0x10, RZ, 0xc0, !PT ;
        /*0170*/                   LOP3.LUT R0, R4.reuse, 0xf, RZ, 0xc0, !PT ;
        /*0180*/                   LOP3.LUT R5, R4, 0x10, RZ, 0xc0, !PT ;
        /*0190*/                   IMAD R4, R8, 0x2, R9 ;
        /*01a0*/                   IMAD R6, R6, 0x4, R7 ;
        /*01b0*/                   IMAD R0, R0, 0x4, R5 ;
        /*01c0*/                   STS.U16 [0x300], R1 ;
        /*01d0*/                   STS [RZ], R3 ;
        /*01e0*/                   STS.U16 [0x200], R2 ;
        /*01f0*/                   NOP ;
        /*0200*/                   NOP ;
        /*0210*/                   LDSM.16.M88.2 R6, [R6+0x200] ;
        /*0220*/                   IMAD.SHL.U32 R5, R4, 0x8, RZ ;
        /*0230*/                   LDS.U.64 R8, [R4.X8] ;
        /*0240*/                   LDS.U.64 R10, [R4.X8+0x80] ;
        /*0250*/                   LDSM.16.MT88.4 R0, [R0+0x300] ;
        /*0260*/                   HMMA.16816.F32 R8, R0, R6, R8 ;
        /*0270*/                   IADD3 R7, R5, 0x80, RZ ;
        /*0280*/                   IMAD.MOV.U32 R0, RZ, RZ, R8 ;
        /*0290*/                   IMAD.MOV.U32 R1, RZ, RZ, R9 ;
        /*02a0*/                   NOP ;
        /*02b0*/                   NOP ;
        /*02c0*/                   IMAD.MOV.U32 R2, RZ, RZ, R10 ;
        /*02d0*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*02e0*/                   IMAD.MOV.U32 R3, RZ, RZ, R11 ;
        /*02f0*/                   STG.E.64.STRONG.CTA [R5.U32+UR4], R0 ;
        /*0300*/                   STG.E.64.STRONG.CTA [R7.U32+UR4], R2 ;
        /*0310*/                   EXIT ;
        /*0320*/                   BRA 0x320;
        /*0330*/                   NOP;
        /*0340*/                   NOP;
        /*0350*/                   NOP;
        /*0360*/                   NOP;
        /*0370*/                   NOP;
