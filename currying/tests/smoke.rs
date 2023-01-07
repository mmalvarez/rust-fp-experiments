use currying::curry;
use currying::capp;
use std::rc::Rc;

#[curry(Rc, Orig("o_"))]
fn add(x : u32, y : u32, z : u32) -> u32 {
    x + y + z
}

#[derive(Clone)]
struct Hello { 
    hi : u32
}

impl Hello {
    //#[currying::curry]
    fn new(x : u32) -> Self {
        Hello {hi : x}
    }
    fn get(&self) -> u32 {
        self.hi
    }

    #[curry(Rc("b_"), Rc("", no_bundle))]
    fn add_in(&self, x : u32) -> u32 {
        x + self.hi
    }

}

#[test]
fn works(){
    let r1 = o_add(1, 2, 3);
    let r2 = add(1)(2)(3);
    assert!(r1 == r2);
}

#[test]
fn bundle() {
    let hello = Hello::new(2);
    let r1 = hello.add_in()(1);
    let r2 = hello.b_add_in(1);
    assert_eq!(r1, r2);
}

#[test]
fn do_capp() {
    let r1 = capp!(add 1 2 3);
    assert_eq!(r1, 6);
}