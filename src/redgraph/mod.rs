use std::num::NonZeroI128;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(pub(crate) NonZeroI128);

pub mod node;
pub mod link;