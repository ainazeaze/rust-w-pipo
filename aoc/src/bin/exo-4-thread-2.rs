use std::{
    sync::mpsc::{Sender, channel},
    thread,
};

struct Client {
    sender: Sender<String>,
}

impl Client {
    fn new(name: &str) -> (Self, thread::JoinHandle<()>) {
        let (sender, receiver) = channel::<String>();
        let print_name = String::from(name);
        let handle = thread::spawn(move || {
            for message in receiver {
                println!("{print_name} : {message}");
            }
        });
        (Self { sender }, handle)
    }
}

#[derive(Clone)]
struct ChatRoom {
    channels: Vec<Sender<String>>,
}

impl ChatRoom {
    fn new() -> Self {
        Self {
            channels: Vec::new(),
        }
    }

    fn join(&mut self, client: &Client) {
        self.channels.push(client.sender.clone());
    }

    fn send(self, message: &str) {
        for sender in self.channels {
            sender.send(String::from(message)).unwrap();
        }
    }
}

fn main() {
    let (alice, alice_handle) = Client::new("Alice");
    let (bob, bob_handle) = Client::new("Bob");
    let (charlie, charlie_handle) = Client::new("Charlie");

    let mut room_a = ChatRoom::new();
    room_a.join(&alice);
    room_a.join(&bob);

    let mut room_b = ChatRoom::new();
    room_b.join(&bob);
    room_b.join(&charlie);

    let mut room_c = ChatRoom::new();
    room_c.join(&alice);
    room_c.join(&charlie);

    let t1 = thread::spawn(|| room_a.send("Hello Room A"));
    let t2 = thread::spawn(|| room_b.send("Hello Room B"));
    let t3 = thread::spawn(|| room_c.send("Hello Room C"));

    let _ = t1.join();
    let _ = t2.join();
    let _ = t3.join();

    // dropping the Arc clients releases the last senders, closing the receivers
    // which ends the for-loop in each print thread
    drop(alice);
    drop(bob);
    drop(charlie);

    let _ = alice_handle.join();
    let _ = bob_handle.join();
    let _ = charlie_handle.join();
}
