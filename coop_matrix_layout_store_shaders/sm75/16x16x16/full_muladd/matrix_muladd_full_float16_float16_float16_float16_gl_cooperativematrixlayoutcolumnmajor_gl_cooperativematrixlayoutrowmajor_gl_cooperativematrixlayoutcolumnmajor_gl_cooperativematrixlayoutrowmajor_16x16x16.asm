        /*0000*/                   S2R R0, SR_CTAID.X ;
        /*0010*/                   IADD3 R0, R0, c[0x0][0x20], RZ ;
        /*0020*/                   IMAD.SHL.U32 R0, R0, 0x2, RZ ;
        /*0030*/                   ISETP.GE.U32.AND P0, PT, R0.reuse, c[0x0][0x48], PT ;
        /*0040*/                   ISETP.GE.U32.AND P1, PT, R0.reuse, c[0x0][0x58], PT ;
        /*0050*/                   ISETP.GE.U32.AND P2, PT, R0, c[0x0][0x68], PT ;
        /*0060*/                   ULDC.64 UR4, c[0x0][0x40] ;
        /*0070*/                   ULDC.64 UR6, c[0x0][0x50] ;
        /*0080*/                   LDG.E.U16.CONSTANT.SYS R8, [R0.U32+UR4], !P0 ;
        /*0090*/                   ULDC.64 UR8, c[0x0][0x60] ;
        /*00a0*/                   LDG.E.U16.CONSTANT.SYS R1, [R0.U32+UR6], !P1 ;
        /*00b0*/                   LDG.E.U16.CONSTANT.SYS R2, [R0.U32+UR8], !P2 ;
        /*00c0*/                   S2R R7, SR_LANEID ;
        /*00d0*/                   LOP3.LUT R3, R7, 0x8, RZ, 0xc0, !PT ;
        /*00e0*/                   LOP3.LUT R4, R7, 0x7, RZ, 0xc0, !PT ;
        /*00f0*/                   IMAD R4, R3, 0x2, R4 ;
        /*0100*/                   LOP3.LUT R3, R7, 0x10, RZ, 0xc0, !PT ;
        /*0110*/                   LEA.HI R4, R3, R4, RZ, 0x1f ;
        /*0120*/                   LOP3.LUT R6, R7, 0xf, RZ, 0xc0, !PT ;
        /*0130*/                   LOP3.LUT R5, R4.reuse, 0xf, RZ, 0xc0, !PT ;
        /*0140*/                   LOP3.LUT R4, R4, 0x10, RZ, 0xc0, !PT ;
        /*0150*/                   IMAD R6, R6, 0x4, R3 ;
        /*0160*/                   IMAD R4, R5, 0x4, R4 ;
        /*0170*/                   STS.U16 [0x400], R8 ;
        /*0180*/                   STS.U16 [0x200], R1 ;
        /*0190*/                   STS.U16 [RZ], R2 ;
        /*01a0*/                   NOP ;
        /*01b0*/                   NOP ;
        /*01c0*/                   LDSM.16.MT88.4 R8, [R6+0x200] ;
        /*01d0*/                   LOP3.LUT R5, R7, 0xfffffffc, RZ, 0xc0, !PT ;
        /*01e0*/                   LDSM.16.MT88.4 R0, [R4+0x400] ;
        /*01f0*/                   LDSM.16.MT88.4 R12, [R4] ;
        /*0200*/                   HMMA.16816.F16 R8, R0, R8, R12 ;
        /*0210*/                   NOP ;
        /*0220*/                   HMMA.16816.F16 R10, R0, R10, R14 ;
        /*0230*/                   LOP3.LUT R0, R7, 0x3, RZ, 0xc0, !PT ;
        /*0240*/                   IMAD R5, R0, 0x4, R5 ;
        /*0250*/                   IADD3 R0, R5.reuse, 0x20, RZ ;
        /*0260*/                   IADD3 R1, R5.reuse, 0x10, RZ ;
        /*0270*/                   IADD3 R2, R5, 0x30, RZ ;
        /*0280*/                   NOP ;
        /*0290*/                   NOP ;
        /*02a0*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*02b0*/                   STG.E.STRONG.CTA [R5.U32+UR4], R8 ;
        /*02c0*/                   STG.E.STRONG.CTA [R0.U32+UR4], R9 ;
        /*02d0*/                   STG.E.STRONG.CTA [R1.U32+UR4], R10 ;
        /*02e0*/                   STG.E.STRONG.CTA [R2.U32+UR4], R11 ;
        /*02f0*/                   EXIT ;
        /*0300*/                   BRA 0x300;
        /*0310*/                   NOP;
        /*0320*/                   NOP;
        /*0330*/                   NOP;
        /*0340*/                   NOP;
        /*0350*/                   NOP;
        /*0360*/                   NOP;
        /*0370*/                   NOP;
