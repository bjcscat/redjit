use super::Id;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Link {
    pub(crate) input: Id,
    pub(crate) output: Id,
    pub(crate) bias: u8,
}