use libc::{signal, SIG_IGN, SIGTERM, SIGINT, SIGQUIT, SIGHUP, SIGPIPE, SIGTSTP, prctl, PR_SET_NAME, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use std::ffi::CString;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
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

fn rename_loop(stop: Arc<AtomicBool>) {
    let mut seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    while !stop.load(Ordering::Relaxed) {
        let mut name = String::with_capacity(20);
        for _ in 0..20 {
            let r = xorshift(&mut seed);
            let idx = (r % chars.len() as u64) as usize;
            name.push(chars[idx] as char);
        }
        
        let cname = CString::new(name).unwrap();
        unsafe { prctl(PR_SET_NAME, cname.as_ptr(), 0, 0, 0); }
        
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    ignore_signals();

    let stop = Arc::new(AtomicBool::new(false));
    let rename_stop = stop.clone();
    
    thread::spawn(move || {
        rename_loop(rename_stop);
    });

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
    
    stop.store(true, Ordering::Relaxed);
    
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().ok();
    std::process::exit(0);
}
