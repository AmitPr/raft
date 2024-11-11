use std::collections::HashSet;

use crate::NodeId;

pub trait State {}

#[derive(Debug)]
pub struct Follower {
    pub leader: Option<NodeId>,
    pub voted_for: Option<NodeId>,
}

#[derive(Debug)]
pub struct Candidate {
    pub votes: HashSet<NodeId>,
}

#[derive(Debug)]
pub struct Leader {}

impl State for Follower {}
impl State for Candidate {}
impl State for Leader {}
