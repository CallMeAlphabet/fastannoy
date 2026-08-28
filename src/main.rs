//! Copyright 2026 CallMeAlphabet (ItzAlphabet)
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!    http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use libc::{ioctl, signal, SIG_IGN, SIGHUP, SIGINT, SIGPIPE, SIGQUIT, SIGTERM, SIGTSTP, STDOUT_FILENO, TIOCGWINSZ};
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const BRIGHT_GREEN: &str = "\x1b[92m";
const WHITE: &str = "\x1b[97m";
const BOLD: &str = "\x1b[1m";

#[repr(C)]
struct WinSize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

fn get_term_size() -> (u16, u16) {
    let mut ws = WinSize { ws_row: 24, ws_col: 80, ws_xpixel: 0, ws_ypixel: 0 };
    unsafe {
        ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws);
    }
    (ws.ws_col, ws.ws_row)
}

fn ignore_signals() {
    unsafe {
        signal(SIGTERM, SIG_IGN);
        signal(SIGINT, SIG_IGN);
        signal(SIGQUIT, SIG_IGN);
        signal(SIGHUP, SIG_IGN);
        signal(SIGPIPE, SIG_IGN);
        signal(SIGTSTP, SIG_IGN);
    }
}

fn xorshift(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

fn main() {
    ignore_signals();

    let start = Instant::now();
    let duration = Duration::from_secs(20);
    let mut seed = 987654321;

    while start.elapsed() < duration {
        let elapsed_secs = start.elapsed().as_secs();
        let remaining = 20 - elapsed_secs;
        
        let (term_w, term_h) = get_term_size();
        let term_w = term_w as usize;
        let term_h = term_h as usize;
        
        print!("\x1b[H");
        
        for _y in 0..term_h {
            let mut line = String::with_capacity(term_w * 12);
            for _x in 0..term_w {
                let r = xorshift(&mut seed);
                let c = if r % 2 == 0 { '0' } else { '1' };
                
                let color = match r % 10 {
                    0 => WHITE,
                    1 | 2 => BRIGHT_GREEN,
                    _ => GREEN,
                };
                
                line.push_str(&format!("{}{}", color, c));
            }
            line.push_str(RESET);
            println!("{}", line);
        }
        
        let countdown_text = format!("  [ {}s ]  ", remaining);
        let mid_y = term_h / 2;
        let mid_x = (term_w / 2).saturating_sub(countdown_text.len() / 2);
        
        print!("\x1b[{};{}H{}{}{}{}", mid_y, mid_x, BOLD, WHITE, countdown_text, RESET);
        
        io::stdout().flush().ok();
        thread::sleep(Duration::from_millis(100));
    }

    print!("\x1b[2J\x1b[H");
    io::stdout().flush().ok();
    std::process::exit(0);
}

