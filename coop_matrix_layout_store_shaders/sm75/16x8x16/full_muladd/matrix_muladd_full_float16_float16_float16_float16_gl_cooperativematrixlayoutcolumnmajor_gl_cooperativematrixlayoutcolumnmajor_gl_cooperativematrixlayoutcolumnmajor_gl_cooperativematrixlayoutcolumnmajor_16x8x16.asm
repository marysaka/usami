        /*0000*/                   S2R R0, SR_CTAID.X ;
        /*0010*/                   IADD3 R0, R0, c[0x0][0x20], RZ ;
        /*0020*/                   IMAD.SHL.U32 R0, R0, 0x2, RZ ;
        /*0030*/                   ISETP.GE.U32.AND P0, PT, R0.reuse, c[0x0][0x48], PT ;
        /*0040*/                   ISETP.GE.U32.AND P1, PT, R0.reuse, c[0x0][0x58], PT ;
        /*0050*/                   ISETP.GE.U32.AND P2, PT, R0, c[0x0][0x68], PT ;
        /*0060*/                   ULDC.64 UR4, c[0x0][0x40] ;
        /*0070*/                   ULDC.64 UR6, c[0x0][0x50] ;
        /*0080*/                   LDG.E.U16.CONSTANT.SYS R7, [R0.U32+UR4], !P0 ;
        /*0090*/                   ULDC.64 UR8, c[0x0][0x60] ;
        /*00a0*/                   LDG.E.U16.CONSTANT.SYS R1, [R0.U32+UR6], !P1 ;
        /*00b0*/                   LDG.E.U16.CONSTANT.SYS R2, [R0.U32+UR8], !P2 ;
        /*00c0*/                   S2R R11, SR_LANEID ;
        /*00d0*/                   LOP3.LUT R3, R11, 0x8, RZ, 0xc0, !PT ;
        /*00e0*/                   IMAD.SHL.U32 R4, R3, 0x2, RZ ;
        /*00f0*/                   LOP3.LUT R3, R11, 0x10, RZ, 0xc0, !PT ;
        /*0100*/                   LOP3.LUT R6, R4, 0x7, R11, 0xf8, !PT ;
        /*0110*/                   LEA.HI R3, R3, R6, RZ, 0x1f ;
        /*0120*/                   LOP3.LUT R5, R11, 0x7, R4, 0xc8, !PT ;
        /*0130*/                   LOP3.LUT R6, R4, 0x10, RZ, 0xc0, !PT ;
        /*0140*/                   LOP3.LUT R4, R3.reuse, 0xf, RZ, 0xc0, !PT ;
        /*0150*/                   LOP3.LUT R3, R3, 0x10, RZ, 0xc0, !PT ;
        /*0160*/                   IMAD R5, R5, 0x4, R6 ;
        /*0170*/                   IMAD R3, R4, 0x4, R3 ;
        /*0180*/                   STS.U16 [0x200], R7 ;
        /*0190*/                   STS.U16 [0x100], R1 ;
        /*01a0*/                   STS.U16 [RZ], R2 ;
        /*01b0*/                   NOP ;
        /*01c0*/                   NOP ;
        /*01d0*/                   LDSM.16.M88.2 R6, [R5+0x100] ;
        /*01e0*/                   LDSM.16.MT88.2 R8, [R5] ;
        /*01f0*/                   LDSM.16.MT88.4 R0, [R3+0x200] ;
        /*0200*/                   HMMA.16816.F16 R6, R0, R6, R8 ;
        /*0210*/                   NOP ;
        /*0220*/                   NOP ;
        /*0230*/                   NOP ;
        /*0240*/                   MOVM.16.MT88 R3, R6 ;
        /*0250*/                   LOP3.LUT R0, R11.reuse, 0xfffffffc, RZ, 0xc0, !PT ;
        /*0260*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0270*/                   LOP3.LUT R1, R11, 0x3, RZ, 0xc0, !PT ;
        /*0280*/                   MOVM.16.MT88 R2, R7 ;
        /*0290*/                   IMAD R0, R1, 0x4, R0 ;
        /*02a0*/                   IADD3 R1, R0, 0x10, RZ ;
        /*02b0*/                   STG.E.STRONG.CTA [R0.U32+UR4], R3 ;
        /*02c0*/                   STG.E.STRONG.CTA [R1.U32+UR4], R2 ;
        /*02d0*/                   EXIT ;
        /*02e0*/                   BRA 0x2e0;
        /*02f0*/                   NOP;
