pub mod tcp;

use crate::Message;

pub type NodeId = String;

pub trait Cluster {
    fn nodes(&self) -> &Vec<NodeId>;
    fn me(&self) -> &NodeId;
    fn quorum_size(&self) -> usize {
        self.nodes().len() / 2 + 1
    }
    fn size(&self) -> usize {
        self.nodes().len()
    }

    fn send_message(&self, to: &NodeId, message: Message);
    fn broadcast(&self, message: Message) {
        for node in self.nodes() {
            self.send_message(node, message.clone());
        }
    }

    fn pending_messages(&self) -> Vec<(NodeId, Message)>;
}
