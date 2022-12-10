use std::collections::HashMap;

use crate::{Instruction, Node, OpCode, Program, Value};

#[derive(Debug)]
pub enum DialogueEvent {
    Line,
    Command,
    Options,
}

#[derive(Debug, Default, Clone)]
pub struct DialogueLine {
    /// The string ID for this line. Use it to look up the player-facing
    /// text in a string table.
    pub id: String,

    /// Values that should be substituted into the text before displaying
    /// it. These generally come from variables in the script.
    pub substitutions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DialogueOption {
    /// The identifying number for this option. Pass this to
    /// [`Dialogue::set_selected_option`].
    #[doc(alias = "id")]
    pub index: usize,

    /// The line of text that should be displayed for this
    /// option.
    pub line: DialogueLine,

    /// The node that will be run if this option is selected.
    pub destination: String,

    /// Whether or not this option should be available to the
    /// player. If this is false, you may want to hide this
    /// option, or make it unselectable.
    pub available: bool,
}

enum ExecutionState {
    Stopped,
    WaitingOnOptionSelection,
    WaitingForContinue,
    Running,
}

pub struct Dialogue {
    execution_state: ExecutionState,

    nodes: HashMap<String, Node>,

    current_node: String,
    current_index: usize,

    stack: Vec<Value>,
    variables: HashMap<String, Value>,

    current_line: DialogueLine,
    current_command: String,
    current_options: Vec<DialogueOption>,
}

impl Dialogue {
    pub fn new() -> Dialogue {
        Dialogue {
            execution_state: ExecutionState::Stopped,

            nodes: HashMap::new(),

            current_node: "Start".into(),
            current_index: 0,

            stack: vec![],
            variables: HashMap::new(),

            current_line: DialogueLine::default(),
            current_command: String::new(),
            current_options: vec![],
        }
    }

    /// Starts or continues execution of the dialogue.
    ///
    /// The equivalent method in C# is called `Continue`, but as this is a
    /// keyword in Rust, the name has been changed.
    #[doc(alias = "continue")]
    pub fn advance(&mut self) -> Option<DialogueEvent> {
        match self.execution_state {
            ExecutionState::Running => panic!("cannot advance script that is already running"),
            ExecutionState::WaitingOnOptionSelection => {
                panic!("cannot advance script that is waiting for option")
            }

            _ => {}
        };

        self.execution_state = ExecutionState::Running;

        let mut node = &self.nodes[&self.current_node];

        loop {
            let instruction = &node.instructions[self.current_index];
            self.current_index += 1;

            match instruction.opcode() {
                OpCode::JumpTo => {
                    let target = get_string_operand(instruction, 0);
                    self.current_index = node.labels[target] as usize;
                }

                OpCode::Jump => {
                    let target = match self.stack.last() {
                        Some(Value::StringValue(s)) => s,
                        other => panic!("unexpected value: {:?}", other),
                    };

                    self.current_index = node.labels[target] as usize;
                }

                OpCode::RunLine => {
                    let id = get_string_operand(instruction, 0).to_owned();
                    let sub_count = get_float_operand(instruction, 1);

                    let substitutions = self
                        .stack
                        .drain(self.stack.len() - sub_count as usize..)
                        .map(value_to_string)
                        .collect();

                    self.current_line = DialogueLine { id, substitutions };

                    self.execution_state = ExecutionState::WaitingForContinue;

                    return Some(DialogueEvent::Line);
                }

                OpCode::RunCommand => {
                    let command = get_string_operand(instruction, 0).to_owned();

                    self.current_command = command;
                    self.execution_state = ExecutionState::WaitingForContinue;

                    return Some(DialogueEvent::Command);
                }

                OpCode::AddOption => {
                    let id = get_string_operand(instruction, 0).to_owned();
                    let destination = get_string_operand(instruction, 1).to_owned();

                    let sub_count = get_float_operand(instruction, 2);

                    let substitutions = self
                        .stack
                        .drain(self.stack.len() - sub_count as usize..)
                        .map(value_to_string)
                        .collect();

                    let line = DialogueLine { id, substitutions };

                    let has_condition = get_bool_operand(instruction, 3);

                    let available = if has_condition {
                        match self.stack.pop() {
                            Some(Value::BoolValue(x)) => x,
                            other => panic!("unexpected value: {:?}", other),
                        }
                    } else {
                        true
                    };

                    self.current_options.push(DialogueOption {
                        index: self.current_options.len(),
                        line,
                        destination,
                        available,
                    });
                }

                OpCode::ShowOptions => {
                    self.execution_state = ExecutionState::WaitingOnOptionSelection;

                    return Some(DialogueEvent::Options);
                }

                OpCode::PushString => {
                    let string = get_string_operand(instruction, 0).to_owned();

                    self.stack.push(Value::StringValue(string));
                }

                OpCode::PushFloat => {
                    let float = get_float_operand(instruction, 0);

                    self.stack.push(Value::FloatValue(float));
                }

                OpCode::PushBool => {
                    let bool = get_bool_operand(instruction, 0);

                    self.stack.push(Value::BoolValue(bool));
                }

                OpCode::PushNull => {
                    unimplemented!();
                }

                OpCode::JumpIfFalse => {
                    let cond = match self.stack.last() {
                        Some(Value::BoolValue(x)) => *x,
                        other => panic!("unexpected value: {:?}", other),
                    };

                    if !cond {
                        let target = get_string_operand(instruction, 0);
                        self.current_index = node.labels[target] as usize;
                    }
                }

                OpCode::Pop => {
                    self.stack.pop();
                }

                OpCode::CallFunc => {
                    unimplemented!()
                }

                OpCode::PushVariable => {
                    let name = get_string_operand(instruction, 0);

                    self.stack.push(self.variables[name].clone());
                }

                OpCode::StoreVariable => {
                    let value = match self.stack.pop() {
                        Some(x) => x,
                        None => unimplemented!(),
                    };

                    let name = get_string_operand(instruction, 0).to_owned();

                    self.variables.insert(name, value);
                }

                OpCode::Stop => {
                    self.execution_state = ExecutionState::Stopped;
                    return None;
                }

                OpCode::RunNode => {
                    let target = match self.stack.pop() {
                        Some(Value::StringValue(s)) => s,
                        other => panic!("unexpected value: {:?}", other),
                    };

                    self.current_node = target;
                    self.current_index = 0;

                    node = &self.nodes[&self.current_node];
                }
            }
        }
    }

