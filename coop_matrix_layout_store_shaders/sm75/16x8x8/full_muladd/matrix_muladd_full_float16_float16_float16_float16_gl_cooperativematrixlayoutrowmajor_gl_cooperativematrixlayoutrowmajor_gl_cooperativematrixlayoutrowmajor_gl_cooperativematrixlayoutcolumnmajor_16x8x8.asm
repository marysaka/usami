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
        /*00c0*/                   S2R R5, SR_LANEID ;
        /*00d0*/                   IMAD.SHL.U32 R4, R5, 0x4, RZ ;
        /*00e0*/                   STS.U16 [0x180], R1 ;
        /*00f0*/                   STS.U16 [0x100], R2 ;
        /*0100*/                   STS.U16 [RZ], R3 ;
        /*0110*/                   NOP ;
        /*0120*/                   NOP ;
        /*0130*/                   LDSM.16.MT88 R2, [R4+0x100] ;
        /*0140*/                   LDSM.16.M88.2 R0, [R4+0x180] ;
        /*0150*/                   LDSM.16.M88.2 R6, [R4] ;
        /*0160*/                   HMMA.1688.F16 R6, R0, R2, R6 ;
        /*0170*/                   NOP ;
        /*0180*/                   NOP ;
        /*0190*/                   MOVM.16.MT88 R3, R6 ;
        /*01a0*/                   LOP3.LUT R0, R5.reuse, 0xfffffffc, RZ, 0xc0, !PT ;
        /*01b0*/                   ULDC.64 UR4, c[0x0][0x30] ;
        /*01c0*/                   LOP3.LUT R1, R5, 0x3, RZ, 0xc0, !PT ;
        /*01d0*/                   MOVM.16.MT88 R2, R7 ;
        /*01e0*/                   IMAD R0, R1, 0x4, R0 ;
        /*01f0*/                   IADD3 R1, R0, 0x10, RZ ;
        /*0200*/                   STG.E.STRONG.CTA [R0.U32+UR4], R3 ;
        /*0210*/                   STG.E.STRONG.CTA [R1.U32+UR4], R2 ;
        /*0220*/                   EXIT ;
        /*0230*/                   BRA 0x230;
        /*0240*/                   NOP;
        /*0250*/                   NOP;
        /*0260*/                   NOP;
        /*0270*/                   NOP;
