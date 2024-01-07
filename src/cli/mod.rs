pub mod opts;

pub const STDERR: &str = "stderr";
pub const STDOUT: &str = "stdout";

pub fn stderr() -> Option<String> {
    Some(STDERR.to_string())
}

pub fn stdout() -> Option<String> {
    Some(STDOUT.to_string())
}

#[cfg(unix)]
pub fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
pub fn reset_sigpipe() {
    // no-op
}
