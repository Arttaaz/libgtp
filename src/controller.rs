use std::io::Write;
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
            Ok(e) => e.start(),
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
        self.engine.write(command.to_string().as_bytes())?;
        self.engine.flush()?;
        let s = self.engine.read_line();
        Ok(Answer::parse_answer(s.as_str()).unwrap())
    }

    pub fn read_info(&self) -> Result<Option<crate::model::Info>, crate::model::ParseError> {
        let s = self.engine.read_info();
        if s.is_empty() {
            return Ok(None)
        }
        let answer = Answer::parse_answer(s.as_str())?;
        if answer.is_info() {
            Ok(Some(answer.to_info().unwrap()))
        } else {
            Err(crate::model::ParseError::WrongAnswerFormat)
        }
    }

    pub fn discard_info(&mut self) {
        self.engine.discard_info();
    }
}
