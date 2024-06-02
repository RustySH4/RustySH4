// THIS CODE WAS GENERATED BY SH4GENERATOR v0.1.0 BY FRANCISZEK ŁOPUSZAŃSKI
#![allow(unused)]
use crate::{
    cpu::CPU,
    opcodes::{OpCode, OpCodeArgs},
};

#[allow(non_snake_case)]
impl CPU {
    fn sign_extend(x: u16) -> u32 {
        if (x & 0x80) == 0 {
            x as u32 & 0x000_000FF
        } else {
            x as u32 | 0xFFFF_FF00
        }
    }

    pub fn execute(&mut self, opcode: OpCode) {
        (opcode.callee)(self, opcode.args)
    }

    pub fn MOV(&mut self, args: OpCodeArgs) {
        /* Rm -> Rn */
        self.registers.r[args.n as usize] = self.registers.r[args.m as usize];
        self.registers.pc += 2;
    }

    pub fn MOVI(&mut self, args: OpCodeArgs) {
        /* imm -> sign extension -> Rn */
        self.registers.r[args.n as usize] = CPU::sign_extend(args.i);
        self.registers.pc += 2;
    }

    pub fn MOVA(&mut self, args: OpCodeArgs) {
        /*
         (disp*4) + (PC & 0xFFFFFFFC) + 4 -> R0
        */
        todo!()
    }

    pub fn MOVWI(&mut self, args: OpCodeArgs) {
        /*
         (disp*2 + PC + 4) -> sign extension -> Rn
        */
        todo!()
    }

    pub fn MOVLI(&mut self, args: OpCodeArgs) {
        /*
         (disp*4 + (PC & 0xFFFFFFFC) + 4) -> sign extension -> Rn
        */
        todo!()
    }

    pub fn MOVBL(&mut self, args: OpCodeArgs) {
        /* (Rm) -> sign extension -> Rn */
        todo!()
    }

    pub fn MOVWL(&mut self, args: OpCodeArgs) {
        /* (Rm) -> sign extension -> Rn */
        todo!()
    }

    pub fn MOVLL(&mut self, args: OpCodeArgs) {
        /* (Rm) -> Rn */
        todo!()
    }

    pub fn MOVBS(&mut self, args: OpCodeArgs) {
        /* Rm -> (Rn) */
        todo!()
    }

    pub fn MOVWS(&mut self, args: OpCodeArgs) {
        /* Rm -> (Rn) */
        todo!()
    }

    pub fn MOVLS(&mut self, args: OpCodeArgs) {
        /* Rm -> (Rn) */
        self.bus.write32(
            self.registers.r[args.n as usize] as usize,
            self.registers.r[args.m as usize],
        );
        self.registers.pc += 2;
    }

    pub fn MOVBP(&mut self, args: OpCodeArgs) {
        /*
         (Rm) -> sign extension -> Rn, Rm+1 -> Rm
        */
        todo!()
    }

    pub fn MOVWP(&mut self, args: OpCodeArgs) {
        /*
         (Rm) -> sign extension -> Rn, Rm+2 -> Rm
        */
        todo!()
    }

    pub fn MOVLP(&mut self, args: OpCodeArgs) {
        /* (Rm) -> Rn, Rm+4 -> Rm */
        self.registers.r[args.n as usize] = self.bus.read32(args.m as usize).unwrap();

        if args.n != args.m {
            self.registers.r[args.m as usize] += 4;
        }

        self.registers.pc += 2;
    }

    pub fn MOVBM(&mut self, args: OpCodeArgs) {
        /* Rn-1 -> Rn, Rm -> (Rn) */
        todo!()
    }

    pub fn MOVWM(&mut self, args: OpCodeArgs) {
        /* Rn-2 -> Rn, Rm -> (Rn) */
        todo!()
    }

    pub fn MOVLM(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, Rm -> (Rn) */
        self.bus.write32(
            self.registers.r[args.n as usize].wrapping_sub(4) as usize,
            self.registers.r[args.m as usize],
        );
        self.registers.r[args.n as usize] = self.registers.r[args.n as usize].wrapping_sub(4);

        self.registers.pc += 2;
    }

    pub fn MOVBL4(&mut self, args: OpCodeArgs) {
        /* (disp + Rm) -> sign extension -> R0 */
        todo!()
    }

    pub fn MOVWL4(&mut self, args: OpCodeArgs) {
        /*
         (disp*2 + Rm) -> sign extension -> R0
        */
        let disp = (0x0000000F & args.d as u32);

        self.registers.r[0] = self
            .bus
            .read16((self.registers.r[args.m as usize] + (disp << 1)) as usize)
            .unwrap() as u32;

        if self.registers.r[0] & 0x8000 == 0 {
            self.registers.r[0] &= 0x0000FFFF;
        } else {
            self.registers.r[0] |= 0xFFFF0000;
        }

        self.registers.pc += 2;
    }

