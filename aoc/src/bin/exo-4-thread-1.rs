use std::{
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone)]
struct Client {
    name: String,
    inbox: Vec<String>,
}

impl Client {
    fn new(name: &str) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            name: String::from(name),
            inbox: Vec::new(),
        }))
    }
    fn print_messages(&self) {
        for message in &self.inbox {
            println!("{} : {message}", &self.name)
        }
    }
}

#[derive(Clone)]
struct ChatRoom {
    members: Arc<Mutex<Vec<Arc<Mutex<Client>>>>>,
}

impl ChatRoom {
    fn new() -> Self {
        Self {
            members: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn join(&mut self, client: Arc<Mutex<Client>>) {
        self.members.lock().unwrap().push(client);
    }
    fn send(self, message: &str) {
        for member in self.members.lock().unwrap().to_vec() {
            member.lock().unwrap().inbox.push(String::from(message))
        }
    }
}

fn main() {
    let alice = Client::new("Alice");
    let bob = Client::new("Bob");
    let charlie = Client::new("Charlie");

    let mut room_a = ChatRoom::new();
    room_a.join(alice.clone());
    room_a.join(bob.clone());

    let mut room_b = ChatRoom::new();
    room_b.join(bob.clone());
    room_b.join(charlie.clone());

    let mut room_c = ChatRoom::new();
    room_c.join(alice.clone());
    room_c.join(charlie.clone());

    let t1 = thread::spawn(|| {
        room_a.send("Hello Room A");
    });
    let t2 = thread::spawn(|| {
        room_b.send("Hello Room B");
    });
    let t3 = thread::spawn(|| {
        room_c.send("Hello Room C");
    });

    let _ = t1.join();
    let _ = t2.join();
    let _ = t3.join();

    // print messages for each
    alice.lock().unwrap().print_messages();
    bob.lock().unwrap().print_messages();
    charlie.lock().unwrap().print_messages();
}
