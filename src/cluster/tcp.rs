use std::{
    collections::HashMap,
    io,
    net::{TcpListener, TcpStream},
    thread::JoinHandle,
};

use crate::Message;

use super::{Cluster, NodeId};

/// A simple TCP-based cluster implementation.
pub struct TcpCluster {
    nodes: Vec<NodeId>,
    me: NodeId,
    conns: HashMap<NodeId, TcpStream>,
    threads: Vec<JoinHandle<()>>,
}

impl TcpCluster {
    pub fn new(nodes: Vec<NodeId>, me: NodeId) -> Self {
        let conns = HashMap::with_capacity(nodes.len());
        let threads = Vec::with_capacity(nodes.len());

        Self {
            nodes,
            me,
            conns,
            threads,
        }
    }

    pub fn connect(&mut self, node: NodeId, ip: &str) -> io::Result<()> {
        let conn = TcpStream::connect(ip)?;
        self.conns.insert(node, conn);

        self.threads.push(std::thread::spawn(move || {
            // Read messages from the connection and pass them to the cluster
            todo!()
        }));

        Ok(())
    }

    pub fn disconnect(&mut self, node: &NodeId) {
        self.conns.remove(node);
    }

    /// Listen in the current thread for incoming connections.
    pub fn listen(&mut self, ip: &str) -> io::Result<()> {
        let listener = TcpListener::bind(ip)?;

        // Accept incoming connections and spawn a new thread to handle each one
        todo!();

        Ok(())
    }
}

impl Cluster for TcpCluster {
    fn send_message(&self, to: &NodeId, message: Message) {
        let conn = self.conns.get(to).expect("Connection not found");

        // Serialize the message and send it over the connection
        todo!("Serialize")
    }

    fn pending_messages(&self) -> Vec<(NodeId, Message)> {
        todo!()
    }

    fn nodes(&self) -> &Vec<NodeId> {
        &self.nodes
    }

    fn me(&self) -> &NodeId {
        &self.me
    }
}
