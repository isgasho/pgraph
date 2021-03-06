mod algo;
pub use algo::*;

mod edges;
pub use edges::*;

mod graph;
pub use graph::*;

mod traversal;
pub use traversal::*;

mod visit;
pub use visit::*;

pub mod prelude;

// Index into the NodeIndex and EdgeIndex arrays
/// Edge direction.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    /// An `Outgoing` edge is an outward edge *from* the current node.
    Outgoing = 0,
    /// An `Incoming` edge is an inbound edge *to* the current node.
    Incoming = 1,
}
