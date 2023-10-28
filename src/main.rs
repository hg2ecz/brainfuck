use std::fs::File;
use std::io::{self, Read};

enum Instr {
    IncPtr, // '>'
    DecPtr, // '<'
    Inc,    // '+'
    Dec,    // '-'
    Output, // '.' u8 to ascii
    Input,  // ',' ascii to u8
    //  LoopBegin '[' --> push(POSITION)
    Jnz(usize), // ']' Jump if not zero
}

fn compiler(src: &str) -> Vec<Instr> {
    let mut prog = vec![];
    let mut loop_start = vec![];
    for ch in src.chars() {
        match ch {
            '>' => prog.push(Instr::IncPtr),
            '<' => prog.push(Instr::DecPtr),
            '+' => prog.push(Instr::Inc),
            '-' => prog.push(Instr::Dec),
            '.' => prog.push(Instr::Output),
            ',' => prog.push(Instr::Input),
            '[' => loop_start.push(prog.len()),
            ']' => prog.push(Instr::Jnz(loop_start.pop().unwrap())),
            _ => (),
        };
    }
    prog
}

fn vcpu_runner(prog: &[Instr]) {
    let mut data: [u8; 1024] = [0; 1024];
    let mut dataptr = 0;
    let mut pc = 0;

    let mut inp: [u8; 1] = [0; 1];
    while pc < prog.len() {
        match prog[pc] {
            Instr::IncPtr => dataptr += 1,
            Instr::DecPtr => dataptr -= 1,
            Instr::Inc => data[dataptr] += 1,
            Instr::Dec => data[dataptr] -= 1,
            Instr::Output => print!("{}", data[dataptr] as char),
            Instr::Input => {
                io::stdin().read_exact(&mut inp).expect("failed to read");
                data[dataptr] = inp[0];
            }
            Instr::Jnz(addr) => {
                if data[dataptr] != 0 {
                    pc = addr;
                    continue;
                }
            }
        }
        pc += 1;
    }
}

fn main() {
    if let Some(fname) = std::env::args().nth(1) {
        let mut file = File::open(fname).expect("program file not found");
        let mut src = String::new();
        file.read_to_string(&mut src).expect("failed to read");
        vcpu_runner(&compiler(&src));
    } else {
        println!("usage: brainfuck <file.bf>");
    }
}
