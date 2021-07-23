use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::io::{ BufReader, BufRead, BufWriter, Write };
use std::process::ChildStdin;

#[derive(Debug)]
pub struct Engine {
    child: std::process::Child,
    stdin: BufWriter<ChildStdin>,
    incoming_lines: Arc<Mutex<VecDeque<String>>>,
    analyze_line: Arc<Mutex<String>>,
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
        Ok(Self {
            child,
            stdin,
            incoming_lines: Arc::new(Mutex::new(VecDeque::new())),
            analyze_line: Arc::new(Mutex::new("".to_string())),
            is_ready: Arc::new(Mutex::new(false)),
        })
    }

    pub fn start(mut self) -> Self {
        let is_ready = self.is_ready.clone();

        let stdout = BufReader::new(self.child.stdout.take().unwrap());
        let lines = self.incoming_lines.clone();
        let line = self.analyze_line.clone();
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
        std::thread::spawn(move || {
            for l in stdout.lines().map(|x| x.unwrap()) {
                if !l.is_empty() {
                    if l.as_bytes()[0] == b'i' {
                        let mut line = line.lock().unwrap();
                        *line = l;
                    } else {
                        let mut lines = lines.lock().unwrap();
                        println!("{}", l);
                        lines.push_back(l);
                    }
                }
            }
        });

        self
    }

    pub fn kill(mut self) {
        self.child.kill().unwrap();
    }

    pub fn read_line(&mut self) -> String {
        while self.incoming_lines.lock().unwrap().front().is_none() {}
        let s = self.incoming_lines.lock().unwrap().pop_front().unwrap();
        s
    }

    pub fn read_info(&self) -> String {
        self.analyze_line.lock().unwrap().clone()
    }
}

impl Write for Engine {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        while *self.is_ready.lock().unwrap() != true {}

        println!("{}", String::from_utf8(buf.to_vec()).unwrap());
        self.stdin.write(buf)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.stdin.flush()
    }
}

