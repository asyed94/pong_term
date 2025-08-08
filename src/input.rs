//! Terminal input handling with raw mode (no external dependencies).

use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

/// Input events that the game can handle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEvent {
    LeftPaddleUp,
    LeftPaddleDown,
    RightPaddleUp,
    RightPaddleDown,
    Quit,
    Pause,
    None,
}

/// Terminal state manager for raw mode.
pub struct Terminal {
    original_termios: Termios,
}

impl Terminal {
    /// Enter raw mode for non-blocking input.
    pub fn enter_raw_mode() -> io::Result<Self> {
        let stdin_fd = io::stdin().as_raw_fd();
        let original = get_termios(stdin_fd)?;

        let mut raw = original;

        // Turn off canonical mode and echo
        raw.c_lflag &= !(ICANON | ECHO | IEXTEN | ISIG);
        raw.c_iflag &= !(IXON | ICRNL | BRKINT | INPCK | ISTRIP);
        // raw.c_oflag &= !OPOST;
        raw.c_cflag |= CS8;

        // Set minimum characters and timeout for read
        raw.c_cc[VMIN] = 0; // Non-blocking
        raw.c_cc[VTIME] = 0;

        set_termios(stdin_fd, &raw)?;

        Ok(Terminal {
            original_termios: original,
        })
    }

    /// Restore terminal to original mode.
    pub fn leave_raw_mode(&self) -> io::Result<()> {
        let stdin_fd = io::stdin().as_raw_fd();
        set_termios(stdin_fd, &self.original_termios)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.leave_raw_mode();
        // Also exit alternate screen to restore terminal
        let _ = crate::render::exit_alternate_screen();
        let _ = crate::render::show_cursor();
    }
}

/// Wait for the user to press Enter without echoing any typed characters.
/// This is useful for "Press Enter to continue" prompts where we don't want
/// typed characters to pollute the display.
pub fn wait_for_enter_no_echo() -> io::Result<()> {
    let stdin_fd = io::stdin().as_raw_fd();

    // Get current terminal settings
    let original = get_termios(stdin_fd)?;

    // Create modified settings with echo disabled
    let mut no_echo = original;
    no_echo.c_lflag &= !ECHO; // Only disable echo, keep canonical mode

    // Apply the no-echo settings
    set_termios(stdin_fd, &no_echo)?;

    // Wait for Enter (read until newline)
    let mut buf = String::new();
    let result = io::stdin().read_line(&mut buf);

    // Restore original terminal settings
    set_termios(stdin_fd, &original)?;

    // Return the result from read_line
    result.map(|_| ())
}

/// Poll for keyboard input without blocking.
/// Drains the entire input buffer to prevent scroll wheel and other
/// events from creating a backlog that causes delayed processing.
pub fn poll_input() -> io::Result<InputEvent> {
    let mut buffer = [0; 32]; // Larger buffer to handle mouse sequences
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut last_event = InputEvent::None;

    // Drain all available input to prevent buildup
    loop {
        match handle.read(&mut buffer) {
            Ok(0) | Err(_) => break, // No more data available
            Ok(bytes_read) => {
                // Process the input buffer
                let event = process_input_buffer(&buffer[..bytes_read]);
                // Keep the last valid event (most recent user input)
                if event != InputEvent::None {
                    last_event = event;
                }
            }
        }
    }

    Ok(last_event)
}

/// Process a buffer of input bytes and extract keyboard events.
/// Ignores mouse sequences and other non-keyboard input.
fn process_input_buffer(buffer: &[u8]) -> InputEvent {
    if buffer.is_empty() {
        return InputEvent::None;
    }

    match buffer[0] {
        b'w' | b'W' => InputEvent::LeftPaddleUp,
        b's' | b'S' => InputEvent::LeftPaddleDown,
        b'q' | b'Q' => InputEvent::Quit,
        b' ' => InputEvent::Pause,
        27 if buffer.len() >= 3 => {
            // Escape sequences for arrow keys
            if buffer[1] == b'[' {
                match buffer[2] {
                    b'A' => InputEvent::RightPaddleUp,   // Up arrow
                    b'B' => InputEvent::RightPaddleDown, // Down arrow
                    _ => InputEvent::None,
                }
            } else {
                InputEvent::None
            }
        }
        _ => InputEvent::None,
    }
}

// Minimal termios implementation without libc
// Using raw syscalls and constants from the Linux kernel

#[repr(C)]
#[derive(Copy, Clone)]
struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

const TCGETS: u64 = 0x5401;
const TCSETS: u64 = 0x5402;

// c_iflag bits
const _IGNBRK: u32 = 0o000001;
const BRKINT: u32 = 0o000002;
const _IGNPAR: u32 = 0o000004;
const _PARMRK: u32 = 0o000010;
const INPCK: u32 = 0o000020;
const ISTRIP: u32 = 0o000040;
const _INLCR: u32 = 0o000100;
const _IGNCR: u32 = 0o000200;
const ICRNL: u32 = 0o000400;
const _IUCLC: u32 = 0o001000;
const IXON: u32 = 0o002000;
const _IXANY: u32 = 0o004000;
const _IXOFF: u32 = 0o010000;
const _IMAXBEL: u32 = 0o020000;
const _IUTF8: u32 = 0o040000;

// c_oflag bits
const _OPOST: u32 = 0o000001;

// c_cflag bits
const CS8: u32 = 0o000060;

// c_lflag bits
const ISIG: u32 = 0o000001;
const ICANON: u32 = 0o000002;
const ECHO: u32 = 0o000010;
const _ECHOE: u32 = 0o000020;
const _ECHOK: u32 = 0o000040;
const _ECHONL: u32 = 0o000100;
const _NOFLSH: u32 = 0o000200;
const _TOSTOP: u32 = 0o000400;
const IEXTEN: u32 = 0o100000;

// c_cc indices
const VMIN: usize = 6;
const VTIME: usize = 5;

fn get_termios(fd: i32) -> io::Result<Termios> {
    unsafe {
        let mut termios = std::mem::zeroed();
        let result = ioctl(fd, TCGETS, &mut termios as *mut _ as *mut u8);
        if result == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(termios)
    }
}

fn set_termios(fd: i32, termios: &Termios) -> io::Result<()> {
    unsafe {
        let result = ioctl(fd, TCSETS, termios as *const _ as *const u8);
        if result == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

// System call for ioctl
#[cfg(target_arch = "x86_64")]
unsafe fn ioctl(fd: i32, request: u64, argp: *const u8) -> i32 {
    let result: i64;
    std::arch::asm!(
        "syscall",
        in("rax") 16i64,  // SYS_ioctl
        in("rdi") fd as i64,
        in("rsi") request,
        in("rdx") argp,
        lateout("rax") result,
        out("rcx") _,
        out("r11") _,
    );
    result as i32
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn ioctl(_fd: i32, _request: u64, _argp: *const u8) -> i32 {
    // Fallback for non-x86_64 architectures
    -1
}