    pub fn MOVLL4(&mut self, args: OpCodeArgs) {
        /* (disp*4 + Rm) -> Rn */
        let disp = (0x0000000F & args.d as u32);

        self.registers.r[args.n as usize] = self
            .bus
            .read32((self.registers.r[args.m as usize] + (disp << 2)) as usize)
            .unwrap();

        self.registers.pc += 2;
    }

    pub fn MOVBS4(&mut self, args: OpCodeArgs) {
        /* R0 -> (disp + Rn) */
        todo!()
    }

    pub fn MOVWS4(&mut self, args: OpCodeArgs) {
        /* R0 -> (disp*2 + Rn) */
        let disp = (0x0000000F & args.d as u32);

        self.bus.write16(
            (self.registers.r[args.n as usize] + (disp << 1)) as usize,
            self.registers.r[0] as u16,
        );

        self.registers.pc += 2;
    }

    pub fn MOVLS4(&mut self, args: OpCodeArgs) {
        /* Rm -> (disp*4 + Rn) */
        let disp = (0x0000000F & args.d as u32);

        self.bus.write32(
            (self.registers.r[args.n as usize] + (disp << 2)) as usize,
            self.registers.r[args.m as usize],
        );

        self.registers.pc += 2;
    }

    pub fn MOVBL0(&mut self, args: OpCodeArgs) {
        /* (R0 + Rm) -> sign extension -> Rn */
        todo!()
    }

    pub fn MOVWL0(&mut self, args: OpCodeArgs) {
        /* (R0 + Rm) -> sign extension -> Rn */
        todo!()
    }

    pub fn MOVLL0(&mut self, args: OpCodeArgs) {
        /* (R0 + Rm) -> Rn */
        todo!()
    }

    pub fn MOVBS0(&mut self, args: OpCodeArgs) {
        /* Rm -> (R0 + Rn) */
        todo!()
    }

    pub fn MOVWS0(&mut self, args: OpCodeArgs) {
        /* Rm -> (R0 + Rn) */
        todo!()
    }

    pub fn MOVLS0(&mut self, args: OpCodeArgs) {
        /* Rm -> (R0 + Rn) */
        todo!()
    }

    pub fn MOVBLG(&mut self, args: OpCodeArgs) {
        /* (disp + GBR) -> sign extension -> R0 */
        todo!()
    }

    pub fn MOVWLG(&mut self, args: OpCodeArgs) {
        /*
         (disp*2 + GBR) -> sign extension -> R0
        */
        todo!()
    }

    pub fn MOVLLG(&mut self, args: OpCodeArgs) {
        /* (disp*4 + GBR) -> R0 */
        todo!()
    }

    pub fn MOVBSG(&mut self, args: OpCodeArgs) {
        /* R0 -> (disp + GBR) */
        todo!()
    }

    pub fn MOVWSG(&mut self, args: OpCodeArgs) {
        /* R0 -> (disp*2 + GBR) */
        todo!()
    }

    pub fn MOVLSG(&mut self, args: OpCodeArgs) {
        /* R0 -> (disp*4 + GBR) */
        todo!()
    }

    pub fn MOVCO(&mut self, args: OpCodeArgs) {
        /*
         LDST -> T If (T == 1): R0 -> Rn 0 -> LDST
        */
        todo!()
    }

    pub fn MOVLINK(&mut self, args: OpCodeArgs) {
        /*
         1 -> LDST (Rm) -> R0 When interrupt/exception occured: 0 ->
         LDST
        */
        todo!()
    }

    pub fn MOVUAL(&mut self, args: OpCodeArgs) {
        /*
         (Rm) -> R0 Load non-boundary alignment data
        */
        todo!()
    }

    pub fn MOVUALP(&mut self, args: OpCodeArgs) {
        /*
         (Rm) -> R0, Rm + 4 -> Rm Load non-boundary alignment data
        */
        todo!()
    }

    pub fn MOVT(&mut self, args: OpCodeArgs) {
        /* T -> Rn */
        todo!()
    }

    pub fn SWAPB(&mut self, args: OpCodeArgs) {
        /* Rm -> swap lower 2 bytes -> Rn */
        todo!()
    }

    pub fn SWAPW(&mut self, args: OpCodeArgs) {
        /* Rm -> swap upper/lower words -> Rn */
        todo!()
    }

    pub fn XTRCT(&mut self, args: OpCodeArgs) {
        /* Rm:Rn middle 32 bits -> Rn */
        todo!()
    }

    pub fn ADD(&mut self, args: OpCodeArgs) {
        /* Rn + Rm -> Rn */
        self.registers.r[args.n as usize] =
            self.registers.r[args.n as usize].wrapping_add(self.registers.r[args.m as usize]);
        self.registers.pc += 2;
    }

    pub fn ADDI(&mut self, args: OpCodeArgs) {
        /* Rn + (sign extension)imm */
        self.registers.r[args.n as usize] =
            self.registers.r[args.n as usize].wrapping_add(CPU::sign_extend(args.i));
        self.registers.pc += 2;
    }

    pub fn ADDC(&mut self, args: OpCodeArgs) {
        /* Rn + Rm + T -> Rn, carry -> T */
        todo!()
    }

