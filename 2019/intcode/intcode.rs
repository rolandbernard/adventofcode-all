
pub type IntValue = i64;
pub type IntMem = Vec<IntValue>;

pub fn parse(text: &str) -> IntMem {
    return text.split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect::<IntMem>();
}

#[derive(Clone)]
pub struct IntState {
    pub pc: usize,
    pub bp: usize,
    pub halt: bool,
}

impl IntState {
    pub fn new() -> IntState {
        return IntState { pc: 0, bp: 0, halt: false };
    }
}

fn read_param(mem: &IntMem, state: &IntState, mode: IntValue, val: IntValue) -> IntValue {
    let off = val as usize;
    let rel = state.bp.wrapping_add(off);
    return match mode {
        0 => if off < mem.len() { mem[off] } else { 0 },
        1 => val,
        2 => if rel < mem.len() { mem[rel] } else { 0 },
        _ => 0,
    }
}

fn write_param(mem: &mut IntMem, state: &IntState, mode: IntValue, addr: IntValue, val: IntValue) {
    if mode == 0 || mode == 2 {
        let off;
        if mode == 0 {
            off = addr as usize;
        } else {
            off = state.bp.wrapping_add(addr as usize);
        }
        if off >= mem.len() {
            mem.resize(off + 1, 0);
        }
        mem[off] = val;
    }
}

pub fn run_instr<FI, FO>(mem: &mut IntMem, state: &mut IntState, mut inp: FI, mut out: FO)
    where FI: FnMut() -> IntValue, FO: FnMut(IntValue)
{
    if !state.halt {
        let instr = mem[state.pc];
        let op = instr % 100;
        let pm0 = (instr / 100) % 10;
        let pm1 = (instr / 1000) % 10;
        let pm2 = (instr / 10000) % 10;
        match op {
            1 => {
                let v0 = read_param(mem, state, pm0, mem[state.pc + 1]);
                let v1 = read_param(mem, state, pm1, mem[state.pc + 2]);
                write_param(mem, state, pm2, mem[state.pc + 3], v0 + v1);
                state.pc += 4;
            },
            2 => {
                let v0 = read_param(mem, state, pm0, mem[state.pc + 1]);
                let v1 = read_param(mem, state, pm1, mem[state.pc + 2]);
                write_param(mem, state, pm2, mem[state.pc + 3], v0 * v1);
                state.pc += 4;
            },
            3 => {
                write_param(mem, state, pm0, mem[state.pc + 1], inp());
                state.pc += 2;
            },
            4 => {
                out(read_param(mem, state, pm0, mem[state.pc + 1]));
                state.pc += 2;
            },
            5 => {
                if read_param(mem, state, pm0, mem[state.pc + 1]) != 0 {
                    state.pc = read_param(mem, state, pm1, mem[state.pc + 2]) as usize;
                } else {
                    state.pc += 3;
                }
            }
            6 => {
                if read_param(mem, state, pm0, mem[state.pc + 1]) == 0 {
                    state.pc = read_param(mem, state, pm1, mem[state.pc + 2]) as usize;
                } else {
                    state.pc += 3;
                }
            }
            7 => {
                let v0 = read_param(mem, state, pm0, mem[state.pc + 1]);
                let v1 = read_param(mem, state, pm1, mem[state.pc + 2]);
                write_param(mem, state, pm2, mem[state.pc + 3], if v0 < v1 { 1 } else { 0 });
                state.pc += 4;
            },
            8 => {
                let v0 = read_param(mem, state, pm0, mem[state.pc + 1]);
                let v1 = read_param(mem, state, pm1, mem[state.pc + 2]);
                write_param(mem, state, pm2, mem[state.pc + 3], if v0 == v1 { 1 } else { 0 });
                state.pc += 4;
            },
            9 => {
                state.bp = state.bp.wrapping_add(read_param(mem, state, pm0, mem[state.pc + 1]) as usize);
                state.pc += 2;
            },
            _ => {
                state.halt = true;
                state.pc += 1;
            },
        }
    }
}
