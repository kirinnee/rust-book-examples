use std::cell::RefCell;
use std::fmt::{Display, Error, Formatter};
use std::rc::{Rc, Weak};

fn main() {
    ref_cell();
    tree();
}

fn ref_cell() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    print_ref("leaf", &leaf);

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);


        println!("leaf parent = {:?}", leaf.parent.borrow());
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        print_ref("leaf", &leaf);
        print_ref("branch", &branch);
    }

    println!("leaf parent = {:?}", leaf.parent.borrow());
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    print_ref("leaf", &leaf);
}

fn print_ref(s: &str, r: &Rc<Node>) {
    println!("{} strong = {}, weak = {}", s, Rc::strong_count(r), Rc::weak_count(r))
}

#[derive(Debug)]
struct RefPoint<'a, T> {
    value: &'a mut T,
    children: &'a mut Vec<&'a RefPoint<'a, T>>,
}

impl<'a, T> Display for RefPoint<'a, T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = self.children.iter().map(|x| (**x).to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "{{value: {}, children: [ {} ] }}", self.value, s)
    }
}


fn tree() {
    let a = RefPoint { value: &mut 5, children: &mut vec![] };
    {
        let b = RefPoint { value: &mut 6, children: &mut vec![] };
        {
            {
                let c = RefPoint { value: &mut 7, children: &mut vec![&a, &b] };
                println!("a = {}", &a);
                println!("b = {}", &b);
                println!("c = {}", &c);
            }
        }
    }
}


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

