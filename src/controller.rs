use std::io::{ Write, Read };
use alloc::collections::VecDeque;
use crate::model::Command;

#[derive(Debug)]
pub struct Controller<W: Write + Read> {
    /// link to engine
    engine: W,
    /// commands waiting an answer from the engine
    waiting_for_answer: VecDeque<Command>,
}

impl<W: Write + Read> Controller<W> {
    pub fn new(engine: W) -> Self {
        Self {
            engine,
            waiting_for_answer: VecDeque::new(),
        }
    }

    pub fn send_command(&mut self, command: Command) -> Result<(), std::io::Error>{
        self.engine.write_all(command.to_string().as_bytes())
    }
}
