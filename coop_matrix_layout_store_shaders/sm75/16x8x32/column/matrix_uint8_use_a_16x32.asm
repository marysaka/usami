        /*0000*/                   IMAD.MOV.U32 R3, RZ, RZ, c[0x0][0x48] ;
        /*0010*/                   STL.128 [RZ], RZ ;
        /*0020*/                   IMAD.MOV.U32 R0, RZ, RZ, 0x1 ;
        /*0030*/                   IMAD.MOV.U32 R1, RZ, RZ, 0x2 ;
        /*0040*/                   ISETP.GT.U32.AND P1, PT, R3, 0x3, PT ;
        /*0050*/                   IMAD.MOV.U32 R2, RZ, RZ, c[0x0][0x58] ;
        /*0060*/                   IMAD.MOV.U32 R3, RZ, RZ, 0x4 ;
        /*0070*/                   STL.U8 [RZ], R0 ;
        /*0080*/                   ISETP.GT.U32.AND P0, PT, R2, 0x3, PT ;
        /*0090*/                   IMAD.MOV.U32 R2, RZ, RZ, 0x3 ;
        /*00a0*/                   STL.U8 [0x1], R1 ;
        /*00b0*/                   IMAD.MOV.U32 R4, RZ, RZ, 0x5 ;
        /*00c0*/                   @!P1 IMAD.MOV.U32 R15, RZ, RZ, RZ ;
        /*00d0*/                   STL.U8 [0x3], R3 ;
        /*00e0*/                   @P1 IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x40] ;
        /*00f0*/                   @P1 IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x44] ;
        /*0100*/                   STL.U8 [0x2], R2 ;
        /*0110*/                   IMAD.MOV.U32 R5, RZ, RZ, 0x6 ;
        /*0120*/                   IMAD.MOV.U32 R7, RZ, RZ, 0x8 ;
        /*0130*/                   STL.U8 [0x4], R4 ;
        /*0140*/                   IMAD.MOV.U32 R8, RZ, RZ, 0x9 ;
        /*0150*/                   IMAD.MOV.U32 R9, RZ, RZ, 0xa ;
        /*0160*/                   @P1 LDG.E.STRONG.CTA R15, [R0] ;
        /*0170*/                   IMAD.MOV.U32 R10, RZ, RZ, 0xb ;
        /*0180*/                   IMAD.MOV.U32 R11, RZ, RZ, 0xc ;
        /*0190*/                   STL.U8 [0x5], R5 ;
        /*01a0*/                   IMAD.MOV.U32 R12, RZ, RZ, 0xd ;
        /*01b0*/                   PRMT R2, R9, 0x7610, R2 ;
        /*01c0*/                   IMAD.MOV.U32 R6, RZ, RZ, 0x7 ;
        /*01d0*/                   STL.U8 [0x7], R7 ;
        /*01e0*/                   @P0 IMAD.MOV.U32 R0, RZ, RZ, c[0x0][0x50] ;
        /*01f0*/                   PRMT R3, R10, 0x7610, R3 ;
        /*0200*/                   @P0 IMAD.MOV.U32 R1, RZ, RZ, c[0x0][0x54] ;
        /*0210*/                   STL.U8 [0x8], R8 ;
        /*0220*/                   PRMT R4, R11, 0x7610, R4 ;
        /*0230*/                   IMAD.MOV.U32 R9, RZ, RZ, 0x10 ;
        /*0240*/                   PRMT R5, R12, 0x7610, R5 ;
        /*0250*/                   IMAD.MOV.U32 R7, RZ, RZ, 0xe ;
        /*0260*/                   STL.U8 [0x6], R6 ;
        /*0270*/                   IMAD.MOV.U32 R8, RZ, RZ, 0xf ;
        /*0280*/                   STL.U8 [0x9], R2 ;
        /*0290*/                   @P0 LDG.E.STRONG.CTA R6, [R0] ;
        /*02a0*/                   STL.U8 [0xa], R3 ;
        /*02b0*/                   STL.U8 [0xb], R4 ;
        /*02c0*/                   STL.U8 [0xc], R5 ;
        /*02d0*/                   STL.U8 [0xd], R7 ;
        /*02e0*/                   STL.U8 [0xe], R8 ;
        /*02f0*/                   STL.U8 [0xf], R9 ;
        /*0300*/                   NOP ;
        /*0310*/                   NOP ;
        /*0320*/                   LDL.128 R0, [RZ] ;
        /*0330*/                   S2R R8, SR_LANEID ;
        /*0340*/                   IMAD.SHL.U32 R4, R8, 0x4, RZ ;
        /*0350*/                   LOP3.LUT R4, R4, 0xc, RZ, 0xe2, !PT ;
        /*0360*/                   IMAD R7, R4, R15, RZ ;
        /*0370*/                   @!P0 CS2R R4, SRZ ;
        /*0380*/                   LEA.HI R8, R8, R7, RZ, 0x1e ;
        /*0390*/                   @P0 IMAD.MOV.U32 R4, RZ, RZ, R6 ;
        /*03a0*/                   IADD3 R6, R8, 0x8, RZ ;
        /*03b0*/                   @P0 IMAD.MOV.U32 R5, RZ, RZ, RZ ;
        /*03c0*/                   IMAD R9, R15, 0x10, R8 ;
        /*03d0*/                   IADD3 R11, P0, R4, c[0x0][0x30], RZ ;
        /*03e0*/                   IADD3.X R12, R5, c[0x0][0x34], RZ, P0, !PT ;
        /*03f0*/                   IADD3 R4, P1, R8, R11.reuse, RZ ;
        /*0400*/                   IADD3 R10, R9.reuse, 0x8, RZ ;
        /*0410*/                   IADD3 R6, P2, R6, R11.reuse, RZ ;
        /*0420*/                   IMAD.X R5, RZ, RZ, R12.reuse, P1 ;
        /*0430*/                   IADD3 R8, P3, R9, R11.reuse, RZ ;
        /*0440*/                   IADD3 R10, P0, R10, R11, RZ ;
        /*0450*/                   IMAD.X R7, RZ, RZ, R12, P2 ;
        /*0460*/                   IMAD.X R9, RZ, RZ, R12.reuse, P3 ;
        /*0470*/                   IMAD.X R11, RZ, RZ, R12, P0 ;
        /*0480*/                   IADD3 R12, P0, R15, R8, RZ ;
        /*0490*/                   IMAD.X R13, RZ, RZ, R9, P0 ;
        /*04a0*/                   LOP3.LUT R17, R0.reuse, 0xff, RZ, 0xc0, !PT ;
        /*04b0*/                   SHF.R.U64 R19, R0.reuse, 0x8, R1.reuse ;
        /*04c0*/                   SHF.R.U64 R28, R0.reuse, 0x10, R1.reuse ;
        /*04d0*/                   SHF.R.U64 R26, R0, 0x18, R1 ;
        /*04e0*/                   IADD3 R0, P1, R15, R4, RZ ;
        /*04f0*/                   STG.E.U8.STRONG.CTA [R4], R17 ;
        /*0500*/                   PRMT R21, R1, 0x7770, RZ ;
        /*0510*/                   SHF.R.U32.HI R23, RZ, 0x8, R1.reuse ;
        /*0520*/                   SHF.R.U32.HI R24, RZ, 0x10, R1.reuse ;
        /*0530*/                   SHF.R.U32.HI R31, RZ, 0x18, R1 ;
        /*0540*/                   IMAD.X R1, RZ, RZ, R5, P1 ;
        /*0550*/                   LOP3.LUT R25, R2.reuse, 0xff, RZ, 0xc0, !PT ;
        /*0560*/                   STG.E.U8.STRONG.CTA [R6], R21 ;
        /*0570*/                   SHF.R.U64 R22, R2.reuse, 0x8, R3.reuse ;
        /*0580*/                   SHF.R.U64 R20, R2, 0x10, R3 ;
        /*0590*/                   SHF.R.U64 R18, R2, 0x18, R3.reuse ;
        /*05a0*/                   IADD3 R2, P1, R15, R6, RZ ;
        /*05b0*/                   STG.E.U8.STRONG.CTA [R8], R25 ;
        /*05c0*/                   PRMT R27, R3, 0x7770, RZ ;
        /*05d0*/                   SHF.R.U32.HI R16, RZ, 0x8, R3.reuse ;
        /*05e0*/                   SHF.R.U32.HI R14, RZ, 0x10, R3.reuse ;
        /*05f0*/                   SHF.R.U32.HI R29, RZ, 0x18, R3 ;
        /*0600*/                   IMAD.X R3, RZ, RZ, R7, P1 ;
        /*0610*/                   LOP3.LUT R19, R19, 0xff, RZ, 0xc0, !PT ;
        /*0620*/                   STG.E.U8.STRONG.CTA [R10], R27 ;
        /*0630*/                   IADD3 R6, P1, R15, R0, RZ ;
        /*0640*/                   IADD3 R4, P0, R15, R10, RZ ;
        /*0650*/                   LOP3.LUT R23, R23, 0xff, RZ, 0xc0, !PT ;
        /*0660*/                   IMAD.X R7, RZ, RZ, R1, P1 ;
        /*0670*/                   STG.E.U8.STRONG.CTA [R0], R19 ;
        /*0680*/                   IMAD.X R5, RZ, RZ, R11, P0 ;
        /*0690*/                   IADD3 R8, P1, R15.reuse, R2, RZ ;
        /*06a0*/                   LOP3.LUT R21, R22, 0xff, RZ, 0xc0, !PT ;
        /*06b0*/                   LOP3.LUT R25, R16, 0xff, RZ, 0xc0, !PT ;
        /*06c0*/                   IMAD.X R9, RZ, RZ, R3, P1 ;
        /*06d0*/                   LOP3.LUT R17, R28, 0xff, RZ, 0xc0, !PT ;
        /*06e0*/                   STG.E.U8.STRONG.CTA [R2], R23 ;
        /*06f0*/                   IADD3 R10, P1, R15, R6, RZ ;
        /*0700*/                   IADD3 R0, P0, R15.reuse, R12, RZ ;
        /*0710*/                   STG.E.U8.STRONG.CTA [R12], R21 ;
        /*0720*/                   LOP3.LUT R19, R24, 0xff, RZ, 0xc0, !PT ;
        /*0730*/                   IMAD.X R11, RZ, RZ, R7, P1 ;
        /*0740*/                   IMAD.X R1, RZ, RZ, R13, P0 ;
        /*0750*/                   STG.E.U8.STRONG.CTA [R4], R25 ;
        /*0760*/                   IADD3 R2, P0, R15, R4, RZ ;
        /*0770*/                   STG.E.U8.STRONG.CTA [R6], R17 ;
        /*0780*/                   LOP3.LUT R23, R20, 0xff, RZ, 0xc0, !PT ;
        /*0790*/                   IMAD.X R3, RZ, RZ, R5, P0 ;
        /*07a0*/                   IADD3 R12, P0, R15.reuse, R0, RZ ;
        /*07b0*/                   STG.E.U8.STRONG.CTA [R8], R19 ;
        /*07c0*/                   LOP3.LUT R21, R14, 0xff, RZ, 0xc0, !PT ;
        /*07d0*/                   IADD3 R4, P1, R15.reuse, R8, RZ ;
        /*07e0*/                   IMAD.X R13, RZ, RZ, R1, P0 ;
        /*07f0*/                   STG.E.U8.STRONG.CTA [R0], R23 ;
        /*0800*/                   IADD3 R6, P2, R15, R2, RZ ;
        /*0810*/                   LOP3.LUT R17, R26, 0xff, RZ, 0xc0, !PT ;
        /*0820*/                   IMAD.X R5, RZ, RZ, R9, P1 ;
        /*0830*/                   LOP3.LUT R15, R18, 0xff, RZ, 0xc0, !PT ;
        /*0840*/                   IMAD.X R7, RZ, RZ, R3, P2 ;
        /*0850*/                   STG.E.U8.STRONG.CTA [R2], R21 ;
        /*0860*/                   STG.E.U8.STRONG.CTA [R10], R17 ;
        /*0870*/                   STG.E.U8.STRONG.CTA [R4], R31 ;
        /*0880*/                   STG.E.U8.STRONG.CTA [R12], R15 ;
        /*0890*/                   STG.E.U8.STRONG.CTA [R6], R29 ;
        /*08a0*/                   EXIT ;
        /*08b0*/                   BRA 0x8b0;
        /*08c0*/                   NOP;
        /*08d0*/                   NOP;
        /*08e0*/                   NOP;
        /*08f0*/                   NOP;