    pub fn ADDV(&mut self, args: OpCodeArgs) {
        /* Rn + Rm -> Rn, overflow -> T */
        todo!()
    }

    pub fn CMPIM(&mut self, args: OpCodeArgs) {
        /*
         If R0 = (sign extension)imm: 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn CMPEQ(&mut self, args: OpCodeArgs) {
        /* If Rn = Rm: 1 -> T Else: 0 -> T */
        todo!()
    }

    pub fn CMPHI(&mut self, args: OpCodeArgs) {
        /*
         If Rn >= Rm (unsigned): 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn CMPGE(&mut self, args: OpCodeArgs) {
        /*
         If Rn >= Rm (signed): 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn CMPHI_DUP(&mut self, args: OpCodeArgs) {
        /*
         If Rn > Rm (unsigned): 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn CMPGT(&mut self, args: OpCodeArgs) {
        /*
         If Rn > Rm (signed): 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn CMPPL(&mut self, args: OpCodeArgs) {
        /*
         If Rn > 0 (signed): 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn CMPPZ(&mut self, args: OpCodeArgs) {
        /*
         If Rn >= 0 (signed): 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn CMPSTR(&mut self, args: OpCodeArgs) {
        /*
         If Rn and Rm have an equal byte: 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn DIV0S(&mut self, args: OpCodeArgs) {
        /*
         MSB of Rn -> Q, MSB of Rm -> M, M ^ Q -> T
        */
        todo!()
    }

    pub fn DIV0U(&mut self, args: OpCodeArgs) {
        /* 0 -> M, 0 -> Q, 0 -> T */
        todo!()
    }

    pub fn DIV1(&mut self, args: OpCodeArgs) {
        /* 1-step division (Rn / Rm) */
        todo!()
    }

    pub fn DMULS(&mut self, args: OpCodeArgs) {
        /*
         Signed, Rn * Rm -> MACH:MACL 32 * 32 -> 64 bits
        */
        todo!()
    }

    pub fn DMULU(&mut self, args: OpCodeArgs) {
        /*
         Unsigned, Rn * Rm -> MACH:MACL 32 * 32 -> 64 bits
        */
        todo!()
    }

    pub fn DT(&mut self, args: OpCodeArgs) {
        /*
         Rn-1 -> Rn If Rn = 0: 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn EXTSB(&mut self, args: OpCodeArgs) {
        /* Rm sign-extended from byte -> Rn */
        todo!()
    }

    pub fn EXTSW(&mut self, args: OpCodeArgs) {
        /* Rm sign-extended from word -> Rn */
        self.registers.r[args.n as usize] = self.registers.r[args.m as usize];

        if self.registers.r[args.m as usize] & 0x00008000 == 0 {
            self.registers.r[args.n as usize] &= 0x0000FFFF;
        } else {
            self.registers.r[args.n as usize] |= 0xFFFF0000;
        }

        self.registers.pc += 2;
    }

    pub fn EXTUB(&mut self, args: OpCodeArgs) {
        /* Rm zero-extended from byte -> Rn */
        todo!()
    }

    pub fn EXTUW(&mut self, args: OpCodeArgs) {
        /* Rm zero-extended from word -> Rn */
        self.registers.r[args.n as usize] = self.registers.r[args.m as usize];
        self.registers.r[args.n as usize] &= 0x0000FFFF;

        self.registers.pc += 2;
    }

    pub fn MACL(&mut self, args: OpCodeArgs) {
        /*
         Signed, (Rn) * (Rm) + MAC -> MAC 32 * 32 + 64 -> 64 bits
        */
        todo!()
    }

    pub fn MACW(&mut self, args: OpCodeArgs) {
        /*
         Signed, (Rn) * (Rm) + MAC -> MAC SH1: 16 * 16 + 42 -> 42 bits
         Other: 16 * 16 + 64 -> 64 bits
        */
        todo!()
    }

    pub fn MULL(&mut self, args: OpCodeArgs) {
        /* Rn * Rm -> MACL 32 * 32 -> 32 bits */
        todo!()
    }

    pub fn MULS(&mut self, args: OpCodeArgs) {
        /*
         Signed, Rn * Rm -> MACL 16 * 16 -> 32 bits
        */
        todo!()
    }

    pub fn MULU(&mut self, args: OpCodeArgs) {
        /*
         Unsigned, Rn * Rm -> MACL 16 * 16 -> 32 bits
        */
        todo!()
    }

    pub fn NEG(&mut self, args: OpCodeArgs) {
        /* 0 - Rm -> Rn */
        todo!()
    }

    pub fn NEGC(&mut self, args: OpCodeArgs) {
        /* 0 - Rm - T -> Rn, borrow -> T */
        todo!()
    }

    pub fn SUB(&mut self, args: OpCodeArgs) {
        /* Rn - Rm -> Rn */
        todo!()
    }

    pub fn SUBC(&mut self, args: OpCodeArgs) {
        /* Rn - Rm - T -> Rn, borrow -> T */
        todo!()
    }

