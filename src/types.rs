#[derive(Debug, Clone)]
pub struct LogEntry {
    pub term: u64,
    pub command: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    RequestVote {
        new_term: u64,
        candidate: String,
        last_log_index: u64,
        last_log_term: u64,
    },
    Vote {
        follower: String,
        new_term: u64,
        vote: bool,
    },
    AppendEntries {
        term: u64,
        leader: String,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<String>,
        leader_commit: u64,
    },
    AppendEntriesResponse {
        term: u64,
        success: bool,
        match_index: u64,
    },
    Timeout,
}
