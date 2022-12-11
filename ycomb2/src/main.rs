use crate::Fix::RunFix;

enum Fix<A> {
    RunFix(A, fn(Fix<A>) -> A)
}

/*
fn rec<A>(f : Fix<A>) -> A {
    match f {
        RunFix(f2, x, rec2) => rec2(RunFix(f2, f2(x), rec2))
    }
}

fn make_fix<A>(f : fn(A) -> A, v : A) -> Fix<A> {
    RunFix (f, v, rec)
}

fn do_fix<A>(f : fn(A) -> A, v : A) -> A {
    rec(make_fix(f, v))
}
*/

/* 
fn fix_init<A> (x : A) -> Fix<A> {

}
*/

fn countdown(r : Fix<u32> ) -> u32 {
    match r {
        RunFix (x, r2) => {
             if x == 1 {1}
             else {r2(RunFix(x-1, r2))}
        }
    }
}

fn eval_fix<A>(f : fn(Fix<A>) -> A, v : A) -> A {
    f(RunFix(v, f))
}

fn main() {
    println!("Hello, world!");
    //let z = countdown(RunFix(10, countdown));
    let z = eval_fix(countdown, 10);
    println!("We made it, we got {z}");
}
