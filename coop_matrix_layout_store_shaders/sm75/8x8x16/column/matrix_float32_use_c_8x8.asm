        /*0000*/                   IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x48] ;
        /*0010*/                   IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x58] ;
        /*0020*/                   ISETP.GT.U32.AND P1, PT, R1, 0x3, PT ;
        /*0030*/                   ISETP.GT.U32.AND P0, PT, R0, 0x3, PT ;
        /*0040*/                   @P1 IMAD.MOV.U32 R4, RZ, RZ, c[0x0][0x40] ;
        /*0050*/                   @P1 MOV R5, c[0x0][0x44] ;
        /*0060*/                   @P0 IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x50] ;
        /*0070*/                   @P0 IMAD.MOV.U32 R3, RZ, RZ, c[0x0][0x54] ;
        /*0080*/                   @P1 LDG.E.STRONG.CTA R4, [R4] ;
        /*0090*/                   @P0 LDG.E.STRONG.CTA R2, [R2] ;
        /*00a0*/                   @!P1 MOV R6, RZ ;
        /*00b0*/                   @!P0 CS2R R0, SRZ ;
        /*00c0*/                   S2R R7, SR_LANEID ;
        /*00d0*/                   IMAD.MOV.U32 R5, RZ, RZ, 0x3f800000 ;
        /*00e0*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, RZ ;
        /*00f0*/                   IMAD.SHL.U32 R8, R7.reuse, 0x2, RZ ;
        /*0100*/                   LOP3.LUT R7, R7, 0xfffffffc, RZ, 0xc0, !PT ;
        /*0110*/                   LOP3.LUT R8, R8, 0x6, RZ, 0xe2, !PT ;
        /*0120*/                   NOP ;
        /*0130*/                   @P1 IMAD.SHL.U32 R6, R4, 0x4, RZ ;
        /*0140*/                   IMAD R6, R8, R6, R7 ;
        /*0150*/                   @P0 SHF.L.U32 R0, R2, 0x2, RZ ;
        /*0160*/                   IADD3 R3, P0, R0, c[0x0][0x30], RZ ;
        /*0170*/                   IADD3 R2, R6, 0x20, RZ ;
        /*0180*/                   IADD3.X R4, R1, c[0x0][0x34], RZ, P0, !PT ;
        /*0190*/                   IADD3 R2, P1, R2, R3.reuse, RZ ;
        /*01a0*/                   IADD3 R0, P0, R6, R3, RZ ;
        /*01b0*/                   IMAD.X R3, RZ, RZ, R4, P1 ;
        /*01c0*/                   IADD3.X R1, RZ, R4, RZ, P0, !PT ;
        /*01d0*/                   NOP ;
        /*01e0*/                   STG.E.STRONG.CTA [R0], R5 ;
        /*01f0*/                   STG.E.STRONG.CTA [R2], R5 ;
        /*0200*/                   EXIT ;
        /*0210*/                   BRA 0x210;
        /*0220*/                   NOP;
        /*0230*/                   NOP;
        /*0240*/                   NOP;
        /*0250*/                   NOP;
        /*0260*/                   NOP;
        /*0270*/                   NOP;
