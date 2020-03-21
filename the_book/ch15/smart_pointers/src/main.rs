struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn list_v1() {
    println!("\nlist_v1()");
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);
}

fn list_v2() {
    println!("\nlist_v2()");
    use std::rc::Rc;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a: {:?}", a);
    println!("refc(a)={}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("b: {:?}", b);
    println!("refc(a)={}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("c: {:?}", a);
        println!("refc(a)={}", Rc::strong_count(&a));
    }
    println!("refc(a)={}", Rc::strong_count(&a));
}

fn list_v3() {
    println!("\nlist_v3()");
    use std::cell::RefCell;
    use std::rc::Rc;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn list_v4() {
    println!("\nlist_v4()");

    use std::cell::RefCell;
    use std::rc::Rc;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("refc(a)={}", Rc::strong_count(&a));
    println!("tail(a)={:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("refc(a)={}", Rc::strong_count(&a));
    println!("refc(b)={}", Rc::strong_count(&b));
    println!("tail(b)={:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("refc(a)={}", Rc::strong_count(&a));
    println!("refc(b)={}", Rc::strong_count(&b));

    // will overflow the stack
    //println!("tail(a)={:?}", a.tail());
}

fn tree() {
    println!("\ntree");

    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    // assert_eq!(5, y); can't compare `{integer}` with `Box<{integer}>`
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    //assert_eq!(5, y); can't compare `{integer}` with `MyBox<{integer}>`
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    let a = CustomSmartPointer {
        data: String::from("a"),
    };
    let b = CustomSmartPointer {
        data: String::from("b"),
    };
    let c = CustomSmartPointer {
        data: String::from("c"),
    };
    println!("CustomerSmartPointers created.");
    //b.drop(); // explicit use of destructor method
    drop(b); // ok

    list_v1();
    list_v2();
    list_v3();
    list_v4();
    tree();
}
