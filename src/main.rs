use std::cell::RefCell;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;
use std::rc::Rc;

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

impl List {
    fn nil() -> Rc<List> {
        Rc::new(Nil)
    }
    fn new(i: &Rc<RefCell<i32>>, next: &Rc<List>) -> Rc<List> {
        Rc::new(Cons(Rc::clone(i), Rc::clone(next)))
    }
    fn num(i: i32) -> Rc<RefCell<i32>> {
        Rc::new(RefCell::new(i))
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut current = self;
        let mut s = vec![];
        loop {
            match current {
                Cons(value, next) => {
                    current = next;
                    let i = *value.deref().borrow();
                    s.push(i.to_string())
                }
                Nil => {
                    break write!(f, "{{{}}}", s.join(","));
                }
            }
        }
    }
}

fn main() {
    let value = List::num(1);
    let a = List::new(&value, &List::nil());
    let b = List::new(&List::num(2), &a);
    println!("created b, count {}", Rc::strong_count(&b));
    let c = List::new(&List::num(3), &b);
    println!("created c, count {}", Rc::strong_count(&b));
    let d = List::new(&List::num(4), &b);
    println!("created d, count {}", Rc::strong_count(&b));

    {
        let e = List::new(&List::num(5), &b);
        println!("created e, count {}", Rc::strong_count(&b));
    }

    println!("deleted e, count {}", Rc::strong_count(&b));

    *value.borrow_mut() += 10;

    println!("d {}", d);
    println!("c {}", c);
}




