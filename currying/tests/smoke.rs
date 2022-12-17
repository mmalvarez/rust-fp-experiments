#[currying::curry]
fn add(x : u32, y : u32, z : u32) -> u32 {
    x + y + z
}

use std::rc::Rc;

#[test]
fn works(){
    let r1 = add(1, 2, 3);
    let r2 = c_add(1)(2)(3);
    assert!(r1 == r2);
}