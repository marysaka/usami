        /*0000*/                   IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x48] ;
        /*0010*/                   I2F.F16 R6, 0x1 ;
        /*0020*/                   IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x58] ;
        /*0030*/                   ISETP.GT.U32.AND P1, PT, R1, 0x3, PT ;
        /*0040*/                   ISETP.GT.U32.AND P0, PT, R0, 0x3, PT ;
        /*0050*/                   I2F.F16 R7, 0x2 ;
        /*0060*/                   I2F.F16 R8, 0x3 ;
        /*0070*/                   @P1 MOV R3, c[0x0][0x44] ;
        /*0080*/                   @P1 IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x40] ;
        /*0090*/                   @P0 IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x50] ;
        /*00a0*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x54] ;
        /*00b0*/                   @P1 LDG.E.STRONG.SM R3, [R2] ;
        /*00c0*/                   I2F.F16 R5, 0x4 ;
        /*00d0*/                   PRMT R6, R6, 0x5410, R7 ;
        /*00e0*/                   @P0 LDG.E.STRONG.SM R4, [R0] ;
        /*00f0*/                   STL.64 [RZ], RZ ;
        /*0100*/                   PRMT R7, R8, 0x5410, R5 ;
        /*0110*/                   STL.64 [RZ], R6 ;
        /*0120*/                   NOP ;
        /*0130*/                   NOP ;
        /*0140*/                   LDL.64 R6, [RZ] ;
        /*0150*/                   @!P1 MOV R2, RZ ;
        /*0160*/                   @P1 IMAD.SHL.U32 R2, R3, 0x2, RZ ;
        /*0170*/                   @!P0 CS2R R0, SRZ ;
        /*0180*/                   S2R R8, SR_LANEID ;
        /*0190*/                   @P0 IMAD.SHL.U32 R0, R4, 0x2, RZ ;
        /*01a0*/                   @P0 MOV R1, RZ ;
        /*01b0*/                   SHF.R.U32.HI R5, RZ, 0x2, R8 ;
        /*01c0*/                   LOP3.LUT R3, R8, 0x3, RZ, 0xc0, !PT ;
        /*01d0*/                   IMAD R4, R5, R2, RZ ;
        /*01e0*/                   IMAD R3, R3, 0x4, R4 ;
        /*01f0*/                   IADD3 R4, P0, R0, c[0x0][0x30], RZ ;
        /*0200*/                   IMAD R2, R2, 0x8, R3 ;
        /*0210*/                   IADD3.X R5, R1, c[0x0][0x34], RZ, P0, !PT ;
        /*0220*/                   IADD3 R0, P0, R3, R4.reuse, RZ ;
        /*0230*/                   IADD3 R2, P1, R2, R4, RZ ;
        /*0240*/                   IADD3.X R1, RZ, R5, RZ, P0, !PT ;
        /*0250*/                   IMAD.X R3, RZ, RZ, R5, P1 ;
        /*0260*/                   STG.E.STRONG.SM [R0], R6 ;
        /*0270*/                   STG.E.STRONG.SM [R2], R7 ;
        /*0280*/                   EXIT ;
        /*0290*/                   BRA 0x290;
        /*02a0*/                   NOP;
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
