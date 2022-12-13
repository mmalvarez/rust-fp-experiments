#[derive(Clone, Copy)]
enum Fix<'a, A : 'a> {
    RunFix(&'a dyn for<'b> Fn(Fix<'b, A>) -> &'b dyn Fn(A) -> A)
}

use crate::Fix::RunFix;

fn outs<'a, A : Clone> (f : Fix<'a, A>) ->
    &dyn for<'c> Fn(Fix<'c, A>) -> &'c (dyn Fn(A) -> A + 'c)  {
    match f {
        RunFix(g) => g
    }
}

fn eval_fix<'a, A : Clone> (f : Fix<'a, A>, a : A) -> A {
    let fx2 = f.clone();
    let f2 = outs(fx2);
    (f2(fx2))(a)
}
    
fn countdown(r : Fix<u32>, x : u32) -> u32 {
    match r {
        RunFix (r2) => {
             if x == 1 {1}
             else {(r2(RunFix(r2)))(x)}
        }
    }
}

fn main() {
    println!("Hello, world!");
}
