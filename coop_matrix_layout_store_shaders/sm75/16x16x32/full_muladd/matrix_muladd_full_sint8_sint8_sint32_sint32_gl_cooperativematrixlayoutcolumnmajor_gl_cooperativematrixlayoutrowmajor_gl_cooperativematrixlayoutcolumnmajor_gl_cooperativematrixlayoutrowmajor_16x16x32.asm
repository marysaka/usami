        /*0000*/                   S2R R0, SR_CTAID.X ;
        /*0010*/                   IADD3 R0, R0, c[0x0][0x20], RZ ;
        /*0020*/                   ISETP.GE.U32.AND P0, PT, R0.reuse, c[0x0][0x48], PT ;
        /*0030*/                   IMAD.SHL.U32 R3, R0.reuse, 0x4, RZ ;
        /*0040*/                   ISETP.GE.U32.AND P1, PT, R0, c[0x0][0x58], PT ;
        /*0050*/                   ISETP.GE.U32.AND P2, PT, R3, c[0x0][0x68], PT ;
        /*0060*/                   ULDC.64 UR4, c[0x0][0x40] ;
        /*0070*/                   LDG.E.U8.CONSTANT.SYS R1, [R0.U32+UR4], !P0 ;
        /*0080*/                   ULDC.64 UR6, c[0x0][0x50] ;
        /*0090*/                   ULDC.64 UR4, c[0x0][0x60] ;
        /*00a0*/                   LDG.E.U8.CONSTANT.SYS R2, [R0.U32+UR6], !P1 ;
        /*00b0*/                   LDG.E.CONSTANT.SYS R3, [R3.U32+UR4], !P2 ;
        /*00c0*/                   S2R R4, SR_LANEID ;
        /*00d0*/                   SHF.R.U32.HI R8, RZ, 0x2, R4 ;
        /*00e0*/                   LOP3.LUT R9, R4.reuse, 0x3, RZ, 0xc0, !PT ;
        /*00f0*/                   LOP3.LUT R4, R4, 0xfffffffc, RZ, 0xc0, !PT ;
        /*0100*/                   IMAD R10, R9, 0x4, R8 ;
        /*0110*/                   STS.U8 [0x600], R1 ;
        /*0120*/                   STS.U8 [0x400], R2 ;
        /*0130*/                   STS [RZ], R3 ;
        /*0140*/                   NOP ;
        /*0150*/                   NOP ;
        /*0160*/                   LDS.U.S8 R2, [R10+0x603] ;
        /*0170*/                   IMAD R43, R9, 0x20, R4 ;
        /*0180*/                   IMAD R9, R8, 0x2, R9 ;
        /*0190*/                   LDS.U.S8 R39, [R10+0x403] ;
        /*01a0*/                   IMAD.SHL.U32 R9, R9, 0x8, RZ ;
        /*01b0*/                   LDS.U.S8 R19, [R10+0x602] ;
        /*01c0*/                   LDS.U.S8 R35, [R10+0x402] ;
        /*01d0*/                   LDS.U.S8 R15, [R10+0x601] ;
        /*01e0*/                   LDS.U.S8 R31, [R10+0x401] ;
        /*01f0*/                   LDS.U.S8 R23, [R10+0x60b] ;
        /*0200*/                   LDS.U.S8 R24, [R10+0x613] ;
        /*0210*/                   LDS.U.S8 R40, [R10+0x40b] ;
        /*0220*/                   LDS.U.S8 R11, [R10+0x600] ;
        /*0230*/                   LDS.U.S8 R27, [R10+0x400] ;
        /*0240*/                   LOP3.LUT R26, R2, 0xff, RZ, 0xc0, !PT ;
        /*0250*/                   LDS.U.S8 R20, [R10+0x60a] ;
        /*0260*/                   LOP3.LUT R44, R39, 0xff, RZ, 0xc0, !PT ;
        /*0270*/                   LDS.U.S8 R21, [R10+0x612] ;
        /*0280*/                   PRMT R26, R26, 0x2104, R19 ;
        /*0290*/                   LDS.U.S8 R36, [R10+0x40a] ;
        /*02a0*/                   PRMT R44, R44, 0x2104, R35 ;
        /*02b0*/                   LDS.U.S8 R16, [R10+0x609] ;
        /*02c0*/                   PRMT R26, R26, 0x2104, R15 ;
        /*02d0*/                   LDS.U.S8 R17, [R10+0x611] ;
        /*02e0*/                   PRMT R44, R44, 0x2104, R31 ;
        /*02f0*/                   LDS.U.S8 R32, [R10+0x409] ;
        /*0300*/                   LOP3.LUT R23, R23, 0xff, RZ, 0xc0, !PT ;
        /*0310*/                   LDS.U R6, [R43] ;
        /*0320*/                   LOP3.LUT R24, R24, 0xff, RZ, 0xc0, !PT ;
        /*0330*/                   LDS.U R7, [R43+0x10] ;
        /*0340*/                   LOP3.LUT R15, R40, 0xff, RZ, 0xc0, !PT ;
        /*0350*/                   LDS.U.S8 R25, [R10+0x61b] ;
        /*0360*/                   PRMT R11, R26, 0x2104, R11 ;
        /*0370*/                   LDS.U.S8 R41, [R10+0x413] ;
        /*0380*/                   PRMT R27, R44, 0x2104, R27 ;
        /*0390*/                   LDS.U.S8 R42, [R10+0x41b] ;
        /*03a0*/                   PRMT R23, R23, 0x2104, R20 ;
        /*03b0*/                   LDS.U.S8 R12, [R10+0x608] ;
        /*03c0*/                   PRMT R24, R24, 0x2104, R21 ;
        /*03d0*/                   LDS.U.S8 R28, [R10+0x408] ;
        /*03e0*/                   PRMT R15, R15, 0x2104, R36 ;
        /*03f0*/                   LDS.U.S8 R22, [R10+0x61a] ;
        /*0400*/                   PRMT R23, R23, 0x2104, R16 ;
        /*0410*/                   LDS.U.S8 R37, [R10+0x412] ;
        /*0420*/                   PRMT R24, R24, 0x2104, R17 ;
        /*0430*/                   LDS.U.S8 R38, [R10+0x41a] ;
        /*0440*/                   PRMT R15, R15, 0x2104, R32 ;
        /*0450*/                   LDS.U.S8 R18, [R10+0x619] ;
        /*0460*/                   LDS.U.S8 R33, [R10+0x411] ;
        /*0470*/                   IMMA.8816.S8.S8 R16, R11.ROW, R27.COL, R6 ;
        /*0480*/                   LDS.U.S8 R34, [R10+0x419] ;
        /*0490*/                   LOP3.LUT R25, R25, 0xff, RZ, 0xc0, !PT ;
        /*04a0*/                   LDS.U R0, [R43+0x80] ;
        /*04b0*/                   LDS.U R1, [R43+0x90] ;
        /*04c0*/                   LOP3.LUT R7, R42, 0xff, RZ, 0xc0, !PT ;
        /*04d0*/                   LDS.U R2, [R43+0xa0] ;
        /*04e0*/                   PRMT R12, R23, 0x2104, R12 ;
        /*04f0*/                   LDS.U R3, [R43+0xb0] ;
        /*0500*/                   PRMT R15, R15, 0x2104, R28 ;
        /*0510*/                   LDS.U R4, [R43+0x20] ;
        /*0520*/                   PRMT R25, R25, 0x2104, R22 ;
        /*0530*/                   LDS.U R5, [R43+0x30] ;
        /*0540*/                   LDS.U.S8 R13, [R10+0x610] ;
        /*0550*/                   PRMT R7, R7, 0x2104, R38 ;
        /*0560*/                   LDS.U.S8 R14, [R10+0x618] ;
        /*0570*/                   PRMT R25, R25, 0x2104, R18 ;
        /*0580*/                   LDS.U.S8 R29, [R10+0x410] ;
        /*0590*/                   LDS.U.S8 R30, [R10+0x418] ;
        /*05a0*/                   PRMT R7, R7, 0x2104, R34 ;
        /*05b0*/                   LOP3.LUT R10, R41, 0xff, RZ, 0xc0, !PT ;
        /*05c0*/                   IMMA.8816.S8.S8 R18, R11.ROW, R15.COL, R0 ;
        /*05d0*/                   IADD3 R11, R9, 0x20, RZ ;
        /*05e0*/                   PRMT R10, R10, 0x2104, R37 ;
        /*05f0*/                   PRMT R10, R10, 0x2104, R33 ;
        /*0600*/                   IMMA.8816.S8.S8 R2, R12.reuse.ROW, R15.COL, R2 ;
        /*0610*/                   IMMA.8816.S8.S8 R4, R12.ROW, R27.COL, R4 ;
        /*0620*/                   PRMT R13, R24, 0x2104, R13 ;
        /*0630*/                   PRMT R14, R25, 0x2104, R14 ;
        /*0640*/                   PRMT R10, R10, 0x2104, R29 ;
        /*0650*/                   PRMT R7, R7, 0x2104, R30 ;
        /*0660*/                   IMMA.8816.S8.S8 R0, R13.reuse.ROW, R10.COL, R16 ;
        /*0670*/                   IMMA.8816.S8.S8 R18, R13.ROW, R7.reuse.COL, R18 ;
        /*0680*/                   IADD3 R13, R9.reuse, 0xa0, RZ ;
        /*0690*/                   IMMA.8816.S8.S8 R2, R14.ROW, R7.COL, R2 ;
        /*06a0*/                   IADD3 R7, R9, 0x80, RZ ;
        /*06b0*/                   IMMA.8816.S8.S8 R4, R14.ROW, R10.COL, R4 ;
        /*06c0*/                   NOP ;
        /*06d0*/                   NOP ;
        /*06e0*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*06f0*/                   STG.E.64.STRONG.CTA [R9.U32+UR4], R0 ;
        /*0700*/                   IMAD.MOV.U32 R0, RZ, RZ, R4 ;
        /*0710*/                   IMAD.MOV.U32 R1, RZ, RZ, R5 ;
        /*0720*/                   STG.E.64.STRONG.CTA [R7.U32+UR4], R0 ;
        /*0730*/                   IMAD.MOV.U32 R0, RZ, RZ, R18 ;
        /*0740*/                   IMAD.MOV.U32 R1, RZ, RZ, R19 ;
        /*0750*/                   STG.E.64.STRONG.CTA [R11.U32+UR4], R0 ;
        /*0760*/                   STG.E.64.STRONG.CTA [R13.U32+UR4], R2 ;
        /*0770*/                   EXIT ;
        /*0780*/                   BRA 0x780;
        /*0790*/                   NOP;
        /*07a0*/                   NOP;
        /*07b0*/                   NOP;
        /*07c0*/                   NOP;
        /*07d0*/                   NOP;
        /*07e0*/                   NOP;
        /*07f0*/                   NOP;