    pub fn SUBV(&mut self, args: OpCodeArgs) {
        /* Rn - Rm -> Rn, underflow -> T */
        todo!()
    }

    pub fn AND(&mut self, args: OpCodeArgs) {
        /* Rn & Rm -> Rn */
        todo!()
    }

    pub fn ANDI(&mut self, args: OpCodeArgs) {
        /* R0 & (zero extend)imm -> R0 */
        todo!()
    }

    pub fn ANDM(&mut self, args: OpCodeArgs) {
        /*
         (R0 + GBR) & (zero extend)imm -> (R0 + GBR)
        */
        todo!()
    }

    pub fn NOT(&mut self, args: OpCodeArgs) {
        /* ~Rm -> Rn */
        todo!()
    }

    pub fn OR(&mut self, args: OpCodeArgs) {
        /* Rn | Rm -> Rn */
        todo!()
    }

    pub fn ORI(&mut self, args: OpCodeArgs) {
        /* R0 | (zero extend)imm -> R0 */
        todo!()
    }

    pub fn ORM(&mut self, args: OpCodeArgs) {
        /*
         (R0 + GBR) | (zero extend)imm -> (R0 + GBR)
        */
        todo!()
    }

    pub fn TAS(&mut self, args: OpCodeArgs) {
        /*
         If (Rn) = 0: 1 -> T Else: 0 -> T 1 -> MSB of (Rn)
        */
        todo!()
    }

    pub fn TST(&mut self, args: OpCodeArgs) {
        /*
         If Rn & Rm = 0: 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn TSTI(&mut self, args: OpCodeArgs) {
        /*
         If R0 & (zero extend)imm = 0: 1 -> T Else: 0 -> T
        */
        todo!()
    }

    pub fn TSTM(&mut self, args: OpCodeArgs) {
        /*
         If (R0 + GBR) & (zero extend)imm = 0: 1 -> T Else 0: -> T
        */
        todo!()
    }

    pub fn XOR(&mut self, args: OpCodeArgs) {
        /* Rn ^ Rm -> Rn */
        todo!()
    }

    pub fn XORI(&mut self, args: OpCodeArgs) {
        /* R0 ^ (zero extend)imm -> R0 */
        todo!()
    }

    pub fn XORM(&mut self, args: OpCodeArgs) {
        /*
         (R0 + GBR) ^ (zero extend)imm -> (R0 + GBR)
        */
        todo!()
    }

    pub fn ROTCL(&mut self, args: OpCodeArgs) {
        /* T << Rn << T */
        todo!()
    }

    pub fn ROTCR(&mut self, args: OpCodeArgs) {
        /* T >> Rn >> T */
        todo!()
    }

    pub fn ROTL(&mut self, args: OpCodeArgs) {
        /* T << Rn << MSB */
        todo!()
    }

    pub fn ROTR(&mut self, args: OpCodeArgs) {
        /* LSB >> Rn >> T */
        todo!()
    }

    pub fn SHAD(&mut self, args: OpCodeArgs) {
        /*
         If Rm >= 0: Rn << Rm -> Rn If Rm < 0: Rn >> |Rm|
         -> [MSB -> Rn]
        */
        todo!()
    }

    pub fn SHAL(&mut self, args: OpCodeArgs) {
        /* T << Rn << 0 */
        todo!()
    }

    pub fn SHAR(&mut self, args: OpCodeArgs) {
        /* MSB >> Rn >> T */
        todo!()
    }

    pub fn SHLD(&mut self, args: OpCodeArgs) {
        /*
         If Rm >= 0: Rn << Rm -> Rn If Rm < 0: Rn >> |Rm|
         -> [0 -> Rn]
        */
        todo!()
    }

    pub fn SHLL(&mut self, args: OpCodeArgs) {
        /* T << Rn << 0 */
        todo!()
    }

    pub fn SHLL2(&mut self, args: OpCodeArgs) {
        /* Rn << 2 -> Rn */
        todo!()
    }

    pub fn SHLL8(&mut self, args: OpCodeArgs) {
        /* Rn << 8 -> Rn */
        todo!()
    }

    pub fn SHLL16(&mut self, args: OpCodeArgs) {
        /* Rn << 16 -> Rn */
        todo!()
    }

    pub fn SHLR(&mut self, args: OpCodeArgs) {
        /* 0 >> Rn >> T */
        todo!()
    }

    pub fn SHLR2(&mut self, args: OpCodeArgs) {
        /* Rn >> 2 -> [0 -> Rn] */
        todo!()
    }

    pub fn SHLR8(&mut self, args: OpCodeArgs) {
        /* Rn >> 8 -> [0 -> Rn] */
        todo!()
    }

    pub fn SHLR16(&mut self, args: OpCodeArgs) {
        /* Rn >> 16 -> [0 -> Rn] */
        todo!()
    }

    pub fn BF(&mut self, args: OpCodeArgs) {
        /*
         If T = 0: disp*2 + PC + 4 -> PC Else: nop
        */
        todo!()
    }

