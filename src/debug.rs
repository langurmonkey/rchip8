use std::io::{stdin, stdout, Read, Write};

pub fn debug(pc: usize, instr: u16, code: u16, x: usize, y: usize, n: u16, nn: u16, nnn: u16) {
    println!("pc:          {} (0x{:04x})", pc - 2, pc - 2);
    println!("{}", debug_instr(code, x, y, n, nn, nnn));
    println!("instr:       0x{:04x}", instr);
    println!("code:        0x{:04x}", code);
    println!("x:           0x{:04x} ({})", x, x);
    println!("y:           0x{:04x} ({})", y, y);
    println!("n:           0x{:04x} ({})", n, y);
    println!("nn:          0x{:04x} ({})", nn, nn);
    println!("nnn:         0x{:04x} ({})", nnn, nnn);
    pause();
}

pub fn debug_instr(code: u16, x: usize, y: usize, n: u16, nn: u16, nnn: u16) -> String {
    match code {
        // 00E0 - clear screen
        0x0000 => format!("CLEAR"),
        // 1NNN - jump
        0x1000 => format!("JUMP TO {}", nnn),
        // 6XNN - set register VX to NN
        0x6000 => format!("V{} := {}", x, nnn),
        // 7XNN - add NN to register VX
        0x7000 => format!("V{} := V{} + {}", x, x, nn),
        0x800 => {
            match n {
                // 8XY0 - VX := VY
                0 => format!("V{} := V{}", x, y),
                // 8XY1 - VX := VX OR VY
                1 => format!("V{} := V{} OR V{}", x, x, y),
                // 8XY2 - VX := VX AND VY
                2 => format!("V{} := V{} AND V{}", x, x, y),
                // 8XY3 - VX := VX XOR VY
                3 => format!("V{} := V{} XOR V{}", x, x, y),
                // 8XY4 - VX := VX + VY
                4 => format!("V{} := V{} + V{}", x, x, y),
                // Default
                _ => format!(""),
            }
        }
        // ANNN - set index register to NNN
        0xA000 => format!("I := {}", nnn),
        // DXYN - display/draw
        0xD000 => format!("DRAW [V{},V{}] <- {}", x, y, n),
        _ => format!(""),
    }
    .to_string()
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
