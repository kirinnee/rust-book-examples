use std::fmt::{Debug, Display};
use std::io::stdin;
use std::rc::Rc;

use colour::*;

use guessing_game::Node;

#[allow(dead_code)]
fn ask_for_input<F, T>(question: &str, f: F) -> T where F: (Fn(&str) -> Result<T, String>) {
    loop {
        println!("{}", question);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let out = input.trim();
        match f(out) {
            Ok(e) => break e,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}

fn main() {
//    manual_building();
    auto_build();
}

fn auto_build() {
    // building a tree
    //
    //               8  h
    //               /  \
    //             /     \
    //           /        \
    //         /           \
    //       /              \
    //     6 f             7  g
    //      /\            / | \
    //     /  \         /  |   \
    //    /    \      /   |     \
    //  1 a   2 b   3 c  4 d     5 e
    let a = Node::new(1);
    let b = Node::new(2);
    let c = Node::new(3);
    let d = Node::new(4);
    let e = Node::new(5);
    let f = Node::new(6);
    let g = Node::new(7);
    let h = Node::new(8);


    h.add_child(&f);
    h.add_child(&g);

    f.add_child(&a);
    f.add_child(&b);

    g.add_child(&c);
    g.add_child(&d);
    g.add_child(&e);


    println!();
    print("a =", &a);
    print("b =", &b);
    print("c =", &c);
    print("d =", &d);
    print("e =", &e);
    print("f =", &f);
    print("g =", &g);
    print("h =", &h);


    println!();
    print("children of a =", a.children_val());
    print("children of b = ", b.children_val());
    print("children of c = ", c.children_val());
    print("children of d = ", d.children_val());
    print("children of e = ", e.children_val());
    print("children of f = ", f.children_val());
    print("children of g = ", g.children_val());
    print("children of h = ", h.children_val());


    println!();
    printd("parent of a = ", a.parent_val());
    printd("parent of b = ", b.parent_val());
    printd("parent of c = ", c.parent_val());
    printd("parent of d = ", d.parent_val());
    printd("parent of e = ", e.parent_val());
    printd("parent of f = ", f.parent_val());
    printd("parent of g = ", g.parent_val());
    printd("parent of h = ", h.parent_val());
}

fn manual_building() {
    // building a tree
    //
    //               8  h
    //               /  \
    //             /     \
    //           /        \
    //         /           \
    //       /              \
    //     6 f             7  g
    //      /\            / | \
    //     /  \         /  |   \
    //    /    \      /   |     \
    //  1 a   2 b   3 c  4 d     5 e
    let a = Node::new(1);
    let b = Node::new(2);
    let c = Node::new(3);
    let d = Node::new(4);
    let e = Node::new(5);
    let f = Node::new(6);
    let g = Node::new(7);
    let h = Node::new(8);

    {
        let mut f_child = f.children();
        f_child.push(Rc::clone(&a));
        *a.parent() = Rc::downgrade(&f);
        f_child.push(Rc::clone(&b));
        *b.parent() = Rc::downgrade(&f);

        let mut g_child = g.children();
        g_child.push(Rc::clone(&c));
        *c.parent() = Rc::downgrade(&g);
        g_child.push(Rc::clone(&d));
        *d.parent() = Rc::downgrade(&g);
        g_child.push(Rc::clone(&e));
        *e.parent() = Rc::downgrade(&g);

        let mut h_child = h.children();
        h_child.push(Rc::clone(&f));
        *f.parent() = Rc::downgrade(&h);
        h_child.push(Rc::clone(&g));
        *g.parent() = Rc::downgrade(&h);
    }

    println!();
    print("a =", &a);
    print("b =", &b);
    print("c =", &c);
    print("d =", &d);
    print("e =", &e);
    print("f =", &f);
    print("g =", &g);
    print("h =", &h);


    println!();
    print("children of a =", a.children_val());
    print("children of b = ", b.children_val());
    print("children of c = ", c.children_val());
    print("children of d = ", d.children_val());
    print("children of e = ", e.children_val());
    print("children of f = ", f.children_val());
    print("children of g = ", g.children_val());
    print("children of h = ", h.children_val());


    println!();
    printd("parent of a = ", a.parent_val());
    printd("parent of b = ", b.parent_val());
    printd("parent of c = ", c.parent_val());
    printd("parent of d = ", d.parent_val());
    printd("parent of e = ", e.parent_val());
    printd("parent of f = ", f.parent_val());
    printd("parent of g = ", g.parent_val());
    printd("parent of h = ", h.parent_val());
}

fn printd(target: &str, any: impl Debug) {
    dark_cyan_ln!(target);
    println!("{:?}", any)
}

fn print(target: &str, any: impl Display) {
    dark_cyan_ln!(target);
    println!("{}", any)
}





