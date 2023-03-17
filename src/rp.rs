use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, AddAssign, DivAssign, Mul};
use crate::rp_config::RustyPeerConfig;

#[derive(Clone, Debug)]
pub struct RustyPeer<K,V: Copy> {
    local_trust: HashMap<K, V>,
    global_trust: HashMap<K, V>,
    normalized_local_trust: HashMap<K, f64>,
    normalized_global_trust: HashMap<K, f64>,
    rp_config: RustyPeerConfig<V>, 
}

impl<K,V> RustyPeer<K,V> 
where
    K: Eq + Hash + Clone,
    V: AddAssign + DivAssign + Add<Output = V> + Mul<Output = V> + Default +  Copy + Into<f64>
{
    pub fn new(rp_config: RustyPeerConfig<V>) -> Self {
        RustyPeer {
            local_trust: HashMap::with_capacity(rp_config.n_neighbors()),
            global_trust: HashMap::with_capacity(rp_config.n_neighbors()),
            normalized_local_trust: HashMap::with_capacity(rp_config.n_neighbors()),
            normalized_global_trust: HashMap::with_capacity(rp_config.n_neighbors()),
            rp_config,
            //TODO: Add a RustyPeerConfig type that provides
            //some of the basic configuration, including what 
            //determines if a node is local, the init value, etc.
        }
    }

    pub fn init_local_trust(&mut self, node_id: K) {
        self.local_trust.entry(node_id).or_insert(self.rp_config.init_trust());
    }

    pub fn init_global_trust(&mut self, node_id: K, init_trust_delta: V) {
        self.global_trust.entry(node_id).or_insert(init_trust_delta);
    }

    pub fn update_local_trust(&mut self, node_id: &K, trust_delta: V) {
        if let Some(trust_score) = self.local_trust.get_mut(node_id) {
            *trust_score += trust_delta;
        } 
        self.normalize_local_trust();
        //TODO: This is a new local peer, add to local trust map with init
        //method.
    }

    pub fn update_global_trust(&mut self, node_id: &K, trust_delta: V) {
        if let Some(trust_score) = self.local_trust.get_mut(node_id) {
            *trust_score += trust_delta;
        } 
        self.normalize_global_trust();
        //TODO: This is a new global peer, add to local trust map with init
        //method.
    }
    
    pub fn get_local_trust(&self, node_id: &K) -> Option<&V> {
        self.local_trust.get(node_id)
    }

    pub fn get_global_trust(&self, node_id: &K) -> Option<&V> {
        self.global_trust.get(&node_id)
    }

    pub fn get_mut_local_trust(&mut self, node_id: &K) -> Option<&mut V> {
        self.local_trust.get_mut(&node_id)
    }

    pub fn get_mut_global_trust(&mut self, node_id: &K) -> Option<&mut V> {
        self.global_trust.get_mut(&node_id)
    }

    pub fn get_normalized_local_trust(&self, node_id: &K) -> Option<&f64> {
        self.normalized_local_trust.get(&node_id) 
    }

    pub fn get_normalized_global_trust(&self, node_id: &K) -> Option<&f64> {
        self.normalized_global_trust.get(&node_id)
    }

    pub fn get_mut_normalized_local_trust(&mut self, node_id: &K) -> Option<&mut f64> {
        self.normalized_local_trust.get_mut(node_id)
    }

    pub fn get_mut_normalized_global_trust(&mut self, node_id: &K) -> Option<&mut f64> {
        self.normalized_global_trust.get_mut(node_id)
    }

    fn normalize_local_trust(&mut self) {
        let total_trust: f64 = self.local_trust.values()
            .cloned().fold(V::default(), |acc, x| acc + x).into();
        self.normalized_local_trust = self.local_trust.iter_mut().map(|(k, v)| {
            let trust_score: V = *v;
            let trust_score = trust_score.into();
            let normalized_trust_score: f64 = trust_score / total_trust;
            (k.clone(), normalized_trust_score)
        }).collect();
    }

    fn normalize_global_trust(&mut self) {
        let total_trust: f64 = self.global_trust.values().cloned().fold(V::default(), |acc, x| acc + x).into();
        self.normalized_global_trust = self.global_trust.iter_mut().map(|(k, v)| {
            let trust_score: V = *v;
            let trust_score = trust_score.into();
            let normalized_trust_score: f64 = trust_score / total_trust;
            (k.clone(), normalized_trust_score)
        }).collect();
    }
}
