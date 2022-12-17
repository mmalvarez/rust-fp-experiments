/*trait Fix<A, F>
where F : for<'a> Fn(dyn Fix<A, F>) -> Box<dyn Fn(A) -> A> + 'a
{
    fn get(self) -> &dyn Fn(dyn Fix<A, F>) -> Box<dyn Fn(A) -> A>;
}
*/

use crate::Rec::In;
#[derive(Clone, Copy)]
enum Rec<'a, A : 'a> {
    In(&'a dyn for<'b> Fn(Rec<'b, A>) -> A)
}

fn out<'a : 'b, 'b, A : Clone> (f : Rec<'a, A>) -> & 'b dyn Fn(Rec<A>) -> A {
    match f {
        In(g) => g
    }
}

use crate::Recs::Ins;

#[derive(Clone, Copy)]
enum Recs<'a, A : 'a> {
    Ins(&'a dyn for<'b> Fn(Recs<'b, A>) -> &'b dyn Fn(A) -> A)
}

fn outs<'a, A : Clone> (f : Recs<'a, A>) ->
    &dyn for<'c> Fn(Recs<'c, A>) -> &'c (dyn Fn(A) -> A + 'c)  {
    match f {
        Ins(g) => g
    }
}

use crate::Recs2::Ins2;
#[derive(Clone, Copy)]
enum Recs2<'a, A : 'a> {
    Ins2(&'a dyn for<'b> Fn(Recs2<'b, A>) -> Box<dyn Fn(A) -> A + 'b>)
}

fn outs2<'a, A : Clone> (f : Recs2<'a, A>) ->
    &dyn for<'b> Fn(Recs2<'b, A>) -> Box<(dyn Fn(A) -> A + 'b)> {
    match f {
        Ins2(g) => g
    }
}


fn yc<'a, A : Clone + 'a + Copy> (f : fn(A) -> A) -> A {
    let f2 = f.clone();
    let f3 = f.clone();
    let r1 = Box::new(|x | (f2 ((out(x))(x))));
    let r2 : Box<dyn Fn(Rec<A>) -> A> = Box::new(|x | (f3 ((out(x))(x))));
    let r3 = In(&r2);
    r1(r3)
}
/*
fn zc<'a, A : Clone + 'a + Copy + 'static> (f :fn(A) -> A, v : A) -> A {
    let f2 = f.clone();
    let f3 = f.clone();
    //let r1 = Box::new(|x | (f2 ((outs(x))(x))));
    let r1 : Box<dyn Fn(Recs<A>) -> A> = Box::new(|x | f2 (outs(x)(x)(v)));
    let r2 : Box<dyn for<'c> Fn(Recs<'c, A>) -> Box<(dyn Fn(A) -> A)>> =
        Box::new(|x | Box::new(|w| f3 (outs(x)(x)(w))));
    let r3 : Box<dyn for<'c> Fn(Recs<'c, A>) -> &(dyn Fn(A) -> A)> =
        Box::new(|foo| &(r2(foo)));
    let r4 = Ins(&r3);
    r1(r4)
}
*/
/*
fn zc<'a, A : Clone + 'a + Copy + 'static> (f :fn(A) -> A, v : A) -> A {
    let f2 = f.clone();
    let f3 = f.clone();
    let r1 : Box<dyn Fn(Recs<A>) -> A> = Box::new(|x | f2 (outs(x)(x)(v)));
    let r2 : Box<dyn for<'c> Fn(Recs<'c, A>) -> Box<(dyn Fn(A) -> A + 'c)>> =
        Box::new(|x | Box::new(move |w| f3 (outs(x)(x)(w))));
    let r3 : Box<dyn for<'c> Fn(Recs<'c, A>) -> &(dyn Fn(A) -> A)> =
        Box::new(move |foo| &(r2(foo)));
    let r4 = Ins(&r3);
    r1(r4)
}
*/


fn zc2<'a, A : Clone + 'a + Copy + 'static> (f :fn(A) -> A, v : A) -> A {
    let f2 = f.clone();
    let f3 = f.clone();
    let r1 : Box<dyn Fn(Recs2<A>) -> A> = Box::new(|x | f2 (outs2(x)(x)(v)));
    let r2 : Box<dyn for<'c> Fn(Recs2<'c, A>) -> Box<(dyn Fn(A) -> A + 'c)>> =
        Box::new(|x | Box::new(move |w| f3 (outs2(x)(x)(w))));
    let r3 : Box<dyn for<'c> Fn(Recs2<'c, A>) -> Box<dyn Fn(A) -> A + 'c>> =
        Box::new(|foo| (r2(foo)));
    let r4 = Ins2(&r3);
    r1(r4)
}

fn countdown(x : u32) -> u32 {
    if x == 1 {1}
    else {x-1}
}


fn main() {
    let result : u32 = zc2(countdown, 1);
    println!("Hello, world! We got {result}");
}
