trait Pilot {
    fn fly(&self);

    fn name(&self);

    fn family(&self);
}

trait Wizard {
    fn fly(&self) {
        println!("Abra!");
    }

    fn name(&self) {
        println!("Wizard");
    }

    fn family(&self) {
        println!("Harry Potter")
    }
}

struct Human;

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }

    fn name(&self) {
        println!("Pilot");
    }

    fn family(&self) {
        println!("FAM TIME");
    }
}

impl Human {
    fn fly(&self) {
        println!("I can fly...!");
    }

    fn name(&self) {
        println!("human");
    }
}



fn main() {
    let sheryl = Human;
    sheryl.fly();
    Pilot::fly(&sheryl);
    Wizard::fly(&sheryl);


    sheryl.name();
    Pilot::name(&sheryl);
    Wizard::name(&sheryl);

}



