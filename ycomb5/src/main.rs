use std::rc::Rc;

#[derive(Clone)]
enum Fix<A> {
    RunFix(Rc<dyn Fn(Fix<A>) -> Rc <dyn Fn(A) -> A>>)
}

use crate::Fix::RunFix;

fn outs<A : Clone> (f : Fix<A>) ->
    Rc<dyn Fn(Fix<A>) -> Rc<dyn Fn(A) -> A>>  {
    match f {
        RunFix(g) => g
    }
}

fn eval_fix<'a, A : Clone> (f : Fix<A>, a : A) -> A {
    let fx2 = f.clone();
    let gx2 = f.clone();
    let f2 = outs(fx2);
    (f2(gx2))(a)
}
    
fn countdown(r : Fix<u32>, x : u32) -> u32 {
    if x == 1 {1}
    else { eval_fix(r, x - 1)}
}

fn countdown2 (f : Fix<u32>) -> Rc <dyn Fn(u32) -> u32> {
    let body = Rc::new(move |x : u32| {
        countdown(f.clone(), x)
    });
    body
}

#[derive(Clone)]
enum Rfix<'a, A : 'a> {
    RunRfix(&'a dyn for<'b> Fn(Rfix<'b, A>) -> &'b dyn Fn(A) -> A)
}

use crate::Rfix::RunRfix;

fn r_outs<'a, A : Clone> (f : Rfix<'a, A>) ->
    &dyn for<'c> Fn(Rfix<'c, A>) -> &'c (dyn Fn(A) -> A + 'c)  {
    match f {
        RunRfix(g) => g
    }
}

fn r_eval_fix<'a, A : Clone> (f : Rfix<'a, A>, a : A) -> A {
    let fx2 = f.clone();
    let gx2 = f.clone();
    let f2 = r_outs(fx2);
    (f2(gx2))(a)
}
    
fn r_countdown(r : Rfix<u32>, x : u32) -> u32 {
    match r {
        RunRfix (r2) => {
             if x == 1 {1}
             else {(r2(RunRfix(r2)))(x)}
        }
    }
}

fn r_countdown2<'a> (f : Rfix<'a,u32>) -> &'a dyn Fn(u32) -> u32  {
    let body = move |x : u32| {
        r_countdown(f.clone(), x)
    };
    &body
}



fn main() {
    let ctd = RunFix(Rc::new(countdown2));
    let z = eval_fix(ctd, 10);
    println!("We made it; we got {z}");


    let body : &dyn Fn (Rfix<u32>) -> Box<dyn Fn(u32) -> u32> = 
    &move |f : Rfix<u32> | {
        let w : Box<dyn Fn(u32) -> u32> =
            Box::new(move |x : u32| {
                r_countdown(f.clone(), x)
            });
        w
    };
    let r_ctd = RunRfix(&body);
    let r_z = r_eval_fix(r_ctd, 10);
    println!("We made it; we got {r_z}");
}
