use std::collections::{HashMap, HashSet};

use either::Either;

use crate::{Candidate, Cluster, Follower, Leader, LogEntry, Message, NodeId, State};

pub struct Node<S: State, C: Cluster> {
    pub log: Vec<LogEntry>,
    pub commit_index: u64,
    pub last_applied: u64,
    pub next_index: HashMap<NodeId, u64>,
    pub match_index: HashMap<NodeId, u64>,
    pub term: u64,
    pub state: S,
    pub cluster: C,
}

impl<S: State, C: Cluster> Node<S, C> {
    /// Checks if the specified log is at least as up-to-date as the current node's log.
    ///
    /// This method is used during the RequestVote RPC to determine if a candidate's log
    /// is sufficiently up-to-date to receive a vote.
    fn is_log_up_to_date(&self, last_log_index: u64, last_log_term: u64) -> bool {
        let last_entry = self.log.last();
        match last_entry {
            Some(entry) => {
                last_log_term > entry.term
                    || (last_log_term == entry.term && last_log_index >= self.log.len() as u64 - 1)
            }
            None => true,
        }
    }
}

impl<C: Cluster> Node<Follower, C> {
    pub fn new(cluster: C) -> Self {
        Node {
            log: vec![],
            commit_index: 0,
            last_applied: 0,
            next_index: HashMap::new(),
            match_index: HashMap::new(),
            term: 0,
            state: Follower {
                leader: None,
                voted_for: None,
            },
            cluster,
        }
    }

    fn can_vote(&self, new_term: u64) -> bool {
        self.state.voted_for.is_none() && new_term >= self.term
    }
}

impl<C: Cluster> Node<Follower, C> {
    pub async fn timeout(self) -> Node<Candidate, C> {
        Node::<Candidate, C>::promote(self).await
    }

    pub async fn vote_requested(&mut self, candidate: NodeId, term: u64) {
        if self.can_vote(term) {
            self.state.voted_for = Some(candidate.clone());
            self.term = term;

            self.cluster
                .send_message(
                    &candidate,
                    Message::Vote {
                        follower: self.cluster.me().clone(),
                        new_term: self.term,
                        vote: true,
                    },
                )
                .await;
        } else {
            self.cluster
                .send_message(
                    &candidate,
                    Message::Vote {
                        follower: self.cluster.me().clone(),
                        new_term: self.term,
                        vote: false,
                    },
                )
                .await;
        }
    }
}

impl<C: Cluster> Node<Candidate, C> {
    pub async fn promote(before: Node<Follower, C>) -> Self {
        Self::trigger_election(before).await
    }

    pub async fn timeout(self) -> Node<Candidate, C> {
        Self::trigger_election(self).await
    }

    async fn trigger_election<S: State>(before: Node<S, C>) -> Self {
        let me = Node {
            log: before.log,
            commit_index: before.commit_index,
            last_applied: before.last_applied,
            next_index: before.next_index,
            match_index: before.match_index,
            term: before.term + 1,
            state: Candidate {
                votes: HashSet::with_capacity(before.cluster.size()),
            },
            cluster: before.cluster,
        };
        me.cluster
            .broadcast(Message::RequestVote {
                new_term: me.term,
                candidate: me.cluster.me().clone(),
                last_log_index: me.log.len() as u64 - 1,
                last_log_term: me.log.last().map_or(0, |entry| entry.term),
            })
            .await;

        todo!("Start election timer");

        me
    }

    pub async fn vote_received(
        mut self,
        follower: NodeId,
        term: u64,
        vote: bool,
    ) -> Either<Self, Node<Leader, C>> {
        if term == self.term && vote {
            self.state.votes.insert(follower.clone());

            if self.state.votes.len() >= self.cluster.quorum_size() {
                Either::Right(Node::<Leader, C>::promote(self).await)
            } else {
                Either::Left(self)
            }
        }
        // If the vote is for a different term, or the vote is false, do nothing
        else {
            Either::Left(self)
        }
    }
}

impl<C: Cluster> Node<Leader, C> {
    pub fn new_term(self, term: u64, leader: Option<NodeId>) -> Node<Follower, C> {
        Node {
            log: self.log,
            commit_index: self.commit_index,
            last_applied: self.last_applied,
            next_index: self.next_index,
            match_index: self.match_index,
            term,
            state: Follower {
                leader,
                voted_for: None,
            },
            cluster: self.cluster,
        }
    }

    pub async fn promote(before: Node<Candidate, C>) -> Self {
        let me = Node {
            log: before.log,
            commit_index: before.commit_index,
            last_applied: before.last_applied,
            next_index: before.next_index,
            match_index: before.match_index,
            term: before.term,
            state: Leader {},
            cluster: before.cluster,
        };

        me.cluster
            .broadcast(Message::AppendEntries {
                term: me.term,
                leader: me.cluster.me().clone(),
                prev_log_index: me.log.len() as u64 - 1,
                prev_log_term: me.log.last().map_or(0, |entry| entry.term),
                entries: vec![],
                leader_commit: me.commit_index,
            })
            .await;

        me
    }
}
