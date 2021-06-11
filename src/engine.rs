use std::io::{ BufReader, BufRead, BufWriter, Read, Write };
use std::process::ChildStdin;
use std::process::ChildStdout;

#[derive(Debug)]
pub struct Engine {
    child: std::process::Child,
    stdout: BufReader<ChildStdout>,
    stdin: BufWriter<ChildStdin>,
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
        let stderr = BufReader::new(child.stderr.take().unwrap());
        //TODO start thread to listen to stderr
        std::thread::spawn(|| {
            for l in stderr.lines() {
                println!("{}", l.unwrap());
            }
            println!("oopsie");
        });
        Ok(Self {
            child,
            stdout,
            stdin,
        })
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
