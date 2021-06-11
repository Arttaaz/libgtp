use std::sync::Arc;
use std::sync::Mutex;
use std::io::{ BufReader, BufRead, BufWriter, Read, Write };
use std::process::ChildStdin;
use std::process::ChildStdout;

#[derive(Debug)]
pub struct Engine {
    child: std::process::Child,
    stdout: BufReader<ChildStdout>,
    stdin: BufWriter<ChildStdin>,
    pub is_ready: Arc<Mutex<bool>>,
}

impl Engine {
    pub fn new(name: &str, args: &[&str]) -> Result<Self, std::io::Error> {
        let mut child = std::process::Command::new(name)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let stdin  = BufWriter::new(child.stdin.take().unwrap());
        let stdout = BufReader::new(child.stdout.take().unwrap());
        //TODO start thread to listen to stderr
        Ok(Self {
            child,
            stdout,
            stdin,
            is_ready: Arc::new(Mutex::new(false)),
        })
    }

    pub fn start(mut self) -> Self {
        let is_ready = self.is_ready.clone();
        let stderr = BufReader::new(self.child.stderr.take().unwrap());
        std::thread::spawn(move || {
            for l in stderr.lines().map(|x| x.unwrap()) {
                eprintln!("{}", l);
                if l == "GTP ready, beginning main protocol loop".to_string() {
                    let mut is_ready = is_ready.lock().unwrap();
                    *is_ready = true;
                }
            }
        });
        self
    }

    pub fn kill(mut self) {
        self.child.kill().unwrap();
    }
}

impl Write for Engine {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.stdin.write(buf)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.stdin.flush()
    }
}

impl Read for Engine {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.stdout.read(buf)
    }
}
