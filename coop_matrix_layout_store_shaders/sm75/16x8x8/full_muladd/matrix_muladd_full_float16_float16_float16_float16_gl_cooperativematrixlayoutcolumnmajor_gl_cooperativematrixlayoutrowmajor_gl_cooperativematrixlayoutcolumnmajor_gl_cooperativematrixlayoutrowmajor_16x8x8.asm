        /*0000*/                   S2R R0, SR_CTAID.X ;
        /*0010*/                   IADD3 R0, R0, c[0x0][0x20], RZ ;
        /*0020*/                   IMAD.SHL.U32 R0, R0, 0x2, RZ ;
        /*0030*/                   ISETP.GE.U32.AND P0, PT, R0.reuse, c[0x0][0x48], PT ;
        /*0040*/                   ISETP.GE.U32.AND P1, PT, R0.reuse, c[0x0][0x58], PT ;
        /*0050*/                   ISETP.GE.U32.AND P2, PT, R0, c[0x0][0x68], PT ;
        /*0060*/                   ULDC.64 UR4, c[0x0][0x40] ;
        /*0070*/                   ULDC.64 UR6, c[0x0][0x50] ;
        /*0080*/                   LDG.E.U16.CONSTANT.SYS R1, [R0.U32+UR4], !P0 ;
        /*0090*/                   ULDC.64 UR8, c[0x0][0x60] ;
        /*00a0*/                   LDG.E.U16.CONSTANT.SYS R2, [R0.U32+UR6], !P1 ;
        /*00b0*/                   LDG.E.U16.CONSTANT.SYS R3, [R0.U32+UR8], !P2 ;
        /*00c0*/                   S2R R9, SR_LANEID ;
        /*00d0*/                   LOP3.LUT R4, R9, 0x8, RZ, 0xc0, !PT ;
        /*00e0*/                   IMAD.SHL.U32 R4, R4, 0x2, RZ ;
        /*00f0*/                   LOP3.LUT R5, R9, 0x7, R4, 0xc8, !PT ;
        /*0100*/                   LOP3.LUT R4, R4, 0x10, RZ, 0xc0, !PT ;
        /*0110*/                   IMAD R4, R5, 0x4, R4 ;
        /*0120*/                   IMAD.SHL.U32 R5, R9, 0x4, RZ ;
        /*0130*/                   STS.U16 [0x180], R1 ;
        /*0140*/                   STS.U16 [0x100], R2 ;
        /*0150*/                   STS.U16 [RZ], R3 ;
        /*0160*/                   NOP ;
        /*0170*/                   NOP ;
        /*0180*/                   LDSM.16.MT88 R2, [R5+0x100] ;
        /*0190*/                   LOP3.LUT R3, R9.reuse, 0xfffffffc, RZ, 0xc0, !PT ;
        /*01a0*/                   LOP3.LUT R8, R9, 0x3, RZ, 0xc0, !PT ;
        /*01b0*/                   LDSM.16.MT88.2 R0, [R4+0x180] ;
        /*01c0*/                   LDSM.16.MT88.2 R6, [R4] ;
        /*01d0*/                   IMAD R3, R8, 0x4, R3 ;
        /*01e0*/                   HMMA.1688.F16 R6, R0, R2, R6 ;
        /*01f0*/                   IADD3 R0, R3, 0x20, RZ ;
        /*0200*/                   NOP ;
        /*0210*/                   NOP ;
        /*0220*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*0230*/                   STG.E.STRONG.CTA [R3.U32+UR4], R6 ;
        /*0240*/                   STG.E.STRONG.CTA [R0.U32+UR4], R7 ;
        /*0250*/                   EXIT ;
        /*0260*/                   BRA 0x260;
        /*0270*/                   NOP;