    pub fn BFS(&mut self, args: OpCodeArgs) {
        /*
         If T = 0: disp*2 + PC + 4 -> PC Else: nop (Delayed branch)
        */
        todo!()
    }

    pub fn BT(&mut self, args: OpCodeArgs) {
        /*
         If T = 1: disp*2 + PC + 4 -> PC Else: nop
        */
        todo!()
    }

    pub fn BTS(&mut self, args: OpCodeArgs) {
        /*
         If T = 1: disp*2 + PC + 4 -> PC Else: nop (Delayed branch)
        */
        todo!()
    }

    pub fn BRA(&mut self, args: OpCodeArgs) {
        /* disp*2 + PC + 4 -> PC (Delayed branch) */
        todo!()
    }

    pub fn BRAF(&mut self, args: OpCodeArgs) {
        /* Rm + PC + 4 -> PC (Delayed branch) */
        todo!()
    }

    pub fn BSR(&mut self, args: OpCodeArgs) {
        /*
         PC + 4 -> PR, disp*2 + PC + 4 -> PC (Delayed branch)
        */
        todo!()
    }

    pub fn BSRF(&mut self, args: OpCodeArgs) {
        /*
         PC + 4 -> PR, Rm + PC + 4 -> PC (Delayed branch)
        */
        todo!()
    }

    pub fn JMP(&mut self, args: OpCodeArgs) {
        /* Rm -> PC (Delayed branch) */
        todo!()
    }

    pub fn JSR(&mut self, args: OpCodeArgs) {
        /*
         PC + 4 -> PR, Rm -> PC (Delayed branch)
        */
        todo!()
    }

    pub fn RTS(&mut self, args: OpCodeArgs) {
        /* PR -> PC Delayed branch */
        let temp: u32 = self.registers.pc;
        self.registers.pc = self.registers.pr;
        self.delay_slot(temp as usize + 2);
    }

    pub fn CLRMAC(&mut self, args: OpCodeArgs) {
        /* 0 -> MACH, 0 -> MACL */
        todo!()
    }

    pub fn CLRS(&mut self, args: OpCodeArgs) {
        /* 0 -> S */
        todo!()
    }

    pub fn CLRT(&mut self, args: OpCodeArgs) {
        /* 0 -> T */
        todo!()
    }

    pub fn ICBI(&mut self, args: OpCodeArgs) {
        /*
         Invalidate instruction cache block indicated by logical address
        */
        todo!()
    }

    pub fn LDCSR(&mut self, args: OpCodeArgs) {
        /* Rm -> SR */
        todo!()
    }

    pub fn LDCMSR(&mut self, args: OpCodeArgs) {
        /* (Rm) -> SR, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDCGBR(&mut self, args: OpCodeArgs) {
        /* Rm -> GBR */
        todo!()
    }

    pub fn LDCMGBR(&mut self, args: OpCodeArgs) {
        /* (Rm) -> GBR, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDCVBR(&mut self, args: OpCodeArgs) {
        /* Rm -> VBR */
        todo!()
    }

    pub fn LDCMVBR(&mut self, args: OpCodeArgs) {
        /* (Rm) -> VBR, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDCSGR(&mut self, args: OpCodeArgs) {
        /* Rm -> SGR */
        todo!()
    }

    pub fn LDCMSGR(&mut self, args: OpCodeArgs) {
        /* (Rm) -> SGR, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDCSSR(&mut self, args: OpCodeArgs) {
        /* Rm -> SSR */
        todo!()
    }

    pub fn LDCMSSR(&mut self, args: OpCodeArgs) {
        /* (Rm) -> SSR, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDCSPC(&mut self, args: OpCodeArgs) {
        /* Rm -> SPC */
        todo!()
    }

    pub fn LDCMSPC(&mut self, args: OpCodeArgs) {
        /* (Rm) -> SPC, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDCDBR(&mut self, args: OpCodeArgs) {
        /* Rm -> DBR */
        todo!()
    }

    pub fn LDCMDBR(&mut self, args: OpCodeArgs) {
        /* (Rm) -> DBR, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDCRn_BANK(&mut self, args: OpCodeArgs) {
        /* Rm -> Rn_BANK (n = 0-7) */
        todo!()
    }

    pub fn LDCMRn_BANK(&mut self, args: OpCodeArgs) {
        /* (Rm) -> Rn_BANK, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDSMACH(&mut self, args: OpCodeArgs) {
        /* Rm -> MACH */
        todo!()
    }

    pub fn LDSMMACH(&mut self, args: OpCodeArgs) {
        /* (Rm) -> MACH, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDSMACL(&mut self, args: OpCodeArgs) {
        /* Rm -> MACL */
        todo!()
    }

    pub fn LDSMMACL(&mut self, args: OpCodeArgs) {
        /* (Rm) -> MACL, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDSPR(&mut self, args: OpCodeArgs) {
        /* Rm -> PR */
        todo!()
    }

    pub fn LDSMPR(&mut self, args: OpCodeArgs) {
        /* (Rm) -> PR, Rm+4 -> Rm */
        todo!()
    }

