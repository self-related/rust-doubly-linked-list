
use std::{cell::RefCell, fmt::Display, rc::Rc};

// store data and also links to other nodes to copy later
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Self>>>,
    prev: Option<Rc<RefCell<Self>>>
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::<RefCell<Self>>::new(RefCell::<Self>::new(Self {
            data,
            next: None,
            prev: None,
        }))
    }
}

// wrap Node<T> to prevent double ownership for Node<T>.
struct List<T> {
    current: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>
}


impl<T: Display> List<T> {

    fn new(data: T) -> Self {
        println!("\n[List created with value: {data}]"); // debug

        let new_list = Self {
            current: Some(Node::<T>::new(data)),
            next: None,
            prev: None,
        };
        
        new_list
    }

    fn insert(&mut self, data: T) {
        println!("\n[Inserting value: {data}]"); // debug

        if let Some(current_node) = self.current.take() {
            let mut temp = current_node.borrow_mut();
            temp.next = Some(
                Rc::<RefCell<Node<T>>>::new(RefCell::<Node<T>>::new(Node {
                    data,
                    next: self.next.clone(),
                    prev: Some(current_node.clone()),
                }))
            );

            self.prev = Some(current_node.clone());
            self.current = temp.next.clone();
        }
    }

    fn move_back(&mut self) {
        if let Some(prev) = self.prev.take() {
            self.prev = prev.borrow().prev.clone();
            self.next = self.current.clone();
            self.current = Some(prev.clone());

            println!("\n[<<<]");
        } else {
            self.next = self.current.clone();
            self.prev = None;
            self.current = None;
        }
    }

    fn move_forward(&mut self) {
        if let Some(next) = self.next.take() {
            self.prev = self.current.clone();
            self.current = Some(next.clone());
            self.next = next.borrow().next.clone();

            println!("\n[>>>]");
        }
    }

    // just print current, next and previous values
    fn debug(&self) {
        if let Some(prev_node) = &self.prev {
            println!("< Previous value: {}", prev_node.borrow().data);
        } else { println!("< Previous value: None"); }

        if let Some(current_node) = &self.current {
            println!("| Current value: {}", current_node.borrow().data);
        } else { println!("| Current value: None"); }
    
        if let Some(next_node) = &self.next {
            println!("> Next value: {}", next_node.borrow().data);
        } else { println!("> Next value: None"); }    
    }
}

fn main() {
    let mut list = List::<i32>::new(10000);

    list.insert(12345);
    list.debug();

    list.move_back();
    list.debug();

    list.insert(999);
    list.debug();

    list.move_forward();
    list.debug();
    
    println!("\n\n___[Move back till the end]");
    while let Some(_) = &list.prev {
        list.move_back();
        list.debug();
    }
    println!("\n\n___[Move forward till the end]");
    while let Some(_) = &list.next {
        list.move_forward();
        list.debug();
    }
}