        /*0000*/                   IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x58] ;
        /*0010*/                   MOV R0, 0xa1 ;
        /*0020*/                   IMAD.MOV.U32 R5, RZ, RZ, 0xa6 ;
        /*0030*/                   STL.128 [RZ], RZ ;
        /*0040*/                   IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x48] ;
        /*0050*/                   ISETP.GT.U32.AND P0, PT, R1, 0x3, PT ;
        /*0060*/                   IMAD.MOV.U32 R1, RZ, RZ, 0xa2 ;
        /*0070*/                   STL.U8 [RZ], R0 ;
        /*0080*/                   MOV R3, 0xa4 ;
        /*0090*/                   IMAD.MOV.U32 R7, RZ, RZ, 0xab ;
        /*00a0*/                   ISETP.GT.U32.AND P1, PT, R2, 0x3, PT ;
        /*00b0*/                   STL.U8 [0x5], R5 ;
        /*00c0*/                   IMAD.MOV.U32 R2, RZ, RZ, 0xa3 ;
        /*00d0*/                   MOV R9, 0xb0 ;
        /*00e0*/                   IMAD.MOV.U32 R4, RZ, RZ, 0xa5 ;
        /*00f0*/                   MOV R0, 0xa7 ;
        /*0100*/                   STL.U8 [0x1], R1 ;
        /*0110*/                   IMAD.MOV.U32 R6, RZ, RZ, 0xa8 ;
        /*0120*/                   MOV R5, 0xad ;
        /*0130*/                   STL.U8 [0x6], R0 ;
        /*0140*/                   IMAD.MOV.U32 R8, RZ, RZ, 0xaf ;
        /*0150*/                   IMAD.MOV.U32 R1, RZ, RZ, 0xa9 ;
        /*0160*/                   STL.U8 [0x3], R3 ;
        /*0170*/                   PRMT R0, R5, 0x7610, R0 ;
        /*0180*/                   STL.U8 [0x2], R2 ;
        /*0190*/                   PRMT R3, R7, 0x7610, R3 ;
        /*01a0*/                   STL.U8 [0x8], R1 ;
        /*01b0*/                   IMAD.MOV.U32 R7, RZ, RZ, 0xae ;
        /*01c0*/                   MOV R2, 0xaa ;
        /*01d0*/                   STL.U8 [0xc], R0 ;
        /*01e0*/                   PRMT R5, R7, 0x7610, R5 ;
        /*01f0*/                   PRMT R1, R9, 0x7610, R1 ;
        /*0200*/                   STL.U8 [0x4], R4 ;
        /*0210*/                   MOV R7, 0xb3 ;
        /*0220*/                   IMAD.MOV.U32 R0, RZ, RZ, 0xb5 ;
        /*0230*/                   STL.U8 [0xa], R3 ;
        /*0240*/                   STL.U8 [0x7], R6 ;
        /*0250*/                   IMAD.MOV.U32 R4, RZ, RZ, 0xac ;
        /*0260*/                   STL.U8 [0x9], R2 ;
        /*0270*/                   IMAD.MOV.U32 R3, RZ, RZ, 0xb2 ;
        /*0280*/                   STL.U8 [0xf], R1 ;
        /*0290*/                   PRMT R6, R8, 0x7610, R6 ;
        /*02a0*/                   IMAD.MOV.U32 R8, RZ, RZ, 0xb4 ;
        /*02b0*/                   IMAD.MOV.U32 R2, RZ, RZ, 0xb1 ;
        /*02c0*/                   STL.64 [0x10], RZ ;
        /*02d0*/                   PRMT R1, R0, 0x7610, R1 ;
        /*02e0*/                   STL.U8 [0xb], R4 ;
        /*02f0*/                   @P0 MOV R0, c[0x0][0x50] ;
        /*0300*/                   STL.U8 [0x11], R3 ;
        /*0310*/                   PRMT R4, R7, 0x7610, R4 ;
        /*0320*/                   STL.U8 [0x10], R2 ;
        /*0330*/                   PRMT R7, R8, 0x7610, R7 ;
        /*0340*/                   IMAD.MOV.U32 R8, RZ, RZ, 0xb8 ;
        /*0350*/                   @P1 MOV R3, c[0x0][0x44] ;
        /*0360*/                   STL.U8 [0x14], R1 ;
        /*0370*/                   STL.U8 [0xd], R5 ;
        /*0380*/                   @P1 IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x40] ;
        /*0390*/                   STL.U8 [0xe], R6 ;
        /*03a0*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x54] ;
        /*03b0*/                   MOV R5, 0xb6 ;
        /*03c0*/                   STL.U8 [0x12], R4 ;
        /*03d0*/                   IMAD.MOV.U32 R6, RZ, RZ, 0xb7 ;
        /*03e0*/                   @P1 LDG.E.STRONG.SM R3, [R2] ;
        /*03f0*/                   @P0 LDG.E.STRONG.SM R4, [R0] ;
        /*0400*/                   STL.U8 [0x13], R7 ;
        /*0410*/                   STL.U8 [0x15], R5 ;
        /*0420*/                   STL.U8 [0x16], R6 ;
        /*0430*/                   STL.U8 [0x17], R8 ;
        /*0440*/                   NOP ;
        /*0450*/                   NOP ;
        /*0460*/                   LDL.64 R0, [0x10] ;
        /*0470*/                   LDL.128 R8, [RZ] ;
        /*0480*/                   IMAD.MOV.U32 R12, RZ, RZ, 0xc1 ;
        /*0490*/                   MOV R14, 0xc3 ;
        /*04a0*/                   IMAD.MOV.U32 R13, RZ, RZ, 0xc2 ;
        /*04b0*/                   S2R R6, SR_LANEID ;
        /*04c0*/                   IMAD.MOV.U32 R15, RZ, RZ, 0xc4 ;
        /*04d0*/                   @!P1 IMAD.MOV.U32 R2, RZ, RZ, RZ ;
        /*04e0*/                   @P1 SHF.L.U32 R2, R3, 0x2, RZ ;
        /*04f0*/                   SHF.R.U32.HI R5, RZ, 0x2, R6 ;
        /*0500*/                   LOP3.LUT R3, R6, 0x3, RZ, 0xc0, !PT ;
        /*0510*/                   IMAD R6, R5, R2, RZ ;
        /*0520*/                   LEA R3, R3, R6, 0x3 ;
        /*0530*/                   LEA R6, R2, R3, 0x3 ;
        /*0540*/                   NOP ;
        /*0550*/                   IMMA.16832.U8.U8 R8, R8.ROW, R0.COL, R12 ;
        /*0560*/                   @!P0 CS2R R0, SRZ ;
        /*0570*/                   @P0 IMAD.SHL.U32 R0, R4, 0x4, RZ ;
        /*0580*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, RZ ;
        /*0590*/                   IADD3 R7, P0, R0, c[0x0][0x30], RZ ;
        /*05a0*/                   IADD3.X R12, R1, c[0x0][0x34], RZ, P0, !PT ;
        /*05b0*/                   IADD3 R2, P0, R3, R7.reuse, RZ ;
        /*05c0*/                   IADD3 R6, P1, R6, R7, RZ ;
        /*05d0*/                   IMAD.MOV.U32 R0, RZ, RZ, R8 ;
        /*05e0*/                   MOV R1, R9 ;
        /*05f0*/                   IMAD.MOV.U32 R4, RZ, RZ, R10 ;
        /*0600*/                   MOV R5, R11 ;
        /*0610*/                   IMAD.X R3, RZ, RZ, R12, P0 ;
        /*0620*/                   IADD3.X R7, RZ, R12, RZ, P1, !PT ;
        /*0630*/                   NOP ;
        /*0640*/                   STG.E.64.STRONG.SM [R2], R0 ;
        /*0650*/                   STG.E.64.STRONG.SM [R6], R4 ;
        /*0660*/                   EXIT ;
        /*0670*/                   BRA 0x670;
        /*0680*/                   NOP;
        /*0690*/                   NOP;
        /*06a0*/                   NOP;
        /*06b0*/                   NOP;
        /*06c0*/                   NOP;
        /*06d0*/                   NOP;
        /*06e0*/                   NOP;
        /*06f0*/                   NOP;
