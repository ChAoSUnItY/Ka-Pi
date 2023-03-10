use std::rc::Rc;

use crate::asm::label::Label;

pub(crate) const JUMP: u32 = 0;
pub(crate) const EXCEPTION: u32 = 0x7FFFFFFF;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Edge {
    pub(crate) info: i32,
    pub(crate) successor: Rc<Label>,
    pub(crate) next_edge: Option<Rc<Edge>>,
}

impl Edge {
    pub const fn new(info: i32, successor: Rc<Label>, next_edge: Option<Rc<Edge>>) -> Self {
        Self {
            info,
            successor,
            next_edge,
        }
    }
}
