enum Bot {}

pub trait Set {
    type S : Set;
}

impl<A, B: Set + 'static> Set for dyn Fn(A) -> B {
    type S = B::S;
}

impl<A, B: Set + 'static> Set for fn(A) -> B {
    type S = B::S;
}

enum Pow<X : Set + 'static> {
    Pw(Box<dyn Fn(X) -> <X as Set>::S>)
} 

impl<X : Set> Set for Pow<X> {
    type S = <X as Set>::S;
}


enum U<X : Set + 'static> {
    Uu(Box<dyn FnOnce(fn(Pow<Pow<X>>) -> X) -> Pow<Pow<X>>>)
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

fn tau<X : Set> (t : Pow<Pow<U<X>>>) -> U<X> {
    let t = match t {Pw(t) => t};
    let result =
        move |f : fn(Pow<Pow<X>> ) -> X| {
            Pw(Box::new(move |p : Pow<X>|
                t(Pw(Box::new(move |x : U<X>| 
                    match x { Uu(x) => 
                        match &p {Pw(p) => p (f (x(f)))}})))))
                    //match x { Uu(x) => f(x(f))}))))
            
        };

    // Uu(Box::new(result))
    //Uu(Box::new(result))
    Uu(Box::new(result))
}


fn sigma<X : Set> (s : U<U<X>>) -> Pow<Pow<U<X>>> {
    match s {
        Uu(s) => s(tau)
    }
}


fn main() {
    println!("Hello, world!");
}
