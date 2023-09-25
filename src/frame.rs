use crate::label::Label;

#[derive(Debug)]
pub(crate) struct Frame {
  label_idx: usize,
  input_locals: Vec<u32>,
  input_stack: Vec<u32>,
  output_locals: Vec<u32>,
  output_stack: Vec<u32>,
  output_stack_start: u16,
  output_stack_top: u16,
  initialization_count: u32,
  initializations: Vec<u32>,
}

impl Frame {
  pub(crate) fn new(label_idx: usize) -> Self {
    Self {
      label_idx,
      input_locals: vec![],
      input_stack: vec![],
      output_locals: vec![],
      output_stack: vec![],
      output_stack_start: 0,
      output_stack_top: 0,
      initialization_count: 0,
      initializations: vec![],
    }
  }

  fn copy_from(&mut self, frame: &Self) {
    self.input_locals = frame.input_locals.clone();
    self.input_stack = frame.input_stack.clone();
    self.output_locals = frame.output_locals.clone();
    self.output_stack = frame.output_stack.clone();
    self.output_stack_start = 0;
    self.output_stack_top = frame.output_stack_top;
    self.initialization_count = frame.initialization_count;
    self.initializations = frame.initializations.clone();
  }
}
