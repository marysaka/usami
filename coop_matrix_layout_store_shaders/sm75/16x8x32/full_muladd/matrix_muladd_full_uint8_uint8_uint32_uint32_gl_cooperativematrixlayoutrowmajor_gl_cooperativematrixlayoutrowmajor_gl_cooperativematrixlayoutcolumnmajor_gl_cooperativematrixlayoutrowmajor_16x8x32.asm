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
        /*0150*/                   LDS.U.S8 R13, [R6+0x203] ;
        /*0160*/                   LOP3.LUT R2, R7.reuse, 0xfffffffc, RZ, 0xc0, !PT ;
        /*0170*/                   LOP3.LUT R0, R7.reuse, 0xf, RZ, 0xc0, !PT ;
        /*0180*/                   LDS.U.S8 R11, [R6+0x202] ;
        /*0190*/                   LOP3.LUT R1, R7, 0x10, RZ, 0xc0, !PT ;
        /*01a0*/                   IMAD R15, R5, 0x20, R2 ;
        /*01b0*/                   LDS.U.S8 R9, [R6+0x201] ;
        /*01c0*/                   IMAD R5, R4, 0x2, R5 ;
        /*01d0*/                   IMAD.IADD R0, R0, 0x1, R1 ;
        /*01e0*/                   LDS.U.S8 R14, [R6+0x213] ;
        /*01f0*/                   IMAD.SHL.U32 R5, R5, 0x8, RZ ;
        /*0200*/                   LDS.U.S8 R7, [R6+0x200] ;
        /*0210*/                   LDS.U.S8 R12, [R6+0x212] ;
        /*0220*/                   LDS.U.S8 R10, [R6+0x211] ;
        /*0230*/                   LDS.U R18, [R15] ;
        /*0240*/                   LDS.U R19, [R15+0x10] ;
        /*0250*/                   LDSM.16.M88.4 R0, [R0+0x300] ;
        /*0260*/                   LOP3.LUT R16, R13, 0xff, RZ, 0xc0, !PT ;
        /*0270*/                   LDS.U R20, [R15+0x20] ;
        /*0280*/                   PRMT R16, R16, 0x2104, R11 ;
        /*0290*/                   LDS.U R21, [R15+0x30] ;
        /*02a0*/                   PRMT R16, R16, 0x2104, R9 ;
        /*02b0*/                   LDS.U.S8 R8, [R6+0x210] ;
        /*02c0*/                   LOP3.LUT R9, R14, 0xff, RZ, 0xc0, !PT ;
        /*02d0*/                   PRMT R7, R16, 0x2104, R7 ;
        /*02e0*/                   PRMT R9, R9, 0x2104, R12 ;
        /*02f0*/                   PRMT R9, R9, 0x2104, R10 ;
        /*0300*/                   IMMA.8816.U8.U8 R18, R0.ROW, R7.reuse.COL, R18 ;
        /*0310*/                   IMMA.8816.U8.U8 R20, R1.ROW, R7.COL, R20 ;
        /*0320*/                   IADD3 R7, R5, 0x80, RZ ;
        /*0330*/                   PRMT R8, R9, 0x2104, R8 ;
        /*0340*/                   IMMA.8816.U8.U8 R0, R2.ROW, R8.reuse.COL, R18 ;
        /*0350*/                   IMMA.8816.U8.U8 R2, R3.ROW, R8.COL, R20 ;
        /*0360*/                   NOP ;
        /*0370*/                   NOP ;
        /*0380*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0390*/                   STG.E.64.STRONG.CTA [R5.U32+UR4], R0 ;
        /*03a0*/                   STG.E.64.STRONG.CTA [R7.U32+UR4], R2 ;
        /*03b0*/                   EXIT ;
        /*03c0*/                   BRA 0x3c0;
        /*03d0*/                   NOP;
        /*03e0*/                   NOP;
        /*03f0*/                   NOP;
