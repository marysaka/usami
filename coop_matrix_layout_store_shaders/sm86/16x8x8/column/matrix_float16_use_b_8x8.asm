        /*0000*/                   IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x48] ;
        /*0010*/                   ISETP.GT.U32.AND P1, PT, R0, 0x3, PT ;
        /*0020*/                   IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x58] ;
        /*0030*/                   @P1 IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x40] ;
        /*0040*/                   @P1 MOV R3, c[0x0][0x44] ;
        /*0050*/                   @P1 LDG.E.STRONG.SM R2, [R2] ;
        /*0060*/                   ISETP.GT.U32.AND P0, PT, R0, 0x3, PT ;
        /*0070*/                   S2R R6, SR_LANEID ;
        /*0080*/                   @P0 IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x50] ;
        /*0090*/                   @P0 MOV R1, c[0x0][0x54] ;
        /*00a0*/                   @P0 LDG.E.STRONG.SM R5, [R0] ;
        /*00b0*/                   @!P1 IMAD.MOV.U32 R4, RZ, RZ, RZ ;
        /*00c0*/                   IMAD.MOV.U32 R8, RZ, RZ, 0x3c003c00 ;
        /*00d0*/                   SHF.R.U32.HI R7, RZ, 0x2, R6 ;
        /*00e0*/                   NOP ;
        /*00f0*/                   @P1 SHF.L.U32 R4, R2, 0x1, RZ ;
        /*0100*/                   NOP ;
        /*0110*/                   MOVM.16.MT88 R3, R8 ;
        /*0120*/                   LOP3.LUT R2, R6, 0x3, RZ, 0xc0, !PT ;
        /*0130*/                   IMAD R7, R7, R4, RZ ;
        /*0140*/                   @!P0 CS2R R0, SRZ ;
        /*0150*/                   @P0 SHF.L.U32 R0, R5, 0x1, RZ ;
        /*0160*/                   IMAD R7, R2, 0x4, R7 ;
        /*0170*/                   @P0 MOV R1, RZ ;
        /*0180*/                   IADD3 R0, P0, P1, R7, c[0x0][0x30], R0 ;
        /*0190*/                   IADD3.X R1, RZ, c[0x0][0x34], R1, P0, P1 ;
        /*01a0*/                   STG.E.STRONG.SM [R0], R3 ;
        /*01b0*/                   EXIT ;
        /*01c0*/                   BRA 0x1c0;
        /*01d0*/                   NOP;
        /*01e0*/                   NOP;
        /*01f0*/                   NOP;
        /*0200*/                   NOP;
        /*0210*/                   NOP;
        /*0220*/                   NOP;
        /*0230*/                   NOP;
        /*0240*/                   NOP;
        /*0250*/                   NOP;
        /*0260*/                   NOP;
        /*0270*/                   NOP;
