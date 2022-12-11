
/*
trait Cfn<A, B>: Fn(A) -> B + Clone {}
*/

use crate::Fix::RunFix;

#[derive(Copy, Clone)]
enum Fix<'l1, A : Clone> 
{
    RunFix(&'l1 dyn Fn(for<'a> Fix<A>) -> Box<dyn  Fn(A) -> A>)
}

trait MyFn<'a> {}
impl<'a, F> MyFn<'a> for F where
    F: FnOnce(&'a i32) -> &'a i32 {}


fn un_fix (fx : Fix) -> &'s dyn Fn(Fix<A>) -> Box< dyn Fn(A) -> A> {
    match self {
        RunFix(g) => g
    }
}
/*
impl<'s, A : Clone> Fix<'s, A> {

    fn un_fix (&'s self) -> &'s dyn Fn(Fix<A>) -> Box< dyn Fn(A) -> A> {
        match self {
            RunFix(g) => g
        }
    }

    fn eval_fix(&'s self, a : A) -> A {
        let cln = self.clone();
        let f2 = self.un_fix();
        (f2(cln))(a)
    }

}
*/
/*
fn eval_fix< A : 'static + Clone> 
    (f : &dyn Fix<A>, a : A) -> A {

    let f2 = f.run_fix();
    ((f2) (f))(a)
}
*/

fn countdown
    (f : Fix<u32>, a : u32) -> u32 {
        if a == 1 {1}
        else {f.eval_fix(a-1)}
}

fn countdown2<'f>
    (f : Fix<'f, u32>) -> Box<dyn Fn(u32) -> u32 + 'f> {
        Box::new(move |a : u32| (
            if a == 1 {1}
            else {f.eval_fix(a-1)}
        ))
}


/*
fn countdown<F0, F>(r : Fix<u32, F0, F>, x : u32) -> u32 {
    match r.clone() {
        RunFix (r2) => {
             if x == 1 {1}
             else {eval_fix(r, x-1)}
        }
    }
}
*/
/*
fn countdown2<F0 : Fn(u32) -> u32 + Copy, F : Fn(Fix<u32, F0, F>) -> F0 + Copy>  
    (f : Fix<u32, F0, F>) -> Box<dyn Fn(u32) -> u32> {
    let f2 = f.clone();
    let w = |x : u32| (
        match f2 {
            RunFix (_, _, f3) => {
                if x == 1 {1}
                else {eval_fix(f3.clone(), x-1)}
            }
        }
    );
    Box::new(w)
}
*/


fn main() {

    let countdown2 :
        Box<dyn Fn(Fix<'static, u32>) -> Box<(dyn Fn(u32) -> u32 + 'static)>>
    = Box::new(|f : Fix<u32>| (
        Box::new(move |a : u32| (
            if a == 1 {1}
            else {f.eval_fix(a-1)}
        ))));
    //let z = countdown(RunFix(10, countdown));

    let ctd2 = RunFix(&countdown2);

    let z = ctd2.eval_fix(10);
    //println!("We made it, we got {z}");
}

