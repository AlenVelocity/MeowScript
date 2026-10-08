//! The interpreter reaches the outside world only through a `Host`, which is what lets the same
//! crate run on the command line, in tests, and in a browser.

use crate::error::{Error, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Host {
    /// `line` has no trailing newline.
    fn print(&mut self, line: &str);

    fn read_line(&mut self, prompt: &str) -> Result<String> {
        let _ = prompt;
        Err(Error::io("there is no keyboard to read from here"))
    }

    fn sleep(&mut self, millis: f64) {
        let _ = millis;
    }

    fn now_millis(&mut self) -> f64 {
        0.0
    }

    fn read_file(&mut self, path: &str) -> Result<String> {
        let _ = path;
        Err(Error::io("there is no scratchpad (filesystem) here"))
    }

    fn write_file(&mut self, path: &str, contents: &str, append: bool) -> Result<()> {
        let _ = (path, contents, append);
        Err(Error::io("there is no scratchpad (filesystem) here"))
    }

    fn file_exists(&mut self, path: &str) -> bool {
        let _ = path;
        false
    }

    fn random_seed(&mut self) -> u64 {
        0x9E37_79B9_7F4A_7C15
    }
}

/// Collects output in memory. Clones share the buffer, so keep one and give the other to the
/// interpreter.
#[derive(Clone, Default)]
pub struct BufferHost {
    lines: Rc<RefCell<Vec<String>>>,
    seed: u64,
}

impl BufferHost {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed,
            ..Self::default()
        }
    }

    pub fn lines(&self) -> Vec<String> {
        self.lines.borrow().clone()
    }

    pub fn output(&self) -> String {
        self.lines.borrow().join("\n")
    }

    pub fn take(&self) -> Vec<String> {
        std::mem::take(&mut *self.lines.borrow_mut())
    }
}

impl Host for BufferHost {
    fn print(&mut self, line: &str) {
        self.lines.borrow_mut().push(line.to_string());
    }

    fn random_seed(&mut self) -> u64 {
        self.seed
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeHost;

#[cfg(not(target_arch = "wasm32"))]
impl Host for NativeHost {
    fn print(&mut self, line: &str) {
        use std::io::Write;
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        let _ = writeln!(lock, "{line}");
        let _ = lock.flush();
    }

    fn read_line(&mut self, prompt: &str) -> Result<String> {
        use std::io::{BufRead, Write};
        let mut stdout = std::io::stdout();
        let _ = write!(stdout, "{prompt}");
        let _ = stdout.flush();
        let mut line = String::new();
        std::io::stdin()
            .lock()
            .read_line(&mut line)
            .map_err(|e| Error::io(format!("could not read input: {e}")))?;
        while line.ends_with(['\n', '\r']) {
            line.pop();
        }
        Ok(line)
    }

    fn sleep(&mut self, millis: f64) {
        if millis > 0.0 {
            std::thread::sleep(std::time::Duration::from_secs_f64(millis / 1000.0));
        }
    }

    fn now_millis(&mut self) -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0)
    }

    fn read_file(&mut self, path: &str) -> Result<String> {
        std::fs::read_to_string(path)
            .map_err(|e| Error::io(format!("could not read `{path}`: {e}")))
    }

    fn write_file(&mut self, path: &str, contents: &str, append: bool) -> Result<()> {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .truncate(!append)
            .open(path)
            .map_err(|e| Error::io(format!("could not open `{path}`: {e}")))?;
        file.write_all(contents.as_bytes())
            .map_err(|e| Error::io(format!("could not write `{path}`: {e}")))
    }

    fn file_exists(&mut self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    fn random_seed(&mut self) -> u64 {
        use std::hash::{BuildHasher, Hasher};
        let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
        hasher.write_u128(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
        );
        hasher.finish()
    }
}
