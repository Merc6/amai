pub mod value;
pub mod inst;

use std::rc::Rc;
use value::*;
use inst::*;

#[derive(Clone)]
pub struct AmaiVM<'vm> {
    pub frames: Vec<CallFrame<'vm>>,
    pub constants: &'vm [Value],
    pub running: bool,
    pub functions: Vec<Rc<Function<'vm>>>,
    pub allow_large_bytecode: bool,
}

impl<'vm> AmaiVM<'vm> {
    pub fn new(constants: &'vm [Value], allow_large_bytecode: bool) -> Self {
        Self {
            frames: Vec::new(),
            constants,
            running: false,
            functions: Vec::new(),
            allow_large_bytecode,
        }
    }

    
    #[inline(always)]
    pub fn add_function(&mut self, bytecode: &'vm [u32], constant_count: usize) {
        if !self.allow_large_bytecode {
            assert!(bytecode.len() < 65536, "Bytecode length is out of jump bounds");
        }

        let func = Function { constant_count, bytecode };
        self.functions.push(Rc::new(func));
    }

    #[inline(always)]
    pub fn call_function(&mut self, id: usize) {
        let func = self.functions[id].clone();
        let ip = func.bytecode.as_ptr();
        let new_frame = CallFrame {
            function: func,
            registers: [Value::nil(); 256],
            constant_idx_base: self.frames
                .last()
                .map(|f|
                    f.constant_idx_base + f.function.constant_count
                ).unwrap_or(0),
            bytecode_base: ip,
            ip,
        };
        self.frames.push(new_frame);
    }

    #[inline(always)]
    pub fn return_function(&mut self) {
        self.frames.pop();
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        self.running = true;
        while self.running {
            unsafe { self.cycle()? }
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(unsafe_op_in_unsafe_fn)]
    pub unsafe fn cycle(&mut self) -> Result<(), &'static str> {
        let frame = self.frames.as_mut_ptr().add(self.frames.len() - 1);
        let inst = *(*frame).ip;
        let next_ip = (*frame).ip.add(1);

        let opcode = (inst & 0xFF) as u8;
        let dest = ((inst >> 8) & 0xFF) as u8;
        let src1 = ((inst >> 16) & 0xFF) as u8;
        let src2 = ((inst >> 24) & 0xFF) as u8;

        match opcode {
            NOP => {},
            LOAD => {
                let id = src1 as u16 | ((src2 as u16) << 8);
                let abs_idx = (*frame).constant_idx_base + id as usize;
                let constant = *self.constants.get_unchecked(abs_idx);

                (*frame).registers[dest as usize] = constant;
            },
            IADD => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.iadd(src2);
            },
            ISUB => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.isub(src2);
            },
            IMUL => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.imul(src2);
            },
            IDIV => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.idiv(src2).ok_or("Division by zero")?;
            },
            IREM => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.irem(src2).ok_or("Division by zero")?;
            },
            FADD => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.fadd(src2);
            },
            FSUB => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.fsub(src2);
            },
            FMUL => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.fmul(src2);
            },
            FDIV => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.fdiv(src2).ok_or("Division by zero")?;
            },
            FREM => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.frem(src2).ok_or("Division by zero")?;
            },
            BOR => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.bor(src2);
            },
            BAND => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.band(src2);
            },
            BXOR => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.bxor(src2);
            },
            BNOT => {
                let dest = dest ;
                let src = (*frame).registers[src1 as usize];

                (*frame).registers[dest as usize] = src.bnot();
            },
            LOR => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.lor(src2);
            },
            LAND => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.land(src2);
            },
            LNOT => {
                let dest = dest;
                let src = (*frame).registers[src1 as usize];

                (*frame).registers[dest as usize] = src.lnot();
            },
            CMEQ => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.cmeq(src2);
            },
            CMNE => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.cmne(src2);
            },
            ICGT => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.icgt(src2);
            },
            ICLT => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.iclt(src2);
            },
            ICGE => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.icge(src2);
            },
            ICLE => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.icle(src2);
            },
            FCGT => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.fcgt(src2);
            },
            FCLT => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.fclt(src2);
            },
            FCGE => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.fcge(src2);
            },
            FCLE => {
                let dest = dest;
                let src1 = (*frame).registers[src1 as usize];
                let src2 = (*frame).registers[src2 as usize];

                (*frame).registers[dest as usize] = src1.fcle(src2);
            },
            JUMP => {
                let addr = (dest as u16
                    | ((src1 as u16) << 8))
                    as i16;

                if addr >= 0 {
                    (*frame).ip = (*frame).bytecode_base.add(addr as usize);
                } else {
                    (*frame).ip = (*frame).bytecode_base.sub(addr.abs() as usize);
                }
            },
            JITR => {
                let addr = (dest as u16
                    | ((src1 as u16) << 8))
                    as i16;
                let src = (*frame).registers[src2 as usize].to_bool();

                if src {
                    if addr >= 0 {
                    (*frame).ip = (*frame).bytecode_base.add(addr as usize);
                    } else {
                        (*frame).ip = (*frame).bytecode_base.sub(addr.abs() as usize);
                    }
                }
            },
            JIFL => {
                let addr = (dest as u16
                    | ((src1 as u16) << 8))
                    as i16;
                let src = (*frame).registers[src2 as usize].to_bool();

                if !src {
                    if addr >= 0 {
                    (*frame).ip = (*frame).bytecode_base.add(addr as usize);
                    } else {
                        (*frame).ip = (*frame).bytecode_base.sub(addr.abs() as usize);
                    }
                }
            },
            CALL => {
                let id = (*frame).registers[dest as usize].to_int() as usize;
                self.call_function(id);
            },
            RETN => self.return_function(),
            HALT => self.running = false,
            _ => panic!("Unknown opcode: {opcode:#04X}"),
        }

        (*frame).ip = next_ip;
        Ok(())
    }
}

#[derive(Clone)]
pub struct CallFrame<'cf> {
    pub function: Rc<Function<'cf>>,
    pub registers: [Value; 256],
    pub constant_idx_base: usize,
    pub bytecode_base: *const u32,
    pub ip: *const u32,
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct Function<'func> {
    pub constant_count: usize,
    pub bytecode: &'func [u32],
}