#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::fmt::{self, Write};
use spin::Mutex;

entry_point!(kernel_main);

static CONSOLE: Mutex<Console> = Mutex::new(Console::new());

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let mut console = CONSOLE.lock();
    let _ = writeln!(console, "Bienvenido a ZerOS, escribe help para empezar");
    drop(console);

    shell_loop()
}

fn shell_loop() -> ! {
    let mut line = [0u8; 128];
    loop {
        print("\nzerOS> ");
        let len = read_line(&mut line);
        run_command(core::str::from_utf8(&line[..len]).unwrap_or(""));
    }
}

fn run_command(cmd: &str) {
    match cmd.trim() {
        "help" => print("Comandos: help, apps, calc, clock, notepad, clear\n"),
        "apps" => print("Instaladas: calculator.bap, clock.bap, notepad.bap\n"),
        "calc" => print("calculator.bap ejecutada (demo)\n"),
        "clock" => print("clock.bap ejecutada (demo)\n"),
        "notepad" => print("notepad.bap ejecutada (demo)\n"),
        "clear" => CONSOLE.lock().clear(),
        "" => {}
        _ => print("Comando desconocido. escribe help\n"),
    }
}

fn read_line(buf: &mut [u8]) -> usize {
    let mut idx = 0;
    loop {
        if let Some(ch) = poll_keyboard_char() {
            match ch {
                '\n' => {
                    print("\n");
                    return idx;
                }
                '\u{8}' => {
                    if idx > 0 {
                        idx -= 1;
                        print("\u{8} \u{8}");
                    }
                }
                _ if idx < buf.len() => {
                    buf[idx] = ch as u8;
                    idx += 1;
                    let mut out = [0u8; 4];
                    let s = ch.encode_utf8(&mut out);
                    print(s);
                }
                _ => {}
            }
        }
    }
}

fn poll_keyboard_char() -> Option<char> {
    let scancode = unsafe { inb(0x60) };
    match scancode {
        0x1C => Some('\n'),
        0x39 => Some(' '),
        0x0E => Some('\u{8}'),
        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0A => Some('9'),
        0x0B => Some('0'),
        0x10 => Some('q'), 0x11 => Some('w'), 0x12 => Some('e'), 0x13 => Some('r'),
        0x14 => Some('t'), 0x15 => Some('y'), 0x16 => Some('u'), 0x17 => Some('i'),
        0x18 => Some('o'), 0x19 => Some('p'), 0x1E => Some('a'), 0x1F => Some('s'),
        0x20 => Some('d'), 0x21 => Some('f'), 0x22 => Some('g'), 0x23 => Some('h'),
        0x24 => Some('j'), 0x25 => Some('k'), 0x26 => Some('l'), 0x2C => Some('z'),
        0x2D => Some('x'), 0x2E => Some('c'), 0x2F => Some('v'), 0x30 => Some('b'),
        0x31 => Some('n'), 0x32 => Some('m'),
        _ => None,
    }
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let mut val: u8;
    core::arch::asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack, preserves_flags));
    val
}

fn print(s: &str) {
    let _ = CONSOLE.lock().write_str(s);
}

struct Console;
impl Console {
    const fn new() -> Self { Self }
    fn clear(&mut self) {}
}
impl Write for Console {
    fn write_str(&mut self, _s: &str) -> fmt::Result { Ok(()) }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = writeln!(CONSOLE.lock(), "panic: {info}");
    loop {}
}
