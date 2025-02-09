use regex::Regex;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

const I_AM_DONE_REGEX: &str = r"(?m)^\s*//?\s*I\s+AM\s+NOT\s+DONE";
const CONTEXT: usize = 2;

// The mode of the exercise.
#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Compile,
    Execute,
    Test,
}

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

// A representation of a noirlings exercise
#[derive(Deserialize, Debug)]
pub struct Exercise {
    // Name of the exercise
    pub name: String,
    // The path to the file containing the exercise's source code
    pub path: PathBuf,
    // The mode of the exercise (Compile/Execute/Test)
    pub mode: Mode,
    // The hint text associated with the exercise
    pub hint: String,
}

// An enum to track the state of an Exercise
#[derive(PartialEq, Debug)]
pub enum State {
    Done,
    Pending(Vec<ContextLine>),
}

#[derive(PartialEq, Debug)]
pub struct ContextLine {
    pub line: String,
    pub number: usize,
    pub important: bool,
}

impl Exercise {
    pub fn compile(&self) -> anyhow::Result<String> {
        // TODO: Implement nargo compile
        todo!()
    }

    pub fn execute(&self) -> anyhow::Result<String> {
        // TODO: Implement nargo execute
        todo!()
    }

    pub fn test(&self) -> anyhow::Result<String> {
        // TODO: Implement nargo test
        todo!()
    }

    pub fn state(&self) -> State {
        let mut source_file = File::open(&self.path)
            .unwrap_or_else(|_| panic!("Unable to open the exercise file! {:?}", self.path));

        let source = {
            let mut s = String::new();
            source_file
                .read_to_string(&mut s)
                .expect("Unable to read the exercise file!");
            s
        };

        let re = Regex::new(I_AM_DONE_REGEX).unwrap();

        if !re.is_match(&source) {
            return State::Done;
        }

        let matched_line_index = source
            .lines()
            .enumerate()
            .find_map(|(i, line)| if re.is_match(line) { Some(i) } else { None })
            .expect("This should not happen at all");

        let min_line = ((matched_line_index as i32) - (CONTEXT as i32)).max(0) as usize;
        let max_line = matched_line_index + CONTEXT;

        let context = source
            .lines()
            .enumerate()
            .filter(|&(i, _)| i >= min_line && i <= max_line)
            .map(|(i, line)| ContextLine {
                line: line.to_string(),
                number: i + 1,
                important: i == matched_line_index,
            })
            .collect();

        State::Pending(context)
    }

    pub fn looks_done(&self) -> bool {
        self.state() == State::Done
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}
