        /*0000*/                   S2R R0, SR_CTAID.X ;
        /*0010*/                   IADD3 R0, R0, c[0x0][0x20], RZ ;
        /*0020*/                   ISETP.GE.U32.AND P0, PT, R0.reuse, c[0x0][0x48], PT ;
        /*0030*/                   IMAD.SHL.U32 R2, R0.reuse, 0x4, RZ ;
        /*0040*/                   ISETP.GE.U32.AND P1, PT, R0, c[0x0][0x58], PT ;
        /*0050*/                   ISETP.GE.U32.AND P2, PT, R2, c[0x0][0x68], PT ;
        /*0060*/                   ULDC.64 UR4, c[0x0][0x40] ;
        /*0070*/                   ULDC.64 UR6, c[0x0][0x50] ;
        /*0080*/                   LDG.E.U8.CONSTANT.SYS R7, [R0.U32+UR4], !P0 ;
        /*0090*/                   ULDC.64 UR8, c[0x0][0x60] ;
        /*00a0*/                   LDG.E.U8.CONSTANT.SYS R1, [R0.U32+UR6], !P1 ;
        /*00b0*/                   LDG.E.CONSTANT.SYS R2, [R2.U32+UR8], !P2 ;
        /*00c0*/                   S2R R6, SR_LANEID ;
        /*00d0*/                   LOP3.LUT R3, R6, 0xf, RZ, 0xc0, !PT ;
        /*00e0*/                   LOP3.LUT R4, R6.reuse, 0x10, RZ, 0xc0, !PT ;
        /*00f0*/                   LOP3.LUT R5, R6.reuse, 0xfffffffc, RZ, 0xc0, !PT ;
        /*0100*/                   LOP3.LUT R6, R6, 0x3, RZ, 0xc0, !PT ;
        /*0110*/                   IMAD.IADD R3, R3, 0x1, R4 ;
        /*0120*/                   IMAD R12, R6, 0x20, R5 ;
        /*0130*/                   STS.U8 [0x600], R7 ;
        /*0140*/                   STS.U8 [0x400], R1 ;
        /*0150*/                   STS [RZ], R2 ;
        /*0160*/                   NOP ;
        /*0170*/                   NOP ;
        /*0180*/                   LDS.U R0, [R12] ;
        /*0190*/                   IADD3 R2, R12, 0xa0, RZ ;
        /*01a0*/                   LDS.U R1, [R12+0x10] ;
        /*01b0*/                   LDSM.16.M88.4 R4, [R3+0x600] ;
        /*01c0*/                   LDSM.16.M88.4 R8, [R3+0x400] ;
        /*01d0*/                   LDS.U R14, [R12+0x20] ;
        /*01e0*/                   LDS.U R15, [R12+0x30] ;
        /*01f0*/                   LDS.U R16, [R12+0x80] ;
        /*0200*/                   LDS.U R17, [R12+0x90] ;
        /*0210*/                   LDS.U R18, [R12+0xa0] ;
        /*0220*/                   LDS.U R19, [R12+0xb0] ;
        /*0230*/                   IMMA.8816.S8.S8 R0, R4.reuse.ROW, R8.reuse.COL, R0 ;
        /*0240*/                   IMMA.8816.S8.S8 R14, R5.ROW, R8.COL, R14 ;
        /*0250*/                   IMMA.8816.S8.S8 R16, R4.ROW, R9.reuse.COL, R16 ;
        /*0260*/                   IMMA.8816.S8.S8 R14, R7.ROW, R10.reuse.COL, R14 ;
        /*0270*/                   IMMA.8816.S8.S8 R18, R5.ROW, R9.COL, R18 ;
        /*0280*/                   IMMA.8816.S8.S8 R4, R6.ROW, R10.COL, R0 ;
        /*0290*/                   IADD3 R0, R12, 0x20, RZ ;
        /*02a0*/                   IMMA.8816.S8.S8 R16, R6.ROW, R11.COL, R16 ;
        /*02b0*/                   IADD3 R1, R12, 0x80, RZ ;
        /*02c0*/                   IMMA.8816.S8.S8 R18, R7.ROW, R11.COL, R18 ;
        /*02d0*/                   NOP ;
        /*02e0*/                   NOP ;
        /*02f0*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0300*/                   STG.E.STRONG.CTA [R12.U32+UR4], R4 ;
        /*0310*/                   STG.E.STRONG.CTA [R12.U32+UR4+0x10], R5 ;
        /*0320*/                   STG.E.STRONG.CTA [R0.U32+UR4], R14 ;
        /*0330*/                   STG.E.STRONG.CTA [R0.U32+UR4+0x10], R15 ;
        /*0340*/                   STG.E.STRONG.CTA [R1.U32+UR4], R16 ;
        /*0350*/                   STG.E.STRONG.CTA [R1.U32+UR4+0x10], R17 ;
        /*0360*/                   STG.E.STRONG.CTA [R2.U32+UR4], R18 ;
        /*0370*/                   STG.E.STRONG.CTA [R2.U32+UR4+0x10], R19 ;
        /*0380*/                   EXIT ;
        /*0390*/                   BRA 0x390;
        /*03a0*/                   NOP;
        /*03b0*/                   NOP;
        /*03c0*/                   NOP;
        /*03d0*/                   NOP;
        /*03e0*/                   NOP;
        /*03f0*/                   NOP;