    pub fn LDTLB(&mut self, args: OpCodeArgs) {
        /* PTEH/PTEL -> TLB */
        todo!()
    }

    pub fn MOVCAL(&mut self, args: OpCodeArgs) {
        /*
         R0 -> (Rn) (without fetching cache block)
        */
        todo!()
    }

    pub fn NOP(&mut self, args: OpCodeArgs) {
        /* No operation */
        self.registers.pc += 2;
    }

    pub fn OCBI(&mut self, args: OpCodeArgs) {
        /* Invalidate operand cache block */
        todo!()
    }

    pub fn OCBP(&mut self, args: OpCodeArgs) {
        /*
         Write back and invalidate operand cache block
        */
        todo!()
    }

    pub fn OCBWB(&mut self, args: OpCodeArgs) {
        /* Write back operand cache block */
        todo!()
    }

    pub fn PREF(&mut self, args: OpCodeArgs) {
        /* (Rn) -> operand cache */
        todo!()
    }

    pub fn PREFI(&mut self, args: OpCodeArgs) {
        /*
         Reads 32-byte instruction block into instruction cache
        */
        todo!()
    }

    pub fn RTE(&mut self, args: OpCodeArgs) {
        /*
         Delayed branch SH1*,SH2*: stack area -> PC/SR SH3*,SH4*: SSR/SPC
         -> SR/PC
        */
        todo!()
    }

    pub fn SETS(&mut self, args: OpCodeArgs) {
        /* 1 -> S */
        todo!()
    }

    pub fn SETT(&mut self, args: OpCodeArgs) {
        /* 1 -> T */
        todo!()
    }

    pub fn SLEEP(&mut self, args: OpCodeArgs) {
        /* Sleep or standby */
        todo!()
    }

    pub fn STCSR(&mut self, args: OpCodeArgs) {
        /* SR -> Rn */
        todo!()
    }

    pub fn STCMSR(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, SR -> (Rn) */
        todo!()
    }

    pub fn STCGBR(&mut self, args: OpCodeArgs) {
        /* GBR -> Rn */
        todo!()
    }

    pub fn STCMGBR(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, GBR -> (Rn) */
        todo!()
    }

    pub fn STCVBR(&mut self, args: OpCodeArgs) {
        /* VBR -> Rn */
        todo!()
    }

    pub fn STCMVBR(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, VBR -> (Rn) */
        todo!()
    }

    pub fn STCSGR(&mut self, args: OpCodeArgs) {
        /* SGR -> Rn */
        todo!()
    }

    pub fn STCMSGR(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, SGR -> (Rn) */
        todo!()
    }

    pub fn STCSSR(&mut self, args: OpCodeArgs) {
        /* SSR -> Rn */
        todo!()
    }

    pub fn STCMSSR(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, SSR -> (Rn) */
        todo!()
    }

    pub fn STCSPC(&mut self, args: OpCodeArgs) {
        /* SPC -> Rn */
        todo!()
    }

    pub fn STCMSPC(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, SPC -> (Rn) */
        todo!()
    }

    pub fn STCDBR(&mut self, args: OpCodeArgs) {
        /* DBR -> Rn */
        todo!()
    }

    pub fn STCMDBR(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, DBR -> (Rn) */
        todo!()
    }

    pub fn STCRm_BANK(&mut self, args: OpCodeArgs) {
        /* Rm_BANK -> Rn (m = 0-7) */
        todo!()
    }

    pub fn STCMRm_BANK(&mut self, args: OpCodeArgs) {
        /*
         Rn-4 -> Rn, Rm_BANK -> (Rn) (m = 0-7)
        */
        todo!()
    }

    pub fn STSMACH(&mut self, args: OpCodeArgs) {
        /* MACH -> Rn */
        todo!()
    }

    pub fn STSMMACH(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, MACH -> (Rn) */
        todo!()
    }

    pub fn STSMACL(&mut self, args: OpCodeArgs) {
        /* MACL -> Rn */
        todo!()
    }

    pub fn STSMMACL(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, MACL -> (Rn) */
        todo!()
    }

    pub fn STSPR(&mut self, args: OpCodeArgs) {
        /* PR -> Rn */
        todo!()
    }

    pub fn STSMPR(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, PR -> (Rn) */
        todo!()
    }

    pub fn SYNCO(&mut self, args: OpCodeArgs) {
        /*
         Prevents the next instruction from being issued until instructions
         issued before this instruction has been completed.
        */
        todo!()
    }

    pub fn TRAPA(&mut self, args: OpCodeArgs) {
        /*
         SH1*,SH2*: PC/SR -> stack area, (imm*4 + VBR) -> PC SH3*,SH4*:
         PC/SR -> SPC/SSR, imm*4 -> TRA, 0x160 -> EXPEVT, VBR + 0x0100
         -> PC
        */
        todo!()
    }

    pub fn FMOV(&mut self, args: OpCodeArgs) {
        /* FRm -> FRn */
        todo!()
    }

    pub fn FMOV_LOAD(&mut self, args: OpCodeArgs) {
        /* (Rm) -> FRn */
        todo!()
    }

