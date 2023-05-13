use petgraph::graphmap::{NodeTrait};
use super::{Id, link::Link};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NodeKind {
	// io nodes

	Input,
	Output,
	
	// node

	IdentityNode, // Node(b) -> 1 IF b ELSE -1
	Inverter,

	// bias

	Biaser, // Biaser(n1, n2[]) -> max(0, n1 - max(n2))
	ConditionalBlocker, // Biaser(n1, n2[]) -> n1 IF max(n2) < n1
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Node {
	pub(crate) kind: NodeKind,
	pub(crate) inputs: Vec<Link>,
	pub(crate) outputs: Vec<Link>
}