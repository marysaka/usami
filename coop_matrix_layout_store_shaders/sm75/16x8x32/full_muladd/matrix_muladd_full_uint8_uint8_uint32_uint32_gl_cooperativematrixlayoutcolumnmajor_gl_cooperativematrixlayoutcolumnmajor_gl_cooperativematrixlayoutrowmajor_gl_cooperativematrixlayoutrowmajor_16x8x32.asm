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
        /*0100*/                   STS.U8 [0x300], R1 ;
        /*0110*/                   STS.U8 [0x200], R2 ;
        /*0120*/                   STS [RZ], R3 ;
        /*0130*/                   NOP ;
        /*0140*/                   NOP ;
        /*0150*/                   LDS.U.S8 R13, [R6+0x303] ;
        /*0160*/                   LOP3.LUT R0, R7.reuse, 0x7, RZ, 0xc0, !PT ;
        /*0170*/                   IMAD R19, R4, 0x2, R5 ;
        /*0180*/                   LOP3.LUT R1, R7, 0x8, RZ, 0xc0, !PT ;
        /*0190*/                   LDS.U.S8 R14, [R6+0x30b] ;
        /*01a0*/                   LDS.U.S8 R9, [R6+0x302] ;
        /*01b0*/                   IMAD R17, R1, 0x2, R0 ;
        /*01c0*/                   LDS.U.S8 R10, [R6+0x30a] ;
        /*01d0*/                   LDS.U.S8 R4, [R6+0x301] ;
        /*01e0*/                   LDS.U.S8 R5, [R6+0x309] ;
        /*01f0*/                   LDS.U.S8 R15, [R6+0x313] ;
        /*0200*/                   LDS.U.S8 R16, [R6+0x31b] ;
        /*0210*/                   LDS.U.S8 R0, [R6+0x300] ;
        /*0220*/                   LDS.U.S8 R1, [R6+0x308] ;
        /*0230*/                   LDS.U.S8 R11, [R6+0x312] ;
        /*0240*/                   LOP3.LUT R18, R13, 0xff, RZ, 0xc0, !PT ;
        /*0250*/                   LDS.U.S8 R12, [R6+0x31a] ;
        /*0260*/                   LOP3.LUT R13, R14, 0xff, RZ, 0xc0, !PT ;
        /*0270*/                   LDS.U.S8 R7, [R6+0x311] ;
        /*0280*/                   PRMT R9, R18, 0x2104, R9 ;
        /*0290*/                   LDS.U.S8 R8, [R6+0x319] ;
        /*02a0*/                   PRMT R10, R13, 0x2104, R10 ;
        /*02b0*/                   LDSM.16.M88.2 R20, [R17+0x200] ;
        /*02c0*/                   PRMT R9, R9, 0x2104, R4 ;
        /*02d0*/                   LDS.U.64 R22, [R19.X8] ;
        /*02e0*/                   PRMT R10, R10, 0x2104, R5 ;
        /*02f0*/                   LDS.U.64 R24, [R19.X8+0x80] ;
        /*0300*/                   LOP3.LUT R4, R15, 0xff, RZ, 0xc0, !PT ;
        /*0310*/                   LDS.U.S8 R2, [R6+0x310] ;
        /*0320*/                   LOP3.LUT R5, R16, 0xff, RZ, 0xc0, !PT ;
        /*0330*/                   LDS.U.S8 R3, [R6+0x318] ;
        /*0340*/                   PRMT R0, R9, 0x2104, R0 ;
        /*0350*/                   PRMT R1, R10, 0x2104, R1 ;
        /*0360*/                   PRMT R4, R4, 0x2104, R11 ;
        /*0370*/                   PRMT R5, R5, 0x2104, R12 ;
        /*0380*/                   PRMT R7, R4, 0x2104, R7 ;
        /*0390*/                   PRMT R8, R5, 0x2104, R8 ;
        /*03a0*/                   IMAD.SHL.U32 R5, R19, 0x8, RZ ;
        /*03b0*/                   IMMA.8816.U8.U8 R22, R0.ROW, R20.reuse.COL, R22 ;
        /*03c0*/                   IMMA.8816.U8.U8 R24, R1.ROW, R20.COL, R24 ;
        /*03d0*/                   PRMT R2, R7, 0x2104, R2 ;
        /*03e0*/                   IADD3 R7, R5, 0x80, RZ ;
        /*03f0*/                   PRMT R3, R8, 0x2104, R3 ;
        /*0400*/                   IMMA.8816.U8.U8 R0, R2.ROW, R21.reuse.COL, R22 ;
        /*0410*/                   IMMA.8816.U8.U8 R2, R3.ROW, R21.COL, R24 ;
        /*0420*/                   NOP ;
        /*0430*/                   NOP ;
        /*0440*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0450*/                   STG.E.64.STRONG.CTA [R5.U32+UR4], R0 ;
        /*0460*/                   STG.E.64.STRONG.CTA [R7.U32+UR4], R2 ;
        /*0470*/                   EXIT ;
        /*0480*/                   BRA 0x480;
        /*0490*/                   NOP;
        /*04a0*/                   NOP;
        /*04b0*/                   NOP;
        /*04c0*/                   NOP;
        /*04d0*/                   NOP;
        /*04e0*/                   NOP;
        /*04f0*/                   NOP;
