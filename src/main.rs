//! Simple "guessing" game for everyday-use powers of 2 intended for programmers.
use noiserand::NoiseRand;
use rand_core::RngCore;
use std::io::stdin;

fn main() {
    let arg = std::env::args().nth(1);

    let mut colorize = true;
    if let Some(arg) = arg {
        if arg == "--help" {
            println!("\n--- HELP ---\n");
            println!("    --help    |this help");
            println!("    --nocolor |no output colorization\n");

            return;
        }

        if arg == "--nocolor" {
            colorize = false;
        }
    }

    println!("\n----> Welcome to the Power of 2 Game <----\n");
    println!("Acquiring quantum fluctuations based seed. Check https://qrng.anu.edu.au/ for more.");
    println!("\n");

    let mut pows = (0..=16).collect::<Vec<u32>>();

    assert_eq!(17, pows.len());

    let mut nr = NoiseRand::new();
    let rn = nr.next_u32();
    let b0 = rn.to_ne_bytes()[0];

    let seed_ix = (b0 / 17) as usize;

    let mut ix1 = 0;
    let mut ix2 = seed_ix;

    while ix2 < 17 {
        let swap = pows[ix2];
        pows[ix2] = pows[ix1];
        pows[ix1] = swap;

        ix1 += 1;
        ix2 += 1;
    }

    for i in 0..=15 {
        ix1 = i;
        ix2 = 16;
        while ix1 < ix2 {
            let swap = pows[ix2];
            pows[ix2] = pows[ix1];
            pows[ix1] = swap;

            ix1 += 1;
            ix2 -= 1;
        }
    }

    println!("Serie: {:?}", pows);

    let mut buff = String::new();
    let mut p_ix = 0;
    while p_ix < 17 {
        let p = pows[p_ix];

        println!("We have power: {p}.");
        println!("Tell the result‽");

        let answer: u32;
        loop {
            buff.clear();

            _ = stdin().read_line(&mut buff);
            buff = buff.replace("\n", "");
            buff = buff.replace("\r", "");

            let parse = buff.parse::<u32>();

            if parse.is_ok() {
                answer = parse.ok().unwrap();
                break;
            }

            println!("Error {:?}", buff);
        }

        let num = 2u32.pow(p);
        let (print, color) = if num == answer {
            p_ix += 1;
            (format!("Offcourse, {}.", answer), "\x1b[0;32m")
        } else {
            (format!("Nope, {}.", num), "\x1b[0;31m")
        };

        println!("{}\n", colorized(colorize, print, color));
    }
}

fn colorized(colorize: bool, mut txt: String, color: &str) -> String {
    if colorize {
        txt.insert_str(0, color);
        txt.push_str("\x1b[0;0m");
    }

    txt
}
