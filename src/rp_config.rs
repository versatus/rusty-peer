
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct RustyPeerConfig<V: Copy> {
    init_trust: V,
    n_neighbors: usize,
    // TODO: add other properties
}

impl<V: Copy> RustyPeerConfig<V> {
    pub fn new(init_trust: V, n_neighbors: usize) -> RustyPeerConfig<V> {
        RustyPeerConfig { init_trust, n_neighbors }
    }

    pub fn init_trust(&self) -> V {
        self.init_trust
    }

    pub fn n_neighbors(&self) -> usize {
        self.n_neighbors
    }
}
