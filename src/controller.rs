use std::io::{ Write, Read };
use std::sync::mpsc::{ channel, Sender, Receiver };
use alloc::collections::VecDeque;
use crate::model::Command;
use crate::Engine;
use log::error;

#[derive(Debug)]
pub struct Controller {
    /// link to engine
    engine: Engine,
    receiver: Receiver<String>,
    engine_sender: Sender<String>,
    /// commands waiting an answer from the engine
    waiting_for_answer: VecDeque<Command>,
}

impl Controller {
    pub fn new(engine_name: &str, engine_args: &[&str]) -> Self {
        let (tx, rx) = channel();
        let (engine, receiver) = match Engine::new(engine_name, engine_args, rx) {
            Ok((e, r)) => (e, r),
            Err(e) => {
                error!("{}", e);
                    panic!();
            },
        };
        Self {
            engine,
            receiver,
            engine_sender: tx,
            waiting_for_answer: VecDeque::new(),
        }
    }

    pub fn send_command(&mut self, command: Command) -> Result<(), std::io::Error>{
        self.engine.write_all(command.to_string().as_bytes())
    }
}
