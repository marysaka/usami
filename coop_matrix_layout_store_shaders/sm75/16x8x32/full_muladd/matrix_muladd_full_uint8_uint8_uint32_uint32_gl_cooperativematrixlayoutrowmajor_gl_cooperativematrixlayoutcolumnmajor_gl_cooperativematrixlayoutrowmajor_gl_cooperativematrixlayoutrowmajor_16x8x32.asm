        /*0000*/                   S2R R0, SR_CTAID.X ;
        /*0010*/                   IADD3 R0, R0, c[0x0][0x20], RZ ;
        /*0020*/                   ISETP.GE.U32.AND P0, PT, R0.reuse, c[0x0][0x48], PT ;
        /*0030*/                   IMAD.SHL.U32 R2, R0.reuse, 0x4, RZ ;
        /*0040*/                   ISETP.GE.U32.AND P1, PT, R0, c[0x0][0x58], PT ;
        /*0050*/                   ISETP.GE.U32.AND P2, PT, R2, c[0x0][0x68], PT ;
        /*0060*/                   ULDC.64 UR4, c[0x0][0x40] ;
        /*0070*/                   ULDC.64 UR6, c[0x0][0x50] ;
        /*0080*/                   LDG.E.U8.CONSTANT.SYS R9, [R0.U32+UR4], !P0 ;
        /*0090*/                   ULDC.64 UR8, c[0x0][0x60] ;
        /*00a0*/                   LDG.E.U8.CONSTANT.SYS R1, [R0.U32+UR6], !P1 ;
        /*00b0*/                   LDG.E.CONSTANT.SYS R2, [R2.U32+UR8], !P2 ;
        /*00c0*/                   S2R R8, SR_LANEID ;
        /*00d0*/                   LOP3.LUT R3, R8, 0xf, RZ, 0xc0, !PT ;
        /*00e0*/                   LOP3.LUT R4, R8.reuse, 0x10, RZ, 0xc0, !PT ;
        /*00f0*/                   LOP3.LUT R5, R8.reuse, 0x7, RZ, 0xc0, !PT ;
        /*0100*/                   LOP3.LUT R6, R8.reuse, 0x8, RZ, 0xc0, !PT ;
        /*0110*/                   SHF.R.U32.HI R7, RZ, 0x2, R8 ;
        /*0120*/                   LOP3.LUT R8, R8, 0x3, RZ, 0xc0, !PT ;
        /*0130*/                   IMAD.IADD R3, R3, 0x1, R4 ;
        /*0140*/                   IMAD R5, R6, 0x2, R5 ;
        /*0150*/                   IMAD R12, R7, 0x2, R8 ;
        /*0160*/                   STS.U8 [0x300], R9 ;
        /*0170*/                   STS.U8 [0x200], R1 ;
        /*0180*/                   STS [RZ], R2 ;
        /*0190*/                   NOP ;
        /*01a0*/                   NOP ;
        /*01b0*/                   LDSM.16.M88.2 R4, [R5+0x200] ;
        /*01c0*/                   LDS.U.64 R6, [R12.X8] ;
        /*01d0*/                   LDSM.16.M88.4 R0, [R3+0x300] ;
        /*01e0*/                   LDS.U.64 R8, [R12.X8+0x80] ;
        /*01f0*/                   IMMA.8816.U8.U8 R6, R0.ROW, R4.reuse.COL, R6 ;
        /*0200*/                   IMMA.8816.U8.U8 R10, R1.ROW, R4.COL, R8 ;
        /*0210*/                   IMMA.8816.U8.U8 R0, R2.ROW, R5.COL, R6 ;
        /*0220*/                   IMAD.SHL.U32 R7, R12, 0x8, RZ ;
        /*0230*/                   IMMA.8816.U8.U8 R2, R3.ROW, R5.COL, R10 ;
        /*0240*/                   IADD3 R9, R7, 0x80, RZ ;
        /*0250*/                   NOP ;
        /*0260*/                   NOP ;
        /*0270*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0280*/                   STG.E.64.STRONG.CTA [R7.U32+UR4], R0 ;
        /*0290*/                   STG.E.64.STRONG.CTA [R9.U32+UR4], R2 ;
        /*02a0*/                   EXIT ;
        /*02b0*/                   BRA 0x2b0;
        /*02c0*/                   NOP;
        /*02d0*/                   NOP;
        /*02e0*/                   NOP;
        /*02f0*/                   NOP;