    pub fn FMOV_STORE(&mut self, args: OpCodeArgs) {
        /* FRm -> (Rn) */
        todo!()
    }

    pub fn FMOV_RESTORE(&mut self, args: OpCodeArgs) {
        /* (Rm) -> FRn, Rm+4 -> Rm */
        todo!()
    }

    pub fn FMOV_SAVE(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, FRm -> (Rn) */
        todo!()
    }

    pub fn FMOV_INDEX_LOAD(&mut self, args: OpCodeArgs) {
        /* (R0 + Rm) -> FRn */
        todo!()
    }

    pub fn FMOV_INDEX_STORE(&mut self, args: OpCodeArgs) {
        /* FRm -> (R0 + Rn) */
        todo!()
    }

    pub fn FMOV_DR(&mut self, args: OpCodeArgs) {
        /* DRm -> DRn */
        todo!()
    }

    pub fn FMOV_DRXD(&mut self, args: OpCodeArgs) {
        /* DRm -> XDn */
        todo!()
    }

    pub fn FMOV_XDDR(&mut self, args: OpCodeArgs) {
        /* XDm -> DRn */
        todo!()
    }

    pub fn FMOV_XDXD(&mut self, args: OpCodeArgs) {
        /* XDm -> XDn */
        todo!()
    }

    pub fn FMOV_LOAD_DR(&mut self, args: OpCodeArgs) {
        /* (Rm) -> DRn */
        todo!()
    }

    pub fn FMOV_LOAD_XD(&mut self, args: OpCodeArgs) {
        /* (Rm) -> XDn */
        todo!()
    }

    pub fn FMOV_STORE_DR(&mut self, args: OpCodeArgs) {
        /* DRm -> (Rn) */
        todo!()
    }

    pub fn FMOV_STORE_XD(&mut self, args: OpCodeArgs) {
        /* XDm -> (Rn) */
        todo!()
    }

    pub fn FMOV_RESTORE_DR(&mut self, args: OpCodeArgs) {
        /* (Rm) -> DRn, Rm + 8 -> Rm */
        todo!()
    }

    pub fn FMOV_RESTORE_XD(&mut self, args: OpCodeArgs) {
        /* (Rm) -> XDn, Rm+8 -> Rm */
        todo!()
    }

    pub fn FMOV_SAVE_DR(&mut self, args: OpCodeArgs) {
        /* Rn-8 -> Rn, DRm -> (Rn) */
        todo!()
    }

    pub fn FMOV_SAVE_XD(&mut self, args: OpCodeArgs) {
        /* Rn-8 -> Rn, (Rn) -> XDm */
        todo!()
    }

    pub fn FMOV_INDEX_LOAD_DR(&mut self, args: OpCodeArgs) {
        /* (R0 + Rm) -> DRn */
        todo!()
    }

    pub fn FMOV_INDEX_LOAD_XD(&mut self, args: OpCodeArgs) {
        /* (R0 + Rm) -> XDn */
        todo!()
    }

    pub fn FMOV_INDEX_STORE_DR(&mut self, args: OpCodeArgs) {
        /* DRm -> (R0 + Rn) */
        todo!()
    }

    pub fn FMOV_INDEX_STORE_XD(&mut self, args: OpCodeArgs) {
        /* XDm -> (R0 + Rn) */
        todo!()
    }

    pub fn FLDI0(&mut self, args: OpCodeArgs) {
        /* 0x00000000 -> FRn */
        todo!()
    }

    pub fn FLDI1(&mut self, args: OpCodeArgs) {
        /* 0x3F800000 -> FRn */
        todo!()
    }

    pub fn FLDS(&mut self, args: OpCodeArgs) {
        /* FRm -> FPUL */
        todo!()
    }

    pub fn FSTS(&mut self, args: OpCodeArgs) {
        /* FPUL -> FRn */
        todo!()
    }

    pub fn FABS(&mut self, args: OpCodeArgs) {
        /* FRn & 0x7FFFFFFF -> FRn */
        todo!()
    }

    pub fn FNEG(&mut self, args: OpCodeArgs) {
        /* FRn ^ 0x80000000 -> FRn */
        todo!()
    }

    pub fn FADD(&mut self, args: OpCodeArgs) {
        /* FRn + FRm -> FRn */
        todo!()
    }

    pub fn FSUB(&mut self, args: OpCodeArgs) {
        /* FRn - FRm -> FRn */
        todo!()
    }

    pub fn FMUL(&mut self, args: OpCodeArgs) {
        /* FRn * FRm -> FRn */
        todo!()
    }

    pub fn FMAC(&mut self, args: OpCodeArgs) {
        /* FR0 * FRm + FRn -> FRn */
        todo!()
    }

    pub fn FDIV(&mut self, args: OpCodeArgs) {
        /* FRn / FRm -> FRn */
        todo!()
    }

    pub fn FSQRT(&mut self, args: OpCodeArgs) {
        /* sqrt (FRn) -> FRn */
        todo!()
    }

