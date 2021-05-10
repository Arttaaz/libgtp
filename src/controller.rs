use std::io::{ Write, Read };
use alloc::collections::VecDeque;
use crate::model::Command;
use crate::model::Answer;
use crate::Engine;
use log::error;

#[derive(Debug)]
pub struct Controller {
    /// link to engine
    engine: Engine,
    /// commands waiting an answer from the engine
    waiting_for_answer: VecDeque<Command>,
}

impl Controller {
    pub fn new(engine_name: &str, engine_args: &[&str]) -> Self {
        let engine = match Engine::new(engine_name, engine_args) {
            Ok(e) => e,
            Err(e) => {
                error!("{}", e);
                    panic!();
            },
        };
        Self {
            engine,
            waiting_for_answer: VecDeque::new(),
        }
    }

    pub fn send_command(&mut self, command: Command) -> Result<Answer, std::io::Error>{
        self.engine.write_all(command.to_string().as_bytes())?;
        let mut s: String = String::new();
        self.engine.read_to_string(&mut s)?;
        Ok(Answer::parse_answer(s.as_str()).unwrap())
    }
}
