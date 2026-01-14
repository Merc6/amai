pub struct FunctionBuilder {
    pub constant_count: usize,
    pub bytecode: Vec<u32>,
}

// functions with weird names are documented with their purposes
impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            constant_count: 0,
            bytecode: Vec::new(),
        }
    }

    /// Creates an instruction with the format: `[opcode - 8][dest - 8][src1 - 8][src2 - 8]`
    pub fn orrr(&mut self, opcode: u8, dest: u8, src1: u8, src2: u8) {
        let packed_inst = (opcode as u32)
            | ((dest as u32) << 8)
            | ((src1 as u32) << 16)
            | ((src2 as u32) << 24);

        self.bytecode.push(packed_inst);
    }
    
    /// Creates an instruction with the format: `[opcode - 8][dest - 8][src - 8]`
    pub fn orr(&mut self, opcode: u8, dest: u8, src: u8) {
        let packed_inst = (opcode as u32)
            | ((dest as u32) << 8)
            | ((src as u32) << 16);

        self.bytecode.push(packed_inst);
    }

    /// Creates an instruction with the format: `[opcode - 8][id - 16]`
    pub fn od16(&mut self, opcode: u8, imm: u16) {
        let packed_inst = (opcode as u32)
            | ((imm as u32) << 8);

        self.bytecode.push(packed_inst);
    }

    /// Creates an instruction with the format: `[opcode - 8][id - 16][src - 8]`
    pub fn od16r(&mut self, opcode: u8, imm: u16, src: u8) {
        let packed_inst = (opcode as u32)
            | ((imm as u32) << 8)
            | ((src as u32) << 24);

        self.bytecode.push(packed_inst);
    }
    
    /// Creates an instruction with the format: `[opcode - 8][src - 8][id - 16]`
    pub fn ord16(&mut self, opcode: u8, src: u8, imm: u16) {
        let packed_inst = (opcode as u32)
            | ((src as u32) << 8)
            | ((imm as u32) << 16);

        self.bytecode.push(packed_inst);
    }

    
    /// Creates an instruction with the format: `[opcode - 8][id - 24]`
    pub fn od24(&mut self, opcode: u8, imm: u32) {
        assert!(imm < 16777216);
        let packed_inst = (opcode as u32)
            | (imm << 8);

        self.bytecode.push(packed_inst);
    }

    
    /// Creates an instruction with the format: `[opcode - 8]`
    pub fn o(&mut self, opcode: u8) {
        self.bytecode.push(opcode as u32);
    }
}