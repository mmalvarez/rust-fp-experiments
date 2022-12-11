enum Bot {}

pub trait Set {
}

impl Set for Bot {
}

impl<A : Set, B : Set> Set for dyn Fn(A) -> B {

}

enum Pow<X : Set, Y : Set> {
    Pw(Box<dyn Fn(X) -> Y>)
} 

impl<X : Set, Y : Set> Set for Pow<X, Y> {

}

// need PhantomData?
/*enum U<X : Set + 'static, Y : Set + 'static> {
    Uu(Box<dyn Fn(fn(Pow<Pow<X, Y>, Y>) -> X) -> Pow<Pow<X, Y>, Y>>)
}
*/

enum U<X : Set, Y : Set> {
    Uu(Box<dyn FnOnce(fn(Pow<Pow<X, Y>, Y>) -> X) -> Pow<Pow<X, Y>, Y>>)
}

impl<X : Set, Y : Set> Set for U<X, Y> {

}

use crate::U::Uu;
use crate::Pow::Pw;

fn tau<X : Set + 'static, Y : Set + 'static> (t : Pow<Pow<U<X, Y>, Y>, Y>) -> U<X, Y> {
    let t = match t {Pw(t) => t};
    let result =
    //|f: Box<dyn Fn(Pow<Pow<X, X>, X>) -> X>| {
    move |f : fn(Pow<Pow<X, Y>, Y> ) -> X| {
        Pw(Box::new(move |p : Pow<X, Y>|
            t(Pw(Box::new(move |x : U<X, Y>| 
                match x { Uu(x) => 
                    match &p {Pw(p) => p (f (x(f)))}})))))
                //match x { Uu(x) => f(x(f))}))))
        
    };

    // Uu(Box::new(result))
    //Uu(Box::new(result))
    Uu(Box::new(result))
}

// this type signature is probably wrong.
// it should return U, not U<U>
fn sigma<X : Set + 'static, Y : Set + 'static> (s : U<U<X, Y>, Y>) -> Pow<Pow<U<X, Y>, Y>, Y> {
    match s {
        Uu(s) => s(tau)
    }
}


fn main() {
    println!("Hello, world!");
}
