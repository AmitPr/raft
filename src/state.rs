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

// impl Candidate {
//     pub fn process_message(
//         &mut self,
//         message: Message,
//         node: &mut NodeData,
//     ) -> Result<State, String> {
//         match message {
//             Message::RequestVote { new_term, .. } => {
//                 if new_term > self.term {
//                     return Ok(State::Follower(Follower {
//                         leader: None,
//                         term: new_term,
//                         voted_for: None,
//                     }));
//                 }
//                 Ok(State::Candidate(self.clone()))
//             }
//             Message::Vote {
//                 follower,
//                 new_term,
//                 vote,
//             } => {
//                 if new_term == self.term && vote {
//                     self.votes.push(follower);
//                     if self.votes.len() > node.quorum_size() {
//                         return Ok(State::Leader(Leader { term: self.term }));
//                     }
//                 }
//                 Ok(State::Candidate(self.clone()))
//             }
//             Message::AppendEntries { term, leader, .. } => {
//                 if term >= self.term {
//                     return Ok(State::Follower(Follower {
//                         leader: Some(leader),
//                         term,
//                         voted_for: None,
//                     }));
//                 }
//                 Ok(State::Candidate(self.clone()))
//             }
//             Message::Timeout => {
//                 self.term += 1;
//                 self.votes = vec![node.id.clone()];
//                 node.request_votes();
//                 Ok(State::Candidate(self.clone()))
//             }
//             _ => Err("Unexpected message".to_string()),
//         }
//     }
// }

// impl Leader {
//     pub fn process_message(
//         &mut self,
//         message: Message,
//         node: &mut NodeData,
//     ) -> Result<State, String> {
//         match message {
//             Message::AppendEntries { term, .. } => {
//                 if term > self.term {
//                     return Ok(State::Follower(Follower {
//                         leader: None,
//                         term,
//                         voted_for: None,
//                     }));
//                 }
//                 Ok(State::Leader(self.clone()))
//             }
//             Message::AppendEntriesResponse {
//                 term,
//                 success,
//                 match_index,
//             } => {
//                 if term > self.term {
//                     return Ok(State::Follower(Follower {
//                         leader: None,
//                         term,
//                         voted_for: None,
//                     }));
//                 }
//                 if success {
//                     node.update_match_index(match_index);
//                     node.try_commit_entries();
//                 } else {
//                     node.decrement_next_index();
//                 }
//                 Ok(State::Leader(self.clone()))
//             }
//             Message::Timeout => {
//                 node.send_heartbeats();
//                 Ok(State::Leader(self.clone()))
//             }
//             _ => Err("Unexpected message".to_string()),
//         }
//     }
// }
