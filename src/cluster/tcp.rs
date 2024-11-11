// use std::collections::HashMap;
// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
// use std::sync::{Arc, Mutex};
// use std::thread;

// use super::{Cluster, NodeId};
// use crate::Message;

// pub struct TcpCluster {
//     nodes: Vec<NodeId>,
//     me: NodeId,
//     connections: Arc<Mutex<HashMap<NodeId, TcpStream>>>,
//     incoming_messages: Arc<Mutex<Vec<(NodeId, Message)>>>,
// }

// impl TcpCluster {
//     pub fn new(nodes: Vec<NodeId>, me: NodeId, port: u16) -> Self {
//         let cluster = TcpCluster {
//             nodes,
//             me,
//             connections: Arc::new(Mutex::new(HashMap::new())),
//             incoming_messages: Arc::new(Mutex::new(Vec::new())),
//         };

//         // Start listening for incoming connections
//         let listener = TcpListener::bind(("0.0.0.0", port)).expect("Failed to bind to port");
//         let incoming_messages = Arc::clone(&cluster.incoming_messages);

//         thread::spawn(move || {
//             for stream in listener.incoming() {
//                 let mut stream = stream.expect("Failed to accept connection");
//                 let incoming_messages = Arc::clone(&incoming_messages);

//                 thread::spawn(move || {
//                     loop {
//                         let mut buffer = [0; 1024];
//                         match stream.read(&mut buffer) {
//                             Ok(0) => break, // Connection closed
//                             Ok(n) => {
//                                 if let Ok(message) = serde_json::from_slice::<Message>(&buffer[..n])
//                                 {
//                                     let sender = String::from_utf8_lossy(&buffer[..n]).to_string();
//                                     incoming_messages.lock().unwrap().push((sender, message));
//                                 }
//                             }
//                             Err(_) => break,
//                         }
//                     }
//                 });
//             }
//         });

//         cluster
//     }

//     fn connect(&self, node: &NodeId) -> Result<TcpStream, std::io::Error> {
//         TcpStream::connect(node)
//     }
// }

// impl Cluster for TcpCluster {
//     fn nodes(&self) -> &Vec<NodeId> {
//         &self.nodes
//     }

//     fn me(&self) -> &NodeId {
//         &self.me
//     }

//     fn send_message(&self, to: &NodeId, message: Message) {
//         let mut connections = self.connections.lock().unwrap();
//         let stream = connections
//             .entry(to.clone())
//             .or_insert_with(|| self.connect(to).expect("Failed to connect"));

//         let serialized = serde_json::to_vec(&message).expect("Failed to serialize message");
//         stream
//             .write_all(&serialized)
//             .expect("Failed to send message");
//     }

//     fn pending_messages(&self) -> Vec<(NodeId, Message)> {
//         let mut messages = self.incoming_messages.lock().unwrap();
//         std::mem::take(&mut *messages)
//     }
// }
