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
        /*00c0*/                   S2R R5, SR_LANEID ;
        /*00d0*/                   SHF.R.U32.HI R4, RZ, 0x2, R5 ;
        /*00e0*/                   LOP3.LUT R5, R5, 0x3, RZ, 0xc0, !PT ;
        /*00f0*/                   IMAD R6, R5, 0x4, R4 ;
        /*0100*/                   STS.U8 [0x600], R1 ;
        /*0110*/                   STS.U8 [0x400], R2 ;
        /*0120*/                   STS [RZ], R3 ;
        /*0130*/                   NOP ;
        /*0140*/                   NOP ;
        /*0150*/                   LDS.U.S8 R0, [R6+0x603] ;
        /*0160*/                   IMAD R40, R4, 0x2, R5 ;
        /*0170*/                   LDS.U.S8 R2, [R6+0x602] ;
        /*0180*/                   LDS.U.S8 R11, [R6+0x601] ;
        /*0190*/                   LDS.U.S8 R7, [R6+0x600] ;
        /*01a0*/                   LDS.U.S8 R34, [R6+0x403] ;
        /*01b0*/                   LDS.U.S8 R18, [R6+0x60b] ;
        /*01c0*/                   LDS.U.S8 R35, [R6+0x40b] ;
        /*01d0*/                   LDS.U.S8 R15, [R6+0x60a] ;
        /*01e0*/                   LDS.U.S8 R30, [R6+0x402] ;
        /*01f0*/                   LDS.U.S8 R31, [R6+0x40a] ;
        /*0200*/                   LDS.U.S8 R12, [R6+0x609] ;
        /*0210*/                   LOP3.LUT R3, R0, 0xff, RZ, 0xc0, !PT ;
        /*0220*/                   LDS.U.S8 R26, [R6+0x401] ;
        /*0230*/                   PRMT R22, R3, 0x2104, R2 ;
        /*0240*/                   LDS.U.S8 R27, [R6+0x409] ;
        /*0250*/                   PRMT R22, R22, 0x2104, R11 ;
        /*0260*/                   LDS.U.S8 R19, [R6+0x613] ;
        /*0270*/                   PRMT R7, R22, 0x2104, R7 ;
        /*0280*/                   LDS.U.S8 R20, [R6+0x61b] ;
        /*0290*/                   LOP3.LUT R11, R34, 0xff, RZ, 0xc0, !PT ;
        /*02a0*/                   LDS.U.S8 R36, [R6+0x413] ;
        /*02b0*/                   LOP3.LUT R18, R18, 0xff, RZ, 0xc0, !PT ;
        /*02c0*/                   LDS.U.S8 R37, [R6+0x41b] ;
        /*02d0*/                   LOP3.LUT R22, R35, 0xff, RZ, 0xc0, !PT ;
        /*02e0*/                   LDS.U.S8 R8, [R6+0x608] ;
        /*02f0*/                   PRMT R15, R18, 0x2104, R15 ;
        /*0300*/                   LDS.U.S8 R21, [R6+0x400] ;
        /*0310*/                   PRMT R11, R11, 0x2104, R30 ;
        /*0320*/                   LDS.U.S8 R23, [R6+0x408] ;
        /*0330*/                   PRMT R22, R22, 0x2104, R31 ;
        /*0340*/                   LDS.U.S8 R16, [R6+0x612] ;
        /*0350*/                   PRMT R15, R15, 0x2104, R12 ;
        /*0360*/                   LDS.U.S8 R17, [R6+0x61a] ;
        /*0370*/                   PRMT R26, R11, 0x2104, R26 ;
        /*0380*/                   LDS.U.S8 R32, [R6+0x412] ;
        /*0390*/                   PRMT R22, R22, 0x2104, R27 ;
        /*03a0*/                   LDS.U.S8 R33, [R6+0x41a] ;
        /*03b0*/                   LOP3.LUT R19, R19, 0xff, RZ, 0xc0, !PT ;
        /*03c0*/                   LDS.U.S8 R13, [R6+0x611] ;
        /*03d0*/                   LOP3.LUT R20, R20, 0xff, RZ, 0xc0, !PT ;
        /*03e0*/                   LDS.U.S8 R14, [R6+0x619] ;
        /*03f0*/                   LOP3.LUT R11, R36, 0xff, RZ, 0xc0, !PT ;
        /*0400*/                   LDS.U.S8 R28, [R6+0x411] ;
        /*0410*/                   LDS.U.S8 R29, [R6+0x419] ;
        /*0420*/                   PRMT R8, R15, 0x2104, R8 ;
        /*0430*/                   LDS.U.64 R38, [R40.X8] ;
        /*0440*/                   PRMT R21, R26, 0x2104, R21 ;
        /*0450*/                   LDS.U.64 R4, [R40.X8+0x80] ;
        /*0460*/                   PRMT R22, R22, 0x2104, R23 ;
        /*0470*/                   LDS.U.64 R0, [R40.X8+0x20] ;
        /*0480*/                   PRMT R16, R19, 0x2104, R16 ;
        /*0490*/                   LDS.U.64 R2, [R40.X8+0xa0] ;
        /*04a0*/                   PRMT R17, R20, 0x2104, R17 ;
        /*04b0*/                   LDS.U.S8 R9, [R6+0x610] ;
        /*04c0*/                   PRMT R11, R11, 0x2104, R32 ;
        /*04d0*/                   LDS.U.S8 R24, [R6+0x410] ;
        /*04e0*/                   LDS.U.S8 R10, [R6+0x618] ;
        /*04f0*/                   PRMT R16, R16, 0x2104, R13 ;
        /*0500*/                   LDS.U.S8 R25, [R6+0x418] ;
        /*0510*/                   PRMT R17, R17, 0x2104, R14 ;
        /*0520*/                   PRMT R11, R11, 0x2104, R28 ;
        /*0530*/                   LOP3.LUT R6, R37, 0xff, RZ, 0xc0, !PT ;
        /*0540*/                   PRMT R6, R6, 0x2104, R33 ;
        /*0550*/                   IMMA.8816.S8.S8 R38, R7.reuse.ROW, R21.reuse.COL, R38 ;
        /*0560*/                   PRMT R6, R6, 0x2104, R29 ;
        /*0570*/                   IMMA.8816.S8.S8 R4, R8.reuse.ROW, R21.COL, R4 ;
        /*0580*/                   IMMA.8816.S8.S8 R14, R7.ROW, R22.reuse.COL, R0 ;
        /*0590*/                   IMMA.8816.S8.S8 R22, R8.ROW, R22.COL, R2 ;
        /*05a0*/                   IMAD.SHL.U32 R3, R40, 0x8, RZ ;
        /*05b0*/                   PRMT R9, R16, 0x2104, R9 ;
        /*05c0*/                   IADD3 R7, R3, 0x20, RZ ;
        /*05d0*/                   PRMT R11, R11, 0x2104, R24 ;
        /*05e0*/                   PRMT R10, R17, 0x2104, R10 ;
        /*05f0*/                   PRMT R6, R6, 0x2104, R25 ;
        /*0600*/                   IMMA.8816.S8.S8 R0, R9.reuse.ROW, R11.reuse.COL, R38 ;
        /*0610*/                   IMMA.8816.S8.S8 R12, R10.ROW, R11.COL, R4 ;
        /*0620*/                   IADD3 R5, R3.reuse, 0x80, RZ ;
        /*0630*/                   IMMA.8816.S8.S8 R14, R9.ROW, R6.COL, R14 ;
        /*0640*/                   IADD3 R9, R3, 0xa0, RZ ;
        /*0650*/                   IMMA.8816.S8.S8 R22, R10.ROW, R6.COL, R22 ;
        /*0660*/                   NOP ;
        /*0670*/                   NOP ;
        /*0680*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0690*/                   IMAD.MOV.U32 R2, RZ, RZ, R22 ;
        /*06a0*/                   STG.E.64.STRONG.CTA [R3.U32+UR4], R0 ;
        /*06b0*/                   IMAD.MOV.U32 R0, RZ, RZ, R12 ;
        /*06c0*/                   IMAD.MOV.U32 R1, RZ, RZ, R13 ;
        /*06d0*/                   IMAD.MOV.U32 R3, RZ, RZ, R23 ;
        /*06e0*/                   STG.E.64.STRONG.CTA [R5.U32+UR4], R0 ;
        /*06f0*/                   IMAD.MOV.U32 R0, RZ, RZ, R14 ;
        /*0700*/                   IMAD.MOV.U32 R1, RZ, RZ, R15 ;
        /*0710*/                   STG.E.64.STRONG.CTA [R7.U32+UR4], R0 ;
        /*0720*/                   STG.E.64.STRONG.CTA [R9.U32+UR4], R2 ;
        /*0730*/                   EXIT ;
        /*0740*/                   BRA 0x740;
        /*0750*/                   NOP;
        /*0760*/                   NOP;
        /*0770*/                   NOP;
