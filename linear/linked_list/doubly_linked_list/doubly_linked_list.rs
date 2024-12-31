use std::ptr;

// Define a Node struct
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    prev: *mut Node<T>,
}

// Define the DoublyLinkedList struct
pub struct DoublyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
}

impl<T> DoublyLinkedList<T> {
    // Create a new empty doubly linked list
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    // Add an element to the end of the list
    pub fn insert_last(&mut self, value: T) {
        let mut new_node = Box::new(Node {
            value,
            next: None,
            prev: ptr::null_mut(),
        });
        let raw_node: *mut _ = &mut *new_node;

        if self.tail.is_null() {
            self.head = Some(new_node);
            self.tail = raw_node;
        } else {
            unsafe {
                (*self.tail).next = Some(new_node);
                (*self.tail).next.as_mut().unwrap().prev = self.tail;
            }
            self.tail = raw_node;
        }
    }

    // Add an element to the front of the list
    pub fn insert_first(&mut self, value: T) {
        let mut new_node = Box::new(Node {
            value,
            next: self.head.take(),
            prev: ptr::null_mut(),
        });

        if let Some(ref mut old_head) = new_node.next {
            old_head.prev = &mut *new_node;
        } else {
            self.tail = &mut *new_node;
        }
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
                let mut new_node = Box::new(Node {
                    value,
                    next: node.next.take(),
                    prev: &mut **node,
                });
                if let Some(ref mut next_node) = new_node.next {
                    next_node.prev = &mut *new_node;
                } else {
                    self.tail = &mut *new_node;
                }
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
            if let Some(ref mut next_node) = node.next {
                if &next_node.value == key {
                    let mut new_node = Box::new(Node {
                        value,
                        next: node.next.take(),
                        prev: &mut **node,
                    });
                    new_node.next.as_mut().unwrap().prev = &mut *new_node;
                    node.next = Some(new_node);
                    return true;
                }
            }
            current = node.next.as_mut();
        }
        false
    }

    // Delete a specific value from the list
    pub fn delete(&mut self, key: &T) -> bool
    where
        T: PartialEq,
    {
        let mut current = self.head.as_mut();
        while let Some(node) = current {
            if &node.value == key {
                let next = node.next.take();
                if let Some(ref mut prev) = unsafe { node.prev.as_mut() } {
                    prev.next = next;
                } else {
                    self.head = next;
                }
                if let Some(next_node) = node.next.as_mut() {
                    next_node.prev = node.prev;
                } else {
                    self.tail = node.prev;
                }
                return true;
            }
            current = node.next.as_mut();
        }
        false
    }

    // Delete the first element
    pub fn delete_first(&mut self) -> bool {
        if let Some(mut head) = self.head.take() {
            self.head = head.next.take();
            if let Some(ref mut new_head) = self.head {
                new_head.prev = ptr::null_mut();
            } else {
                self.tail = ptr::null_mut();
            }
            return true;
        }
        false
    }

    // Delete the last element
    pub fn delete_last(&mut self) -> bool {
        if self.tail.is_null() {
            return false;
        }
        unsafe {
            if self.tail == self.head.as_deref_mut().map(|node| &mut **node).unwrap_or(ptr::null_mut()) {
                self.head = None;
                self.tail = ptr::null_mut();
            } else {
                let prev = (*self.tail).prev;
                (*prev).next = None;
                self.tail = prev;
            }
        }
        true
    }

    // Print the list
    pub fn print_list(&self)
    where
        T: std::fmt::Debug,
    {
        let mut current = self.head.as_deref();
        if current.is_none() {
            println!("Empty list");
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
    let mut dll = DoublyLinkedList::new();

    dll.insert_last(1);
    dll.insert_last(2);
    dll.insert_last(3);
    dll.insert_last(4);

    dll.print_list();

    dll.insert_first(0);
    dll.insert_first(-1);
    dll.insert_first(-2);

    dll.print_list();

    println!("Search 0: {}", dll.search(&0));

    dll.insert_after(&0, 0.5);

    dll.print_list();

    dll.insert_before(&0, -0.5);

    dll.print_list();

    dll.delete(&0);

    dll.print_list();

    dll.delete_first();

    dll.print_list();

    dll.delete_last();

    dll.print_list();

    dll.delete_last();

    dll.print_list();
}
