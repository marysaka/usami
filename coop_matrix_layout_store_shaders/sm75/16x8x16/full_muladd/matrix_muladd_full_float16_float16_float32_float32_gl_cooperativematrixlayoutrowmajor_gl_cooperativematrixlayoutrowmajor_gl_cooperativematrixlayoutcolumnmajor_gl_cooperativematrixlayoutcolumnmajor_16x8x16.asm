        /*0000*/                   S2R R0, SR_CTAID.X ;
        /*0010*/                   IADD3 R0, R0, c[0x0][0x20], RZ ;
        /*0020*/                   IMAD.SHL.U32 R3, R0.reuse, 0x4, RZ ;
        /*0030*/                   IMAD.SHL.U32 R0, R0, 0x2, RZ ;
        /*0040*/                   ISETP.GE.U32.AND P2, PT, R3, c[0x0][0x68], PT ;
        /*0050*/                   ISETP.GE.U32.AND P0, PT, R0.reuse, c[0x0][0x48], PT ;
        /*0060*/                   ISETP.GE.U32.AND P1, PT, R0, c[0x0][0x58], PT ;
        /*0070*/                   ULDC.64 UR4, c[0x0][0x40] ;
        /*0080*/                   ULDC.64 UR8, c[0x0][0x60] ;
        /*0090*/                   LDG.E.U16.CONSTANT.SYS R1, [R0.U32+UR4], !P0 ;
        /*00a0*/                   ULDC.64 UR6, c[0x0][0x50] ;
        /*00b0*/                   LDG.E.CONSTANT.SYS R3, [R3.U32+UR8], !P2 ;
        /*00c0*/                   LDG.E.U16.CONSTANT.SYS R2, [R0.U32+UR6], !P1 ;
        /*00d0*/                   S2R R9, SR_LANEID ;
        /*00e0*/                   LOP3.LUT R4, R9, 0xf, RZ, 0xc0, !PT ;
        /*00f0*/                   LOP3.LUT R5, R9.reuse, 0x10, RZ, 0xc0, !PT ;
        /*0100*/                   LOP3.LUT R6, R9.reuse, 0xfffffffc, RZ, 0xc0, !PT ;
        /*0110*/                   LOP3.LUT R7, R9.reuse, 0x3, RZ, 0xc0, !PT ;
        /*0120*/                   IMAD R4, R4, 0x4, R5 ;
        /*0130*/                   IMAD.SHL.U32 R9, R9, 0x4, RZ ;
        /*0140*/                   IMAD R8, R7, 0x20, R6 ;
        /*0150*/                   STS.U16 [0x300], R1 ;
        /*0160*/                   STS [RZ], R3 ;
        /*0170*/                   STS.U16 [0x200], R2 ;
        /*0180*/                   NOP ;
        /*0190*/                   NOP ;
        /*01a0*/                   LDSM.16.MT88.2 R0, [R9+0x200] ;
        /*01b0*/                   LDS.U R12, [R8] ;
        /*01c0*/                   LDS.U R13, [R8+0x10] ;
        /*01d0*/                   LDS.U R14, [R8+0x20] ;
        /*01e0*/                   LDS.U R15, [R8+0x30] ;
        /*01f0*/                   LDSM.16.M88.4 R4, [R4+0x300] ;
        /*0200*/                   HMMA.16816.F32 R12, R4, R0, R12 ;
        /*0210*/                   IADD3 R0, R8, 0x20, RZ ;
        /*0220*/                   NOP ;
        /*0230*/                   NOP ;
        /*0240*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0250*/                   STG.E.STRONG.CTA [R8.U32+UR4], R12 ;
        /*0260*/                   STG.E.STRONG.CTA [R8.U32+UR4+0x10], R13 ;
        /*0270*/                   STG.E.STRONG.CTA [R0.U32+UR4], R14 ;
        /*0280*/                   STG.E.STRONG.CTA [R0.U32+UR4+0x10], R15 ;
        /*0290*/                   EXIT ;
        /*02a0*/                   BRA 0x2a0;
        /*02b0*/                   NOP;
        /*02c0*/                   NOP;
        /*02d0*/                   NOP;
        /*02e0*/                   NOP;
        /*02f0*/                   NOP;
