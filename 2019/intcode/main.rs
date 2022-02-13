
use std::fs;
use std::io;
use std::env;
use std::io::Read;
use std::io::Write;

mod intcode;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let args = env::args().collect::<Vec<String>>();
    let code = fs::read_to_string(&args[1]).unwrap();
    let mut mem = intcode::parse(&code);
    let mut cpu = intcode::IntState::new();
    let inp = || {
        return stdin.lock().bytes().next()
            .and_then(|result| result.ok())
            .map(|byte| byte as intcode::IntValue)
            .unwrap_or(0);
    };
    let out = |x| {
        stdout.lock().write_all(&[x as u8]).unwrap();
    };
    while !cpu.halt {
        intcode::run_instr(&mut mem, &mut cpu, inp, out);
    }
}

