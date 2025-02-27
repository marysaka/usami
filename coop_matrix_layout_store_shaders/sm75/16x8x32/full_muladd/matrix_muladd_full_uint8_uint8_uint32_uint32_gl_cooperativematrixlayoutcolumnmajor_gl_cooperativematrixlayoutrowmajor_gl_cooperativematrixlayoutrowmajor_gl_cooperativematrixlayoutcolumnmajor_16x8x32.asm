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
        /*00c0*/                   S2R R27, SR_LANEID ;
        /*00d0*/                   SHF.R.U32.HI R4, RZ, 0x2, R27 ;
        /*00e0*/                   LOP3.LUT R5, R27, 0x3, RZ, 0xc0, !PT ;
        /*00f0*/                   IMAD R6, R5, 0x4, R4 ;
        /*0100*/                   STS.U8 [0x300], R1 ;
        /*0110*/                   STS.U8 [0x200], R2 ;
        /*0120*/                   STS [RZ], R3 ;
        /*0130*/                   NOP ;
        /*0140*/                   NOP ;
        /*0150*/                   LDS.U.S8 R14, [R6+0x303] ;
        /*0160*/                   IMAD R29, R4, 0x2, R5 ;
        /*0170*/                   LDS.U.S8 R25, [R6+0x203] ;
        /*0180*/                   LDS.U.S8 R15, [R6+0x30b] ;
        /*0190*/                   LDS.U.S8 R10, [R6+0x302] ;
        /*01a0*/                   LDS.U.S8 R23, [R6+0x202] ;
        /*01b0*/                   LDS.U.S8 R11, [R6+0x30a] ;
        /*01c0*/                   LDS.U.S8 R7, [R6+0x301] ;
        /*01d0*/                   LDS.U.S8 R21, [R6+0x201] ;
        /*01e0*/                   LDS.U.S8 R4, [R6+0x309] ;
        /*01f0*/                   LDS.U.S8 R0, [R6+0x300] ;
        /*0200*/                   LDS.U.S8 R16, [R6+0x313] ;
        /*0210*/                   LOP3.LUT R19, R14, 0xff, RZ, 0xc0, !PT ;
        /*0220*/                   LDS.U.S8 R26, [R6+0x213] ;
        /*0230*/                   LOP3.LUT R28, R25, 0xff, RZ, 0xc0, !PT ;
        /*0240*/                   LDS.U.S8 R17, [R6+0x31b] ;
        /*0250*/                   LOP3.LUT R14, R15, 0xff, RZ, 0xc0, !PT ;
        /*0260*/                   LDS.U.S8 R18, [R6+0x200] ;
        /*0270*/                   PRMT R10, R19, 0x2104, R10 ;
        /*0280*/                   LDS.U.S8 R1, [R6+0x308] ;
        /*0290*/                   PRMT R28, R28, 0x2104, R23 ;
        /*02a0*/                   LDS.U.S8 R12, [R6+0x312] ;
        /*02b0*/                   PRMT R11, R14, 0x2104, R11 ;
        /*02c0*/                   LDS.U.S8 R24, [R6+0x212] ;
        /*02d0*/                   PRMT R7, R10, 0x2104, R7 ;
        /*02e0*/                   LDS.U.S8 R13, [R6+0x31a] ;
        /*02f0*/                   PRMT R21, R28, 0x2104, R21 ;
        /*0300*/                   LDS.U.S8 R8, [R6+0x311] ;
        /*0310*/                   PRMT R4, R11, 0x2104, R4 ;
        /*0320*/                   LDS.U.S8 R22, [R6+0x211] ;
        /*0330*/                   PRMT R0, R7, 0x2104, R0 ;
        /*0340*/                   LDS.U.S8 R9, [R6+0x319] ;
        /*0350*/                   LOP3.LUT R7, R16, 0xff, RZ, 0xc0, !PT ;
        /*0360*/                   LDS.U.64 R30, [R29.X8] ;
        /*0370*/                   LOP3.LUT R11, R26, 0xff, RZ, 0xc0, !PT ;
        /*0380*/                   LDS.U.64 R32, [R29.X8+0x80] ;
        /*0390*/                   LDS.U.S8 R2, [R6+0x310] ;
        /*03a0*/                   PRMT R18, R21, 0x2104, R18 ;
        /*03b0*/                   LDS.U.S8 R20, [R6+0x210] ;
        /*03c0*/                   PRMT R1, R4, 0x2104, R1 ;
        /*03d0*/                   LDS.U.S8 R3, [R6+0x318] ;
        /*03e0*/                   PRMT R7, R7, 0x2104, R12 ;
        /*03f0*/                   PRMT R11, R11, 0x2104, R24 ;
        /*0400*/                   LOP3.LUT R6, R17, 0xff, RZ, 0xc0, !PT ;
        /*0410*/                   PRMT R6, R6, 0x2104, R13 ;
        /*0420*/                   PRMT R7, R7, 0x2104, R8 ;
        /*0430*/                   PRMT R11, R11, 0x2104, R22 ;
        /*0440*/                   PRMT R6, R6, 0x2104, R9 ;
        /*0450*/                   IMMA.8816.U8.U8 R30, R0.ROW, R18.reuse.COL, R30 ;
        /*0460*/                   LOP3.LUT R0, R27, 0xfffffffc, RZ, 0xc0, !PT ;
        /*0470*/                   IMMA.8816.U8.U8 R32, R1.ROW, R18.COL, R32 ;
        /*0480*/                   IMAD R0, R5, 0x20, R0 ;
        /*0490*/                   PRMT R2, R7, 0x2104, R2 ;
        /*04a0*/                   PRMT R11, R11, 0x2104, R20 ;
        /*04b0*/                   IADD3 R1, R0, 0x20, RZ ;
        /*04c0*/                   PRMT R3, R6, 0x2104, R3 ;
        /*04d0*/                   IMMA.8816.U8.U8 R30, R2.ROW, R11.reuse.COL, R30 ;
        /*04e0*/                   IMMA.8816.U8.U8 R32, R3.ROW, R11.COL, R32 ;
        /*04f0*/                   NOP ;
        /*0500*/                   NOP ;
        /*0510*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0520*/                   STG.E.STRONG.CTA [R0.U32+UR4], R30 ;
        /*0530*/                   STG.E.STRONG.CTA [R0.U32+UR4+0x10], R31 ;
        /*0540*/                   STG.E.STRONG.CTA [R1.U32+UR4], R32 ;
        /*0550*/                   STG.E.STRONG.CTA [R1.U32+UR4+0x10], R33 ;
        /*0560*/                   EXIT ;
        /*0570*/                   BRA 0x570;
