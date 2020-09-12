use std::io::{ BufReader, BufRead, BufWriter, Read, Write };
use std::sync::mpsc::{ channel, Sender, Receiver };

#[derive(Debug)]
pub struct Engine {
    child: std::process::Child,
    rx: Receiver<String>,
    tx: Sender<String>,
}

impl Engine {
    pub fn new(name: &str, args: &[&str], rx: Receiver<String>) -> Result<(Self, Receiver<String>), std::io::Error> {
        let (tx, controller_receiver) = channel();
        let mut child = std::process::Command::new(name)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let mut stdin  = BufWriter::new(child.stdin.take().unwrap());
        let mut stdout = BufReader::new(child.stdout.take().unwrap());
        let mut stderr = BufReader::new(child.stderr.take().unwrap());
        // start thread to listen to stderr
        Ok((Self {
            child,
            rx,
            tx,
        }, controller_receiver))
    }
}

impl Write for Engine {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.child.stdin.as_mut().expect("failed to get stdin").write(buf)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.child.stdin.as_mut().expect("failed to get stdin").flush()
    }
}

impl Read for Engine {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.child.stdout.as_mut().expect("failed to get stdout").read(buf)
    }
}
