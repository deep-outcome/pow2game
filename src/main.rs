//! Simple "guessing" game for everyday-use powers of 2 intended for programmers.
use noiserand::NoiseRand;
use rand_core::RngCore;
use std::io::{stdin, stdout, Write};

fn main() -> std::io::Result<()> {
    let arg = std::env::args().nth(1);

    let mut stdout = stdout();

    let mut colorize = true;
    if let Some(arg) = arg {
        if arg == "--help" {
            writeln!(&mut stdout, "\n--- HELP ---\n")?;
            writeln!(&mut stdout, "    --help    |this help")?;
            writeln!(&mut stdout, "    --nocolor |no output colorization\n")?;
            stdout.flush()?;

            return Ok(());
        }

        if arg == "--nocolor" {
            colorize = false;
        }
    }

    writeln!(
        &mut stdout,
        "\n----> Welcome to the Power of 2 Game <----\n"
    )?;
    writeln!(
        &mut stdout,
        "Acquiring quantum fluctuations based seed. Check https://qrng.anu.edu.au/ for more.\n\n"
    )?;

    let mut pows = (0..=16).collect::<Vec<u32>>();

    assert_eq!(17, pows.len());

    let mut nr = NoiseRand::new();
    let rn = nr.next_u32();
    let b0 = rn.to_ne_bytes()[0];

    let mut ix1 = 0;
    let mut ix2 = (b0 / 17) as usize;

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

    writeln!(&mut stdout, "Serie: {:?}", pows);

    let mut buff = String::new();
    let mut p_ix = 0;
    'ml: while p_ix < 17 {
        let p = pows[p_ix];

        writeln!(&mut stdout, "We have power: {p}.")?;
        writeln!(&mut stdout, "Tell the result‽")?;
        stdout.flush()?;

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

            writeln!(
                &mut stdout,
                "{}\n",
                colorized(colorize, format!("Error ╏ '{}'", buff), "\x1b[0;35m")
            )?;
            continue 'ml;
        }

        let num = 2u32.pow(p);
        let (print, color) = if num == answer {
            p_ix += 1;
            (format!("Of course, {}.", answer), "\x1b[0;32m")
        } else {
            (format!("Nope, {}.", num), "\x1b[0;31m")
        };

        writeln!(&mut stdout, "{}\n", colorized(colorize, print, color))?;
    }

    stdout.flush()
}

fn colorized(colorize: bool, mut txt: String, color: &str) -> String {
    if colorize {
        txt.insert_str(0, color);
        txt.push_str("\x1b[0;0m");
    }

    txt
}
