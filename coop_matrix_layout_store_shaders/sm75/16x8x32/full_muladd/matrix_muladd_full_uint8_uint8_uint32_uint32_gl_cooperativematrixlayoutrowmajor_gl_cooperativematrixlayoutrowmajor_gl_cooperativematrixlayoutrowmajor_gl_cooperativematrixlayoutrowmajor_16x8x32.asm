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
        /*0150*/                   LDS.U.S8 R11, [R6+0x203] ;
        /*0160*/                   LOP3.LUT R0, R7.reuse, 0xf, RZ, 0xc0, !PT ;
        /*0170*/                   IMAD R13, R4, 0x2, R5 ;
        /*0180*/                   LOP3.LUT R1, R7, 0x10, RZ, 0xc0, !PT ;
        /*0190*/                   LDS.U.S8 R9, [R6+0x202] ;
        /*01a0*/                   LDS.U.S8 R7, [R6+0x201] ;
        /*01b0*/                   IMAD.IADD R0, R0, 0x1, R1 ;
        /*01c0*/                   LDS.U.S8 R12, [R6+0x213] ;
        /*01d0*/                   LDS.U.S8 R4, [R6+0x200] ;
        /*01e0*/                   LDS.U.S8 R10, [R6+0x212] ;
        /*01f0*/                   LDS.U.S8 R8, [R6+0x211] ;
        /*0200*/                   LDS.U.64 R16, [R13.X8] ;
        /*0210*/                   LDSM.16.M88.4 R0, [R0+0x300] ;
        /*0220*/                   LDS.U.64 R18, [R13.X8+0x80] ;
        /*0230*/                   LOP3.LUT R14, R11, 0xff, RZ, 0xc0, !PT ;
        /*0240*/                   LDS.U.S8 R5, [R6+0x210] ;
        /*0250*/                   PRMT R14, R14, 0x2104, R9 ;
        /*0260*/                   PRMT R7, R14, 0x2104, R7 ;
        /*0270*/                   LOP3.LUT R9, R12, 0xff, RZ, 0xc0, !PT ;
        /*0280*/                   PRMT R4, R7, 0x2104, R4 ;
        /*0290*/                   IMAD.SHL.U32 R7, R13, 0x8, RZ ;
        /*02a0*/                   PRMT R9, R9, 0x2104, R10 ;
        /*02b0*/                   PRMT R8, R9, 0x2104, R8 ;
        /*02c0*/                   IADD3 R9, R7, 0x80, RZ ;
        /*02d0*/                   IMMA.8816.U8.U8 R16, R0.ROW, R4.reuse.COL, R16 ;
        /*02e0*/                   IMMA.8816.U8.U8 R18, R1.ROW, R4.COL, R18 ;
        /*02f0*/                   PRMT R5, R8, 0x2104, R5 ;
        /*0300*/                   IMMA.8816.U8.U8 R0, R2.ROW, R5.reuse.COL, R16 ;
        /*0310*/                   IMMA.8816.U8.U8 R2, R3.ROW, R5.COL, R18 ;
        /*0320*/                   NOP ;
        /*0330*/                   NOP ;
        /*0340*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0350*/                   STG.E.64.STRONG.CTA [R7.U32+UR4], R0 ;
        /*0360*/                   STG.E.64.STRONG.CTA [R9.U32+UR4], R2 ;
        /*0370*/                   EXIT ;
        /*0380*/                   BRA 0x380;
        /*0390*/                   NOP;
        /*03a0*/                   NOP;
        /*03b0*/                   NOP;
        /*03c0*/                   NOP;
        /*03d0*/                   NOP;
        /*03e0*/                   NOP;
        /*03f0*/                   NOP;
