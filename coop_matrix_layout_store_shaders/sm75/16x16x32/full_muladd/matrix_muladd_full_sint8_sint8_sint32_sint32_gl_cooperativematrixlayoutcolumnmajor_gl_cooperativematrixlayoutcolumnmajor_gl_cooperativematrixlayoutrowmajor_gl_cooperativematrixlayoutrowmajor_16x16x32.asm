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
        /*00c0*/                   S2R R7, SR_LANEID ;
        /*00d0*/                   SHF.R.U32.HI R4, RZ, 0x2, R7 ;
        /*00e0*/                   LOP3.LUT R5, R7, 0x3, RZ, 0xc0, !PT ;
        /*00f0*/                   IMAD R6, R5, 0x4, R4 ;
        /*0100*/                   STS.U8 [0x600], R1 ;
        /*0110*/                   STS.U8 [0x400], R2 ;
        /*0120*/                   STS [RZ], R3 ;
        /*0130*/                   NOP ;
        /*0140*/                   NOP ;
        /*0150*/                   LDS.U.S8 R17, [R6+0x603] ;
        /*0160*/                   LOP3.LUT R0, R7.reuse, 0xf, RZ, 0xc0, !PT ;
        /*0170*/                   IMAD R21, R4, 0x2, R5 ;
        /*0180*/                   LOP3.LUT R1, R7, 0x10, RZ, 0xc0, !PT ;
        /*0190*/                   LDS.U.S8 R13, [R6+0x602] ;
        /*01a0*/                   LDS.U.S8 R18, [R6+0x60b] ;
        /*01b0*/                   IMAD.IADD R0, R0, 0x1, R1 ;
        /*01c0*/                   LDS.U.S8 R14, [R6+0x60a] ;
        /*01d0*/                   LDS.U.S8 R9, [R6+0x601] ;
        /*01e0*/                   LDS.U.S8 R10, [R6+0x609] ;
        /*01f0*/                   LDS.U.S8 R4, [R6+0x600] ;
        /*0200*/                   LDS.U.S8 R19, [R6+0x613] ;
        /*0210*/                   LDS.U.S8 R20, [R6+0x61b] ;
        /*0220*/                   LDS.U.S8 R5, [R6+0x608] ;
        /*0230*/                   LDS.U.S8 R15, [R6+0x612] ;
        /*0240*/                   LOP3.LUT R22, R17, 0xff, RZ, 0xc0, !PT ;
        /*0250*/                   LDS.U.S8 R16, [R6+0x61a] ;
        /*0260*/                   PRMT R22, R22, 0x2104, R13 ;
        /*0270*/                   LDS.U.S8 R11, [R6+0x611] ;
        /*0280*/                   LOP3.LUT R13, R18, 0xff, RZ, 0xc0, !PT ;
        /*0290*/                   LDS.U.S8 R12, [R6+0x619] ;
        /*02a0*/                   PRMT R13, R13, 0x2104, R14 ;
        /*02b0*/                   LDS.U.64 R24, [R21.X8] ;
        /*02c0*/                   PRMT R9, R22, 0x2104, R9 ;
        /*02d0*/                   LDSM.16.M88.4 R0, [R0+0x400] ;
        /*02e0*/                   PRMT R10, R13, 0x2104, R10 ;
        /*02f0*/                   PRMT R4, R9, 0x2104, R4 ;
        /*0300*/                   LDS.U.64 R28, [R21.X8+0x20] ;
        /*0310*/                   LOP3.LUT R14, R19, 0xff, RZ, 0xc0, !PT ;
        /*0320*/                   LDS.U.64 R30, [R21.X8+0xa0] ;
        /*0330*/                   LOP3.LUT R9, R20, 0xff, RZ, 0xc0, !PT ;
        /*0340*/                   LDS.U.64 R26, [R21.X8+0x80] ;
        /*0350*/                   PRMT R5, R10, 0x2104, R5 ;
        /*0360*/                   LDS.U.S8 R7, [R6+0x610] ;
        /*0370*/                   PRMT R14, R14, 0x2104, R15 ;
        /*0380*/                   LDS.U.S8 R8, [R6+0x618] ;
        /*0390*/                   PRMT R9, R9, 0x2104, R16 ;
        /*03a0*/                   PRMT R14, R14, 0x2104, R11 ;
        /*03b0*/                   PRMT R9, R9, 0x2104, R12 ;
        /*03c0*/                   IMMA.8816.S8.S8 R24, R4.reuse.ROW, R0.reuse.COL, R24 ;
        /*03d0*/                   IMMA.8816.S8.S8 R28, R4.ROW, R1.reuse.COL, R28 ;
        /*03e0*/                   IMMA.8816.S8.S8 R30, R5.reuse.ROW, R1.COL, R30 ;
        /*03f0*/                   IMMA.8816.S8.S8 R26, R5.ROW, R0.COL, R26 ;
        /*0400*/                   IMAD.SHL.U32 R5, R21, 0x8, RZ ;
        /*0410*/                   PRMT R7, R14, 0x2104, R7 ;
        /*0420*/                   PRMT R8, R9, 0x2104, R8 ;
        /*0430*/                   IADD3 R9, R5, 0xa0, RZ ;
        /*0440*/                   IMMA.8816.S8.S8 R0, R7.reuse.ROW, R2.COL, R24 ;
        /*0450*/                   IMMA.8816.S8.S8 R28, R7.ROW, R3.reuse.COL, R28 ;
        /*0460*/                   IADD3 R7, R5.reuse, 0x20, RZ ;
        /*0470*/                   IMMA.8816.S8.S8 R30, R8.ROW, R3.COL, R30 ;
        /*0480*/                   IADD3 R3, R5, 0x80, RZ ;
        /*0490*/                   IMMA.8816.S8.S8 R26, R8.ROW, R2.COL, R26 ;
        /*04a0*/                   NOP ;
        /*04b0*/                   NOP ;
        /*04c0*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*04d0*/                   IMAD.MOV.U32 R2, RZ, RZ, R30 ;
        /*04e0*/                   STG.E.64.STRONG.CTA [R5.U32+UR4], R0 ;
        /*04f0*/                   IMAD.MOV.U32 R0, RZ, RZ, R26 ;
        /*0500*/                   IMAD.MOV.U32 R1, RZ, RZ, R27 ;
        /*0510*/                   STG.E.64.STRONG.CTA [R3.U32+UR4], R0 ;
        /*0520*/                   IMAD.MOV.U32 R0, RZ, RZ, R28 ;
        /*0530*/                   IMAD.MOV.U32 R1, RZ, RZ, R29 ;
        /*0540*/                   IMAD.MOV.U32 R3, RZ, RZ, R31 ;
        /*0550*/                   STG.E.64.STRONG.CTA [R7.U32+UR4], R0 ;
        /*0560*/                   STG.E.64.STRONG.CTA [R9.U32+UR4], R2 ;
        /*0570*/                   EXIT ;
        /*0580*/                   BRA 0x580;
        /*0590*/                   NOP;
        /*05a0*/                   NOP;
        /*05b0*/                   NOP;
        /*05c0*/                   NOP;
        /*05d0*/                   NOP;
        /*05e0*/                   NOP;
        /*05f0*/                   NOP;
