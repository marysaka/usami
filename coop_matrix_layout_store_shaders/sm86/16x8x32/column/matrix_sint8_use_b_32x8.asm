        /*0000*/                   IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x48] ;
        /*0010*/                   ISETP.GT.U32.AND P1, PT, R0, 0x3, PT ;
        /*0020*/                   IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x58] ;
        /*0030*/                   ISETP.GT.U32.AND P0, PT, R0, 0x3, PT ;
        /*0040*/                   @!P1 IMAD.MOV.U32 R6, RZ, RZ, RZ ;
        /*0050*/                   @P1 MOV R4, c[0x0][0x40] ;
        /*0060*/                   @P1 IMAD.MOV.U32 R5, RZ, RZ, c[0x0][0x44] ;
        /*0070*/                   @P0 IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x50] ;
        /*0080*/                   @P0 MOV R3, c[0x0][0x54] ;
        /*0090*/                   @P1 LDG.E.STRONG.SM R6, [R4] ;
        /*00a0*/                   @P0 LDG.E.STRONG.SM R2, [R2] ;
        /*00b0*/                   @!P0 CS2R R0, SRZ ;
        /*00c0*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, RZ ;
        /*00d0*/                   S2R R8, SR_LANEID ;
        /*00e0*/                   IMAD.MOV.U32 R5, RZ, RZ, 0x1010101 ;
        /*00f0*/                   SHF.R.U32.HI R7, RZ, 0x2, R8 ;
        /*0100*/                   NOP ;
        /*0110*/                   IMAD R7, R7, R6, RZ ;
        /*0120*/                   LOP3.LUT R6, R8, 0x3, RZ, 0xc0, !PT ;
        /*0130*/                   @P0 IMAD.MOV.U32 R0, RZ, RZ, R2 ;
        /*0140*/                   LEA R6, R6, R7, 0x2 ;
        /*0150*/                   IADD3 R8, P0, R0, c[0x0][0x30], RZ ;
        /*0160*/                   IADD3 R7, R6.reuse, 0x10, RZ ;
        /*0170*/                   IADD3.X R4, R1, c[0x0][0x34], RZ, P0, !PT ;
        /*0180*/                   IADD3 R0, P0, R6, R8.reuse, RZ ;
        /*0190*/                   IADD3 R2, P1, R7, R8, RZ ;
        /*01a0*/                   IMAD.X R1, RZ, RZ, R4, P0 ;
        /*01b0*/                   IADD3.X R3, RZ, R4, RZ, P1, !PT ;
        /*01c0*/                   NOP ;
        /*01d0*/                   STG.E.STRONG.SM [R0], R5 ;
        /*01e0*/                   STG.E.STRONG.SM [R2], R5 ;
        /*01f0*/                   EXIT ;
        /*0200*/                   BRA 0x200;
        /*0210*/                   NOP;
        /*0220*/                   NOP;
        /*0230*/                   NOP;
        /*0240*/                   NOP;
        /*0250*/                   NOP;
        /*0260*/                   NOP;
        /*0270*/                   NOP;
        /*0280*/                   NOP;
        /*0290*/                   NOP;
        /*02a0*/                   NOP;
        /*02b0*/                   NOP;
        /*02c0*/                   NOP;
        /*02d0*/                   NOP;
        /*02e0*/                   NOP;
        /*02f0*/                   NOP;
