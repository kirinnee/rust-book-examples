use std::cell::{RefCell, RefMut};
use std::fmt::{Debug, Display, Error, Formatter};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node<T> {
    pub value: RefCell<T>,
    pub parent: RefCell<Weak<Node<T>>>,
    pub children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = if self.children.borrow().len() > 0 {
            String::from("[    \n") + &(self.children
                .borrow()
                .iter()
                .map(|x| {
                    let y: Vec<String> = (*x).to_string()
                        .split('\n')
                        .map(|x| "        ".to_string() + x).collect();
                    y.join("\n")
                }
                )
                .collect::<Vec<String>>()
                .join(",\n")) + "\n    ]\n}"
        } else {
            "[] }".to_string()
        };

        write!(f, "{{ {}, {}", *self.value.borrow(), s)
    }
}

impl<T: Display> Node<T> {
    pub fn new(value: T) -> Rc<Node<T>> {
        Rc::new(Node {
            value: RefCell::new(value),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    pub fn add_parent(self: &Rc<Self>, a: &Rc<Node<T>>) {
        *self.parent() = Rc::downgrade(a);
        a.children().push(Rc::clone(self));
    }

    pub fn add_child(self: &Rc<Self>, a: &Rc<Node<T>>) {
        (*self.children()).push(Rc::clone(a));
        *a.parent() = Rc::downgrade(self);
    }

    pub fn children_val(&self) -> String {
        self.children.borrow()
            .iter()
            .map(|x| (*(**x).value.borrow()).to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn parent_val(&self) -> Option<String> {
        let parent = self.parent.borrow_mut().upgrade()?;
        let temp = Rc::clone(&parent);
        let val = &temp.value.borrow().to_string();
        Some(val.to_string())
    }

    pub fn children(&self) -> RefMut<Vec<Rc<Node<T>>>> {
        self.children.borrow_mut()
    }

    pub fn parent(&self) -> RefMut<Weak<Node<T>>> {
        self.parent.borrow_mut()
    }

    pub fn value(&self) -> RefMut<T> {
        self.value.borrow_mut()
    }
}
