enum Bot {}

pub trait Set {
    type S : Set;
}

impl<A, B: Set> Set for Rc<dyn Fn(A) -> B> {
    type S = B::S;
}

use std::rc::Rc;


#[derive(Clone)]
enum Pow<X : Set> {
    Pw(Rc<dyn Fn(X) -> <X as Set>::S>)
} 

/*
trait Tpow<X : Set> {
    fn do_pow(&self, x :X) -> X::S;
}
*/

/*
trait Tu<X : Set> {
    type PX : Tpow<X>;
    type PPX : Tpow<Self::PX>;
    type F1 : Fn(Self::PPX) -> X;

    fn do_u(&self, x : Self::F1) -> Self::PPX;
}
*/
impl<X : Set> Set for Pow<X> {
    type S = <X as Set>::S;
}




#[derive(Clone)]
enum U<X : Set + 'static> {
    Uu(Rc<dyn Fn(Rc<dyn Fn(Pow<Pow<X>>) -> X>) -> Pow<Pow<X>>>)
}

/*
enum U<X : Set + 'static> {
    Uu(Box< dyn FnOnce(<X as Set>:: S) -> Box<dyn FnOnce(fn(Pow<Pow<X>>) -> X) -> Pow<Pow<X>>>>)
}
*/

impl<X : Set> Set for U<X> {
    type S = <X as Set>::S;
}

use crate::U::Uu;
use crate::Pow::Pw;

fn tau<X : Set + Clone> (t : Pow<Pow<U<X>>>) -> U<X> {
    let t = match t.clone() {Pw(t) => t};
    let t = t.clone();
    let result =
        move |f : Rc<dyn Fn(Pow<Pow<X>>) -> X>| {
            let t = t.clone();
            Pw(Rc::new(move |p : Pow<X>| {
                let f = f.clone();
                t(Pw(Rc::new(move |x : U<X>|  {
                    let f = f.clone();
                    match x { 
                        Uu(x) => 
                            match p.clone() {
                                Pw(p) => p (f.clone() (x(f)))
                            }
                        }
                    })))
        }))
    };

    Uu(Rc::new(result))
}
/*
fn sigma<X : Set + Clone> (s : U<U<X>>) -> Pow<Pow<U<X>>> {
    match s {
        Uu(s) => s(Rc::new(tau))
    }
}
*/

fn sigma<X : Set + Clone> (s : U<U<X>>) -> Pow<Pow<U<X>>> {
    match s {
        Uu(s) => s(Rc::new(tau))
    }
}


fn delta<X : Set + Clone> (a : ()) -> Pow<U<X>> {
    let result = |f| {
        todo!()
    };
    Pw(Rc::new(result))
}

fn omega<X : Set + Clone> (a : ()) -> U<X> {
    let arg = |p| {
        todo!()
    };
    tau(Pw(Rc::new(arg)))
}

fn main() {
    println!("Hello, world!");
}

