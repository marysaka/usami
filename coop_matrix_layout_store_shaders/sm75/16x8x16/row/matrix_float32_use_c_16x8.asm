        /*0000*/                   IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x48] ;
        /*0010*/                   IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x58] ;
        /*0020*/                   ISETP.GT.U32.AND P1, PT, R1, 0x3, PT ;
        /*0030*/                   ISETP.GT.U32.AND P0, PT, R0, 0x3, PT ;
        /*0040*/                   IMAD.MOV.U32 R4, RZ, RZ, 0x3f800000 ;
        /*0050*/                   MOV R6, 0x40400000 ;
        /*0060*/                   IMAD.MOV.U32 R5, RZ, RZ, 0x40000000 ;
        /*0070*/                   IMAD.MOV.U32 R7, RZ, RZ, 0x40800000 ;
        /*0080*/                   @P1 MOV R2, c[0x0][0x40] ;
        /*0090*/                   @P1 IMAD.MOV.U32 R3, RZ, RZ, c[0x0][0x44] ;
        /*00a0*/                   @P0 MOV R1, c[0x0][0x54] ;
        /*00b0*/                   @P0 IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x50] ;
        /*00c0*/                   STL.128 [RZ], R4 ;
        /*00d0*/                   @P1 LDG.E.STRONG.CTA R3, [R2] ;
        /*00e0*/                   @P0 LDG.E.STRONG.CTA R8, [R0] ;
        /*00f0*/                   NOP ;
        /*0100*/                   NOP ;
        /*0110*/                   LDL.128 R4, [RZ] ;
        /*0120*/                   @!P1 IMAD.MOV.U32 R2, RZ, RZ, RZ ;
        /*0130*/                   @!P0 CS2R R0, SRZ ;
        /*0140*/                   S2R R10, SR_LANEID ;
        /*0150*/                   @P1 SHF.L.U32 R2, R3, 0x2, RZ ;
        /*0160*/                   @P0 IMAD.SHL.U32 R0, R8, 0x4, RZ ;
        /*0170*/                   @P0 MOV R1, RZ ;
        /*0180*/                   SHF.R.U32.HI R9, RZ, 0x2, R10 ;
        /*0190*/                   LOP3.LUT R3, R10, 0x3, RZ, 0xc0, !PT ;
        /*01a0*/                   IMAD R8, R9, R2, RZ ;
        /*01b0*/                   IMAD R3, R3, 0x8, R8 ;
        /*01c0*/                   IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
        /*01d0*/                   LEA R2, R2, R3, 0x3 ;
        /*01e0*/                   IADD3.X R9, R1, c[0x0][0x34], RZ, P0, !PT ;
        /*01f0*/                   IADD3 R0, P0, R3, R8.reuse, RZ ;
        /*0200*/                   IADD3 R2, P1, R2, R8, RZ ;
        /*0210*/                   IMAD.X R1, RZ, RZ, R9, P0 ;
        /*0220*/                   IADD3.X R3, RZ, R9, RZ, P1, !PT ;
        /*0230*/                   STG.E.64.STRONG.CTA [R0], R4 ;
        /*0240*/                   STG.E.64.STRONG.CTA [R2], R6 ;
        /*0250*/                   EXIT ;
        /*0260*/                   BRA 0x260;
        /*0270*/                   NOP;
