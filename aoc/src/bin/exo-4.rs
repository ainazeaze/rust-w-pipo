use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
struct Client {
    name: String,
    inbox: Vec<String>,
}

impl Client {
    fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: String::from(name),
            inbox: Vec::new(),
        }))
    }

    fn receive(self, message: &str) {
        println!("{message}");
    }
}

#[derive(Clone)]
struct ChatRoom {
    members: Vec<Rc<RefCell<Client>>>,
    inbox: Vec<String>,
}

impl ChatRoom {
    fn new() -> Self {
        Self {
            members: Vec::new(),
            inbox: Vec::new(),
        }
    }

    fn join(&mut self, client: Rc<RefCell<Client>>) {
        self.members.push(client);
    }
    fn send(mut self, message: &str) {
        for mut member in self.members {
            member.borrow_mut().inbox.push(String::from(message))
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

    room_a.send("Hello Room A");
    room_b.send("Hello Room B");
    room_c.send("Hello Room C");

    // print messages for each
}
