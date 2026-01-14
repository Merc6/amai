pub mod value;
pub mod inst;
pub mod function;
pub mod call_frame;

use std::rc::Rc;
use value::*;
use inst::*;
use function::Function;
use call_frame::CallFrame;

#[derive(Clone)]
pub struct AmaiVM {
    pub frames: Vec<CallFrame>,
    pub constants: Box<[Value]>,
    pub running: bool,
    pub functions: Vec<Rc<Function>>,
    pub allow_large_bytecode: bool,
}

impl AmaiVM {
    pub fn new(constants: Box<[Value]>, allow_large_bytecode: bool) -> Self {
        Self {
            frames: Vec::new(),
            constants,
            running: false,
            functions: Vec::new(),
            allow_large_bytecode,
        }
    }

    
    #[inline(always)]
    pub fn add_function(&mut self, bytecode: Box<[u32]>, constant_count: usize) -> usize {
        if !self.allow_large_bytecode {
            assert!(bytecode.len() < 65536, "Bytecode length is out of jump bounds");
        }

        let func = Function { constant_count, bytecode };
        self.functions.push(Rc::new(func));
        self.functions.len() - 1
    }

    #[inline(always)]
    pub fn call_function(&mut self, id: usize) {
        let func = self.functions[id].clone();
        let new_frame = CallFrame {
            function: func,
            registers: [Value::nil(); 256],
            constant_idx_base: self.frames
                .last()
                .map(|f|
                    f.constant_idx_base + f.function.constant_count
                ).unwrap_or(0),
            ip: 0,
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
        let frame = self.frames.last_mut().unwrap() as *mut CallFrame;
        let inst = if let Some(inst) = (&(*frame).function).bytecode.get((*frame).ip) {
            inst
        } else {
            self.running = false;
            return Ok(());
        };
        (*frame).ip += 1;

        let opcode = (inst & 0xFF) as u8;

        match opcode {
            NOP => {},
            LOAD => {
                let dest = ((inst >> 8) & 0xFF) as u8;
                let id = ((inst >> 16) & 0xFFFF) as u16;
                let abs_idx = (*frame).constant_idx_base + id as usize;
                let constant = *self.constants.get_unchecked(abs_idx);

                (*frame).registers[dest as usize] = constant;
            },
            IADD => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.iadd(src2);
            },
            ISUB => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.isub(src2);
            },
            IMUL => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.imul(src2);
            },
            IDIV => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.idiv(src2).ok_or("Division by zero")?;
            },
            IREM => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.irem(src2).ok_or("Division by zero")?;
            },
            FADD => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.fadd(src2);
            },
            FSUB => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.fsub(src2);
            },
            FMUL => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.fmul(src2);
            },
            FDIV => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.fdiv(src2).ok_or("Division by zero")?;
            },
            FREM => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.frem(src2).ok_or("Division by zero")?;
            },
            BOR => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.bor(src2);
            },
            BAND => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.band(src2);
            },
            BXOR => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.bxor(src2);
            },
            BNOT => {
                let src = (*frame).registers[((inst >> 16) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src.bnot();
            },
            LOR => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.lor(src2);
            },
            LAND => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.land(src2);
            },
            LNOT => {
                let src = (*frame).registers[((inst >> 16) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src.lnot();
            },
            CMEQ => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.cmeq(src2);
            },
            CMNE => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.cmne(src2);
            },
            ICGT => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.icgt(src2);
            },
            ICLT => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.iclt(src2);
            },
            ICGE => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.icge(src2);
            },
            ICLE => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.icle(src2);
            },
            FCGT => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.fcgt(src2);
            },
            FCLT => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.fclt(src2);
            },
            FCGE => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.fcge(src2);
            },
            FCLE => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];
                let src2 = (*frame).registers[((inst >> 24) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.fcle(src2);
            },
            JUMP => {
                let addr = ((inst >> 8) & 0xFFFF) as i16;

                if addr >= 0 {
                    (*frame).ip += addr as usize;
                } else {
                    (*frame).ip -= addr as usize;
                }
            },
            JITR => {
                let addr = ((inst >> 8) & 0xFFFF) as i16;
                let src = (*frame).registers[((inst >> 24) & 0xFF) as usize].to_bool();

                if src {
                    if addr >= 0 {
                        (*frame).ip += addr as usize;
                    } else {
                        (*frame).ip -= addr as usize;
                    }
                }
            },
            JIFL => {
                let addr = ((inst >> 8) & 0xFFFF) as i16;
                let src = (*frame).registers[((inst >> 24) & 0xFF) as usize].to_bool();

                if !src {
                    if addr >= 0 {
                        (*frame).ip += addr as usize;
                    } else {
                        (*frame).ip -= addr as usize;
                    }
                }
            },
            CALL => {
                let id = (inst >> 8) & 0xFFFFFF;
                self.call_function(id as usize);
            },
            RETN => self.return_function(),
            INEG => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.ineg();
            },
            FNEG => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1.fneg();
            },
            MOVE => {
                let src1 = (*frame).registers[((inst >> 16) & 0xFF) as usize];

                (*frame).registers[((inst >> 8) & 0xFF) as usize] = src1;
            },
            HALT => self.running = false,
            _ => panic!("Unknown opcode: {opcode:#04X}"),
        }

        Ok(())
    }
}