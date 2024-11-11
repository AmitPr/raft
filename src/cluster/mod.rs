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

    fn send_message(
        &self,
        to: &NodeId,
        message: Message,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn broadcast(&self, message: Message) -> impl std::future::Future<Output = ()> + Send {
        let futs = self
            .nodes()
            .iter()
            .map(|node| self.send_message(node, message.clone()))
            .collect::<Vec<_>>();

        async {
            futures::future::join_all(futs).await;
        }
    }

    fn poll_inbox(&self) -> impl std::future::Future<Output = Option<(NodeId, Message)>> + Send;
}
