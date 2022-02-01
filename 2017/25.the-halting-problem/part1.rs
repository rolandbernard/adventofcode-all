
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut state = lines[0].as_bytes()[15] - 'A' as u8;
    let steps = lines[1].split(" ").collect::<Vec<&str>>()[5].parse::<i64>().unwrap();
    let mut states = Vec::new();
    for i in (2..lines.len()).step_by(10) {
        states.push((
            (
                lines[i + 3].as_bytes()[22] == '1' as u8,
                lines[i + 4].as_bytes()[27] == 'r' as u8,
                lines[i + 5].as_bytes()[26] - 'A' as u8,
            ), (
                lines[i + 7].as_bytes()[22] == '1' as u8,
                lines[i + 8].as_bytes()[27] == 'r' as u8,
                lines[i + 9].as_bytes()[26] - 'A' as u8,
            )
        ));
    }
    let mut tally = 0;
    let mut tape = (Vec::new(), Vec::new());
    let mut curs: isize = 0;
    for _ in 0..steps {
        let loc = if curs < 0 {
            if tape.0.len() <= (-curs - 1) as usize {
                tape.0.push(false);
            }
            &mut tape.0[(-curs - 1) as usize]
        } else {
            if tape.1.len() <= curs as usize {
                tape.1.push(false);
            }
            &mut tape.1[curs as usize]
        };
        let trans = states[state as usize];
        let act = if *loc { trans.1 } else { trans.0 };
        tally += act.0 as i64 - *loc as i64;
        *loc = act.0;
        curs += if act.1 { 1 } else { -1 };
        state = act.2;
    }
    println!("Result: {}", tally);
}
