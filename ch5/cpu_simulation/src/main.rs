

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        pc: 0,
        memory: [0; 0x1000],
        sp: 0,
        stack: [0; 16],
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.memory[0] = 0x21; cpu.memory[1] = 0x00;
    cpu.memory[2] = 0x21; cpu.memory[3] = 0x00;
    cpu.memory[4] = 0x00; cpu.memory[5] = 0x00;

    cpu.memory[0x100] = 0x80; cpu.memory[0x101] = 0x14;
    cpu.memory[0x102] = 0x80; cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00; cpu.memory[0x105] = 0xee;

    cpu.run();
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}


struct CPU {
    registers: [u8; 16],
    pc: usize,
    memory: [u8; 0x1000],
    sp: usize,
    stack: [u16; 16],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.pc;
        if p >= 0x1000 {
            return 0x0000;
        }

        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        (op_byte1 << 8) | op_byte2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (sum, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = sum;
        self.registers[0xf] = if overflow { 1 } else { 0 };
    }

    fn call(&mut self, addr: u16) {
        let sp = self.sp;
        if sp >= self.stack.len() {
            panic!("stack overflow");
        }

        self.stack[sp] = self.pc as u16;
        self.sp += 1;
        self.pc = addr as usize;
    }

    fn ret(&mut self) {
        if self.sp == 0 {
            panic!("stack underflow");
        }

        self.sp -= 1;
        self.pc = self.stack[self.sp] as usize;
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2;

            let c = ((opcode & 0xf000) >> 12) as u8;
            let x = ((opcode & 0x0f00) >> 8) as u8;
            let y = ((opcode & 0x00f0) >> 4) as u8;
            let d = ((opcode & 0x000f) >> 0) as u8;
            let nnn = opcode & 0x0fff;

            match (c, x, y, d) {
                (0, 0, 0, 0) => break,
                (0, 0, 0xe, 0xe) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode = {:04x}", opcode),
            }
        }
    }
}
