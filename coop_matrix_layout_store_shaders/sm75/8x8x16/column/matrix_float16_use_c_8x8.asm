        /*0000*/                   IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x48] ;
        /*0010*/                   ISETP.GT.U32.AND P1, PT, R0, 0x3, PT ;
        /*0020*/                   IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x58] ;
        /*0030*/                   ISETP.GT.U32.AND P0, PT, R0, 0x3, PT ;
        /*0040*/                   @P1 IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x40] ;
        /*0050*/                   @P1 MOV R3, c[0x0][0x44] ;
        /*0060*/                   @P1 LDG.E.STRONG.CTA R2, [R2] ;
        /*0070*/                   @P0 IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x50] ;
        /*0080*/                   @P0 MOV R1, c[0x0][0x54] ;
        /*0090*/                   S2R R6, SR_LANEID ;
        /*00a0*/                   @P0 LDG.E.STRONG.CTA R5, [R0] ;
        /*00b0*/                   @!P1 IMAD.MOV.U32 R4, RZ, RZ, RZ ;
        /*00c0*/                   IMAD.MOV.U32 R8, RZ, RZ, 0x3c003c00 ;
        /*00d0*/                   SHF.R.U32.HI R7, RZ, 0x2, R6 ;
        /*00e0*/                   NOP ;
        /*00f0*/                   @P1 SHF.L.U32 R4, R2, 0x1, RZ ;
        /*0100*/                   NOP ;
        /*0110*/                   MOVM.16.MT88 R3, R8 ;
        /*0120*/                   LOP3.LUT R2, R6, 0x3, RZ, 0xc0, !PT ;
        /*0130*/                   @!P0 CS2R R0, SRZ ;
        /*0140*/                   IMAD R7, R7, R4, RZ ;
        /*0150*/                   IMAD R7, R2, 0x4, R7 ;
        /*0160*/                   @P0 SHF.L.U32 R0, R5, 0x1, RZ ;
        /*0170*/                   @P0 MOV R1, RZ ;
        /*0180*/                   IADD3 R0, P0, P1, R7, c[0x0][0x30], R0 ;
        /*0190*/                   IADD3.X R1, RZ, c[0x0][0x34], R1, P0, P1 ;
        /*01a0*/                   STG.E.STRONG.CTA [R0], R3 ;
        /*01b0*/                   EXIT ;
        /*01c0*/                   BRA 0x1c0;
        /*01d0*/                   NOP;
        /*01e0*/                   NOP;
        /*01f0*/                   NOP;
