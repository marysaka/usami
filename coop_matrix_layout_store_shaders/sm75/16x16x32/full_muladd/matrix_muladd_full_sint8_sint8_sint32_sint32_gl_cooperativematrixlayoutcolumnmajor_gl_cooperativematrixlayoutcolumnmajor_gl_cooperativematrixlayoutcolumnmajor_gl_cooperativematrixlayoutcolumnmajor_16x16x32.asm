        /*0000*/                   S2R R0, SR_CTAID.X ;
        /*0010*/                   IADD3 R0, R0, c[0x0][0x20], RZ ;
        /*0020*/                   ISETP.GE.U32.AND P0, PT, R0.reuse, c[0x0][0x48], PT ;
        /*0030*/                   IMAD.SHL.U32 R3, R0.reuse, 0x4, RZ ;
        /*0040*/                   ISETP.GE.U32.AND P1, PT, R0, c[0x0][0x58], PT ;
        /*0050*/                   ISETP.GE.U32.AND P2, PT, R3, c[0x0][0x68], PT ;
        /*0060*/                   ULDC.64 UR4, c[0x0][0x40] ;
        /*0070*/                   ULDC.64 UR6, c[0x0][0x50] ;
        /*0080*/                   LDG.E.U8.CONSTANT.SYS R1, [R0.U32+UR4], !P0 ;
        /*0090*/                   ULDC.64 UR8, c[0x0][0x60] ;
        /*00a0*/                   LDG.E.U8.CONSTANT.SYS R2, [R0.U32+UR6], !P1 ;
        /*00b0*/                   LDG.E.CONSTANT.SYS R3, [R3.U32+UR8], !P2 ;
        /*00c0*/                   S2R R6, SR_LANEID ;
        /*00d0*/                   LOP3.LUT R4, R6, 0x3, RZ, 0xc0, !PT ;
        /*00e0*/                   IMAD.SHL.U32 R5, R4, 0x4, RZ ;
        /*00f0*/                   LEA.HI R5, R6, R5, RZ, 0x1e ;
        /*0100*/                   STS.U8 [0x600], R1 ;
        /*0110*/                   STS.U8 [0x400], R2 ;
        /*0120*/                   STS [RZ], R3 ;
        /*0130*/                   NOP ;
        /*0140*/                   NOP ;
        /*0150*/                   LDS.U.S8 R17, [R5+0x603] ;
        /*0160*/                   LOP3.LUT R3, R6.reuse, 0xfffffffc, RZ, 0xc0, !PT ;
        /*0170*/                   LOP3.LUT R0, R6.reuse, 0xf, RZ, 0xc0, !PT ;
        /*0180*/                   LDS.U.S8 R13, [R5+0x602] ;
        /*0190*/                   LOP3.LUT R1, R6, 0x10, RZ, 0xc0, !PT ;
        /*01a0*/                   IMAD R21, R4, 0x20, R3 ;
        /*01b0*/                   LDS.U.S8 R18, [R5+0x60b] ;
        /*01c0*/                   IMAD.IADD R0, R0, 0x1, R1 ;
        /*01d0*/                   LDS.U.S8 R14, [R5+0x60a] ;
        /*01e0*/                   LDS.U.S8 R9, [R5+0x601] ;
        /*01f0*/                   LDS.U.S8 R10, [R5+0x609] ;
        /*0200*/                   LDS.U.S8 R19, [R5+0x613] ;
        /*0210*/                   LDS.U.S8 R20, [R5+0x61b] ;
        /*0220*/                   LDS.U.S8 R4, [R5+0x600] ;
        /*0230*/                   LDS.U.S8 R6, [R5+0x608] ;
        /*0240*/                   LDS.U.S8 R15, [R5+0x612] ;
        /*0250*/                   LOP3.LUT R22, R17, 0xff, RZ, 0xc0, !PT ;
        /*0260*/                   LDS.U.S8 R16, [R5+0x61a] ;
        /*0270*/                   PRMT R22, R22, 0x2104, R13 ;
        /*0280*/                   LDS.U.S8 R11, [R5+0x611] ;
        /*0290*/                   LOP3.LUT R13, R18, 0xff, RZ, 0xc0, !PT ;
        /*02a0*/                   LDS.U.S8 R12, [R5+0x619] ;
        /*02b0*/                   PRMT R13, R13, 0x2104, R14 ;
        /*02c0*/                   LDS.U R24, [R21] ;
        /*02d0*/                   PRMT R9, R22, 0x2104, R9 ;
        /*02e0*/                   LDS.U R25, [R21+0x10] ;
        /*02f0*/                   PRMT R13, R13, 0x2104, R10 ;
        /*0300*/                   LDSM.16.M88.4 R0, [R0+0x400] ;
        /*0310*/                   LOP3.LUT R10, R19, 0xff, RZ, 0xc0, !PT ;
        /*0320*/                   LDS.U R26, [R21+0x20] ;
        /*0330*/                   PRMT R4, R9, 0x2104, R4 ;
        /*0340*/                   LDS.U R27, [R21+0x30] ;
        /*0350*/                   PRMT R6, R13, 0x2104, R6 ;
        /*0360*/                   LDS.U R28, [R21+0x80] ;
        /*0370*/                   PRMT R10, R10, 0x2104, R15 ;
        /*0380*/                   LDS.U R29, [R21+0x90] ;
        /*0390*/                   LDS.U R30, [R21+0xa0] ;
        /*03a0*/                   PRMT R10, R10, 0x2104, R11 ;
        /*03b0*/                   LDS.U R31, [R21+0xb0] ;
        /*03c0*/                   LDS.U.S8 R7, [R5+0x610] ;
        /*03d0*/                   LDS.U.S8 R8, [R5+0x618] ;
        /*03e0*/                   IMMA.8816.S8.S8 R24, R4.ROW, R0.COL, R24 ;
        /*03f0*/                   LOP3.LUT R5, R20, 0xff, RZ, 0xc0, !PT ;
        /*0400*/                   PRMT R5, R5, 0x2104, R16 ;
        /*0410*/                   PRMT R5, R5, 0x2104, R12 ;
        /*0420*/                   IMMA.8816.S8.S8 R26, R6.ROW, R0.COL, R26 ;
        /*0430*/                   IADD3 R0, R21.reuse, 0x20, RZ ;
        /*0440*/                   IMMA.8816.S8.S8 R28, R4.ROW, R1.reuse.COL, R28 ;
        /*0450*/                   IMMA.8816.S8.S8 R30, R6.ROW, R1.COL, R30 ;
        /*0460*/                   IADD3 R1, R21, 0x80, RZ ;
        /*0470*/                   PRMT R7, R10, 0x2104, R7 ;
        /*0480*/                   PRMT R5, R5, 0x2104, R8 ;
        /*0490*/                   IMMA.8816.S8.S8 R24, R7.ROW, R2.reuse.COL, R24 ;
        /*04a0*/                   IMMA.8816.S8.S8 R26, R5.ROW, R2.COL, R26 ;
        /*04b0*/                   IADD3 R2, R21, 0xa0, RZ ;
        /*04c0*/                   IMMA.8816.S8.S8 R28, R7.ROW, R3.reuse.COL, R28 ;
        /*04d0*/                   IMMA.8816.S8.S8 R30, R5.ROW, R3.COL, R30 ;
        /*04e0*/                   NOP ;
        /*04f0*/                   NOP ;
        /*0500*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0510*/                   STG.E.STRONG.CTA [R21.U32+UR4], R24 ;
        /*0520*/                   STG.E.STRONG.CTA [R21.U32+UR4+0x10], R25 ;
        /*0530*/                   STG.E.STRONG.CTA [R0.U32+UR4], R26 ;
        /*0540*/                   STG.E.STRONG.CTA [R0.U32+UR4+0x10], R27 ;
        /*0550*/                   STG.E.STRONG.CTA [R1.U32+UR4], R28 ;
        /*0560*/                   STG.E.STRONG.CTA [R1.U32+UR4+0x10], R29 ;
        /*0570*/                   STG.E.STRONG.CTA [R2.U32+UR4], R30 ;
        /*0580*/                   STG.E.STRONG.CTA [R2.U32+UR4+0x10], R31 ;
        /*0590*/                   EXIT ;
        /*05a0*/                   BRA 0x5a0;
        /*05b0*/                   NOP;
        /*05c0*/                   NOP;
        /*05d0*/                   NOP;
        /*05e0*/                   NOP;
        /*05f0*/                   NOP;