    pub fn FCMP_EQ(&mut self, args: OpCodeArgs) {
        /* If FRn = FRm: 1 -> T Else: 0 -> T */
        todo!()
    }

    pub fn FCMP_GT(&mut self, args: OpCodeArgs) {
        /* If FRn > FRm: 1 -> T Else: 0 -> T */
        todo!()
    }

    pub fn FLOAT_single(&mut self, args: OpCodeArgs) {
        /* (float)FPUL -> FRn */
        todo!()
    }

    pub fn FTRC_single(&mut self, args: OpCodeArgs) {
        /* (long)FRm -> FPUL */
        todo!()
    }

    pub fn FIPR(&mut self, args: OpCodeArgs) {
        /* inner_product (FVm, FVn) -> FR[n+3] */
        todo!()
    }

    pub fn FTRV(&mut self, args: OpCodeArgs) {
        /* transform_vector (XMTRX, FVn) -> FVn */
        todo!()
    }

    pub fn FSRRA(&mut self, args: OpCodeArgs) {
        /* 1.0 / sqrt (FRn) -> FRn */
        todo!()
    }

    pub fn FSCA(&mut self, args: OpCodeArgs) {
        /*
         sin (FPUL) -> FRn cos (FPUL) -> FR[n+1]
        */
        todo!()
    }

    pub fn FABS_DUP(&mut self, args: OpCodeArgs) {
        /* DRn & 0x7FFFFFFFFFFFFFFF -> DRn */
        todo!()
    }

    pub fn FNEG_DUP(&mut self, args: OpCodeArgs) {
        /* DRn ^ 0x8000000000000000 -> DRn */
        todo!()
    }

    pub fn FADD_DUP(&mut self, args: OpCodeArgs) {
        /* DRn + DRm -> DRn */
        todo!()
    }

    pub fn FSUB_DUP(&mut self, args: OpCodeArgs) {
        /* DRn - DRm -> DRn */
        todo!()
    }

    pub fn FMUL_DUP(&mut self, args: OpCodeArgs) {
        /* DRn * DRm -> DRn */
        todo!()
    }

    pub fn FDIV_DUP(&mut self, args: OpCodeArgs) {
        /* DRn / DRm -> DRn */
        todo!()
    }

    pub fn FSQRT_DUP(&mut self, args: OpCodeArgs) {
        /* sqrt (DRn) -> DRn */
        todo!()
    }

    pub fn FCMP_EQ_DUP(&mut self, args: OpCodeArgs) {
        /* If DRn = DRm: 1 -> T Else: 0 -> T */
        todo!()
    }

    pub fn FCMP_GT_DUP(&mut self, args: OpCodeArgs) {
        /* If DRn > DRm: 1 -> T Else: 0 -> T */
        todo!()
    }

    pub fn FLOAT_double(&mut self, args: OpCodeArgs) {
        /* (double)FPUL -> DRn */
        todo!()
    }

    pub fn FTRC_double(&mut self, args: OpCodeArgs) {
        /* (long)DRm -> FPUL */
        todo!()
    }

    pub fn FCNVDS(&mut self, args: OpCodeArgs) {
        /* double_to_float (DRm) -> FPUL */
        todo!()
    }

    pub fn FCNVSD(&mut self, args: OpCodeArgs) {
        /* float_to_double (FPUL) -> DRn */
        todo!()
    }

    pub fn LDSFPSCR(&mut self, args: OpCodeArgs) {
        /* Rm -> FPSCR */
        todo!()
    }

    pub fn STSFPSCR(&mut self, args: OpCodeArgs) {
        /* FPSCR -> Rn */
        todo!()
    }

    pub fn LDSMFPSCR(&mut self, args: OpCodeArgs) {
        /* (Rm) -> FPSCR, Rm+4 -> Rm */
        todo!()
    }

    pub fn STSMFPSCR(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, FPSCR -> (Rn) */
        todo!()
    }

    pub fn LDSFPUL(&mut self, args: OpCodeArgs) {
        /* Rm -> FPUL */
        todo!()
    }

    pub fn STSFPUL(&mut self, args: OpCodeArgs) {
        /* FPUL -> Rn */
        todo!()
    }

    pub fn LDSMFPUL(&mut self, args: OpCodeArgs) {
        /* (Rm) -> FPUL, Rm+4 -> Rm */
        todo!()
    }

    pub fn STSMFPUL(&mut self, args: OpCodeArgs) {
        /* Rn-4 -> Rn, FPUL -> (Rn) */
        todo!()
    }

    pub fn FRCHG(&mut self, args: OpCodeArgs) {
        /*
         If FPSCR.PR = 0: ~FPSCR.FR -> FPSCR.FR Else: Undefined Operation
        */
        todo!()
    }

    pub fn FSCHG(&mut self, args: OpCodeArgs) {
        /*
         If FPSCR.PR = 0: ~FPSCR.SZ -> FPSCR.SZ Else: Undefined Operation
        */
        todo!()
    }

    pub fn FPCHG(&mut self, args: OpCodeArgs) {
        /* ~FPSCR.PR -> FPSCR.PR */
        todo!()
    }
}
