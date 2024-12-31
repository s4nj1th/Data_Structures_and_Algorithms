use std::mem;

// Define a Node struct
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

// Define the LinkedList struct
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Add an element to the end of the list
    pub fn insert_last(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });
        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut current) => {
                while let Some(next) = current.next.as_mut() {
                    current = next;
                }
                current.next = Some(new_node);
            }
        }
    }

    // Add an element to the front of the list
    pub fn insert_first(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Search for an element in the list
    pub fn search(&self, key: &T) -> bool
    where
        T: PartialEq,
    {
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            if &node.value == key {
                return true;
            }
            current = node.next.as_deref();
        }
        false
    }

    // Insert a value after the first occurrence of key
    pub fn insert_after(&mut self, key: &T, value: T) -> bool
    where
        T: PartialEq,
    {
        let mut current = self.head.as_mut();
        while let Some(node) = current {
            if &node.value == key {
                let new_node = Box::new(Node {
                    value,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                return true;
            }
            current = node.next.as_mut();
        }
        false
    }

    // Insert a value before the first occurrence of key
    pub fn insert_before(&mut self, key: &T, value: T) -> bool
    where
        T: PartialEq,
    {
        if let Some(node) = self.head.as_mut() {
            if &node.value == key {
                self.insert_first(value);
                return true;
            }
        }

        let mut current = self.head.as_mut();
        while let Some(node) = current {
            if let Some(next) = node.next.as_mut() {
                if &next.value == key {
                    let new_node = Box::new(Node {
                        value,
                        next: node.next.take(),
                    });
                    node.next = Some(new_node);
                    return true;
                }
            }
            current = node.next.as_mut();
        }
        false
    }

    // Delete the nth occurrence of a value
    pub fn delete_nth_occurrence(&mut self, key: &T, n: usize) -> bool
    where
        T: PartialEq,
    {
        let mut current = self.head.as_mut();
        let mut count = 0;
        let mut prev: Option<&mut Box<Node<T>>> = None;

        while let Some(node) = current {
            if &node.value == key {
                count += 1;
                if count == n {
                    match prev {
                        None => self.head = node.next.take(),
                        Some(prev_node) => prev_node.next = node.next.take(),
                    }
                    return true;
                }
            }
            prev = current;
            current = node.next.as_mut();
        }
        false
    }

    // Delete the first element
    pub fn delete_first(&mut self) -> bool {
        if self.head.is_none() {
            return false;
        }
        self.head = self.head.take().and_then(|node| node.next);
        true
    }

    // Delete the last element
    pub fn delete_last(&mut self) -> bool {
        match self.head.as_mut() {
            None => false,
            Some(node) if node.next.is_none() => {
                self.head = None;
                true
            }
            Some(mut current) => {
                while let Some(next) = current.next.as_mut() {
                    if next.next.is_none() {
                        current.next = None;
                        return true;
                    }
                    current = next;
                }
                false
            }
        }
    }

    // Delete all occurrences of a value
    pub fn delete_all_occurrences(&mut self, key: &T) -> bool
    where
        T: PartialEq,
    {
        let mut modified = false;
        while let Some(node) = self.head.as_ref() {
            if &node.value == key {
                self.head = self.head.take().and_then(|node| node.next);
                modified = true;
            } else {
                break;
            }
        }

        let mut current = self.head.as_mut();
        while let Some(node) = current {
            while let Some(next) = node.next.as_mut() {
                if &next.value == key {
                    node.next = next.next.take();
                    modified = true;
                } else {
                    break;
                }
            }
            current = node.next.as_mut();
        }
        modified
    }

    // Print the list
    pub fn print_list(&self)
    where
        T: std::fmt::Debug,
    {
        let mut current = self.head.as_deref();
        if current.is_none() {
            println!("List is empty");
            return;
        }
        while let Some(node) = current {
            print!("{:?} -> ", node.value);
            current = node.next.as_deref();
        }
        println!();
    }
}

// Example usage
fn main() {
    let mut sll = LinkedList::new();

    sll.insert_last(1);
    sll.insert_last(2);
    sll.insert_last(3);
    sll.insert_last(4);
    sll.insert_last(5);
    sll.insert_last(6);
    sll.insert_last(7);

    sll.insert_first(0);
    sll.insert_first(0);
    sll.insert_first(0);

    sll.print_list();

    println!("Search 0: {}", sll.search(&0));
    println!("Search 1: {}", sll.search(&1));
    println!("Search 2: {}", sll.search(&2));
    println!("Search 3: {}", sll.search(&3));
    println!("Search 4: {}", sll.search(&4));

    println!("Delete nth occurrence 0: {}", sll.delete_nth_occurrence(&0, 1));
    println!("Delete nth occurrence 0: {}", sll.delete_nth_occurrence(&0, 1));
    println!("Delete nth occurrence 0: {}", sll.delete_nth_occurrence(&0, 1));

    sll.print_list();

    println!("Delete first: {}", sll.delete_first());
    println!("Delete first: {}", sll.delete_first());
    println!("Delete first: {}", sll.delete_first());

    sll.print_list();

    println!("Delete last: {}", sll.delete_last());
    println!("Delete last: {}", sll.delete_last());
    println!("Delete last: {}", sll.delete_last());

    sll.print_list();

    println!("Delete all occurrences of 1: {}", sll.delete_all_occurrences(&1));

    sll.print_list();
}
