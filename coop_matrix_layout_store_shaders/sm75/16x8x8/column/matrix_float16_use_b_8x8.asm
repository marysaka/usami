        /*0000*/                   I2F.F16 R1, 0x1 ;
        /*0010*/                   IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x58] ;
        /*0020*/                   STL [RZ], RZ ;
        /*0030*/                   IMAD.MOV.U32 R3, RZ, RZ, c[0x0][0x48] ;
        /*0040*/                   ISETP.GT.U32.AND P0, PT, R2, 0x3, PT ;
        /*0050*/                   I2F.F16 R0, 0x2 ;
        /*0060*/                   ISETP.GT.U32.AND P1, PT, R3, 0x3, PT ;
        /*0070*/                   @P1 IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x40] ;
        /*0080*/                   @P1 MOV R3, c[0x0][0x44] ;
        /*0090*/                   PRMT R5, R1, 0x5410, R0 ;
        /*00a0*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x54] ;
        /*00b0*/                   @P0 MOV R0, c[0x0][0x50] ;
        /*00c0*/                   @P1 LDG.E.STRONG.CTA R2, [R2] ;
        /*00d0*/                   @P0 LDG.E.STRONG.CTA R4, [R0] ;
        /*00e0*/                   STL [RZ], R5 ;
        /*00f0*/                   NOP ;
        /*0100*/                   NOP ;
        /*0110*/                   LDL R3, [RZ] ;
        /*0120*/                   @!P1 IMAD.MOV.U32 R0, RZ, RZ, RZ ;
        /*0130*/                   S2R R6, SR_LANEID ;
        /*0140*/                   @P1 IMAD.SHL.U32 R0, R2, 0x2, RZ ;
        /*0150*/                   SHF.R.U32.HI R1, RZ, 0x2, R6 ;
        /*0160*/                   LOP3.LUT R2, R6, 0x3, RZ, 0xc0, !PT ;
        /*0170*/                   IMAD R5, R1, R0, RZ ;
        /*0180*/                   @!P0 CS2R R0, SRZ ;
        /*0190*/                   IMAD R5, R2, 0x4, R5 ;
        /*01a0*/                   @P0 SHF.L.U32 R0, R4, 0x1, RZ ;
        /*01b0*/                   @P0 MOV R1, RZ ;
        /*01c0*/                   IADD3 R0, P0, P1, R5, c[0x0][0x30], R0 ;
        /*01d0*/                   IADD3.X R1, RZ, c[0x0][0x34], R1, P0, P1 ;
        /*01e0*/                   MOVM.16.MT88 R3, R3 ;
        /*01f0*/                   STG.E.STRONG.CTA [R0], R3 ;
        /*0200*/                   EXIT ;
        /*0210*/                   BRA 0x210;
        /*0220*/                   NOP;
        /*0230*/                   NOP;
        /*0240*/                   NOP;
        /*0250*/                   NOP;
        /*0260*/                   NOP;
        /*0270*/                   NOP;
