        /*0000*/                   IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x48] ;
        /*0010*/                   IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x58] ;
        /*0020*/                   ISETP.GT.U32.AND P1, PT, R1, 0x3, PT ;
        /*0030*/                   ISETP.GT.U32.AND P0, PT, R0, 0x3, PT ;
        /*0040*/                   @P1 IMAD.MOV.U32 R4, RZ, RZ, c[0x0][0x40] ;
        /*0050*/                   @P1 MOV R5, c[0x0][0x44] ;
        /*0060*/                   @P0 IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x50] ;
        /*0070*/                   @P0 IMAD.MOV.U32 R3, RZ, RZ, c[0x0][0x54] ;
        /*0080*/                   @P1 LDG.E.STRONG.SM R4, [R4] ;
        /*0090*/                   @P0 LDG.E.STRONG.SM R2, [R2] ;
        /*00a0*/                   @!P0 CS2R R0, SRZ ;
        /*00b0*/                   @!P1 MOV R7, RZ ;
        /*00c0*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, RZ ;
        /*00d0*/                   S2R R6, SR_LANEID ;
        /*00e0*/                   IMAD.MOV.U32 R9, RZ, RZ, 0x3f800000 ;
        /*00f0*/                   IMAD.SHL.U32 R8, R6.reuse, 0x2, RZ ;
        /*0100*/                   LOP3.LUT R6, R6, 0xfffffffc, RZ, 0xc0, !PT ;
        /*0110*/                   LOP3.LUT R8, R8, 0x6, RZ, 0xe2, !PT ;
        /*0120*/                   NOP ;
        /*0130*/                   @P1 IMAD.SHL.U32 R7, R4, 0x4, RZ ;
        /*0140*/                   @P0 SHF.L.U32 R0, R2, 0x2, RZ ;
        /*0150*/                   IMAD R6, R8, R7, R6 ;
        /*0160*/                   IADD3 R5, P0, R0, c[0x0][0x30], RZ ;
        /*0170*/                   IADD3 R4, R6.reuse, 0x20, RZ ;
        /*0180*/                   IADD3.X R2, R1, c[0x0][0x34], RZ, P0, !PT ;
        /*0190*/                   IADD3 R0, P0, R6, R5.reuse, RZ ;
        /*01a0*/                   IADD3 R4, P1, R4, R5, RZ ;
        /*01b0*/                   IADD3.X R1, RZ, R2, RZ, P0, !PT ;
        /*01c0*/                   NOP ;
        /*01d0*/                   IMAD.X R5, RZ, RZ, R2, P1 ;
        /*01e0*/                   IADD3 R2, P0, R7.reuse, R0, RZ ;
        /*01f0*/                   IMAD.MOV.U32 R13, RZ, RZ, 0x40400000 ;
        /*0200*/                   IADD3 R6, P1, R7, R4, RZ ;
        /*0210*/                   STG.E.STRONG.SM [R0], R9 ;
        /*0220*/                   MOV R11, 0x40000000 ;
        /*0230*/                   IMAD.X R3, RZ, RZ, R1, P0 ;
        /*0240*/                   MOV R15, 0x40800000 ;
        /*0250*/                   IMAD.X R7, RZ, RZ, R5, P1 ;
        /*0260*/                   STG.E.STRONG.SM [R2], R11 ;
        /*0270*/                   STG.E.STRONG.SM [R4], R13 ;
        /*0280*/                   STG.E.STRONG.SM [R6], R15 ;
        /*0290*/                   EXIT ;
        /*02a0*/                   BRA 0x2a0;
        /*02b0*/                   NOP;
        /*02c0*/                   NOP;
        /*02d0*/                   NOP;
        /*02e0*/                   NOP;
        /*02f0*/                   NOP;
        /*0300*/                   NOP;
        /*0310*/                   NOP;
        /*0320*/                   NOP;
        /*0330*/                   NOP;
        /*0340*/                   NOP;
        /*0350*/                   NOP;
        /*0360*/                   NOP;
        /*0370*/                   NOP;
