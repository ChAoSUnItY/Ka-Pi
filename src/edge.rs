use std::rc::Rc;

use crate::label::Label;

pub(crate) struct Edge {
    pub(crate) info: i32,
    pub(crate) successor: Rc<dyn Label>,
    pub(crate) next_edge: Option<Rc<Edge>>
}

impl Edge {
    pub(crate) const JUMP: u32 = 0;
    pub(crate) const EXCEPTION: u32 = 0x7FFFFFFF;

    pub const fn new(info: i32, successor: Rc<dyn Label>, next_edge: Option<Rc<Edge>>) -> Self {
        Self{
            info,
            successor,
            next_edge
        }
    }
}
