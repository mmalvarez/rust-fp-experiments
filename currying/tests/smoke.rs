#[currying::curry(Rc)]
fn add(x : u32, y : u32, z : u32) -> u32 {
    x + y + z
}

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
}

use std::rc::Rc;

#[test]
fn works(){
    let r1 = add(1, 2, 3);
    let r2 = c_add(1)(2)(3);
    assert!(r1 == r2);
}