    pub fn current_line(&self) -> &DialogueLine {
        &self.current_line
    }

    pub fn current_command(&self) -> &str {
        &self.current_command
    }

    pub fn current_options(&self) -> &[DialogueOption] {
        &self.current_options
    }

    pub fn add_program(&mut self, program: &Program) {
        self.nodes
            .extend(program.nodes.iter().map(|(k, v)| (k.clone(), v.clone())));

        // TODO: Don't think this is quite right
        self.variables.extend(
            program
                .initial_values
                .iter()
                .map(|(k, v)| (k.clone(), v.value.as_ref().unwrap().clone())),
        )
    }

    pub fn set_selected_option(&mut self, option: usize) {
        match self.execution_state {
            ExecutionState::WaitingOnOptionSelection => {}
            _ => panic!("not waiting on option selection"),
        }

        if option >= self.current_options.len() {
            panic!("out of bounds option");
        }

        let destination = &self.current_options[option].destination;
        self.stack.push(Value::StringValue(destination.clone())); // TODO: can avoid this

        self.current_options.clear();

        self.execution_state = ExecutionState::WaitingForContinue;
    }
}

impl Default for Dialogue {
    fn default() -> Self {
        Dialogue::new()
    }
}

pub fn expand_substitutions(text: &str, substitutions: &[String]) -> String {
    // TODO: Do this properly.
    if !substitutions.is_empty() {
        text.replace("{0}", &substitutions[0])
    } else {
        text.to_owned()
    }
}

fn get_operand(instruction: &Instruction, index: usize) -> &Value {
    match instruction
        .operands
        .get(index)
        .and_then(|o| o.value.as_ref())
    {
        Some(x) => x,
        None => panic!("missing operand at index {}", index),
    }
}

fn get_string_operand(instruction: &Instruction, index: usize) -> &str {
    match get_operand(instruction, index) {
        Value::StringValue(x) => x,
        other => panic!("unexpected operand at index {}: {:?}", index, other),
    }
}

fn get_bool_operand(instruction: &Instruction, index: usize) -> bool {
    match get_operand(instruction, index) {
        Value::BoolValue(x) => *x,
        other => panic!("unexpected operand at index {}: {:?}", index, other),
    }
}

fn get_float_operand(instruction: &Instruction, index: usize) -> f32 {
    match get_operand(instruction, index) {
        Value::FloatValue(x) => *x,
        other => panic!("unexpected operand at index {}: {:?}", index, other),
    }
}

fn value_to_string(value: Value) -> String {
    match value {
        Value::StringValue(x) => x,
        Value::BoolValue(x) => x.to_string(),
        Value::FloatValue(x) => x.to_string(),
    }
}
