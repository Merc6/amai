#[derive(Debug, Clone)] pub struct OperandSlot(pub u8);

#[derive(Debug, Clone)]
pub enum IRInst {
    Nop,
    Load { dest: OperandSlot, const_id: u16 },
    Move { dest: OperandSlot, reg: u8 },
    IAdd { dest: u8, src1: u8, src2: u8 },
    ISub { dest: u8, src1: u8, src2: u8 },
    IMul { dest: u8, src1: u8, src2: u8 },
    IDiv { dest: u8, src1: u8, src2: u8 },
    IRem { dest: u8, src1: u8, src2: u8 },
    INeg { dest: u8, src: u8 },
    FAdd { dest: u8, src1: u8, src2: u8 },
    FSub { dest: u8, src1: u8, src2: u8 },
    FMul { dest: u8, src1: u8, src2: u8 },
    FDiv { dest: u8, src1: u8, src2: u8 },
    FRem { dest: u8, src1: u8, src2: u8 },
    FNeg { dest: u8, src: u8 },
    BOr { dest: u8, src1: u8, src2: u8 },
}