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
enum U<X : Set, Y : Set> {
    Uu(Box<dyn Fn(Box<dyn Fn(Pow<Pow<X, Y>, Y>) -> X>) -> Pow<Pow<X, Y>, Y>>)
}

impl<X : Set, Y : Set> Set for U<X, Y> {

}

use crate::U::Uu;
use crate::Pow::Pw;


fn tau<X : Set> (t : Pow<Pow<U<X, X>, X>, X>) -> U<X, X> {
    let t = match t {Pw(t) => t};
    let result =
    |f: Box<dyn Fn(Pow<Pow<X, X>, X>) -> X> | {
        Pw(Box::new(|p : Pow<X, X>|
            t(Pw(Box::new(|x : U<X, X>| 
                match x { Uu(x) => 
                    match p {Pw(p) => p (f (x(f)))}})))))
                //match x { Uu(x) => f(x(f))}))))
        
    };

    // Uu(Box::new(result))
    //Uu(Box::new(result))
    Uu(Box::new(result))
}


/* 
fn tau<X : Set> (t : Pow<Pow<U<X, X>, X>, X>) -> U<X, X> {
    let t = match t {Pw(t) => t};
    let result =
    |f: Box<dyn Fn(Pow<Pow<X, X>, X>) -> X> | {
        Pw(Box::new(|p : Pow<X, X> |
            t(Pw(Box::new(|x : U<X, X>| 
                //match x { Uu(x) => p (f (x(f)))}))
                match x { Uu(x) => f(x(f))})))))
        
    };

    // Uu(Box::new(result))
    //Uu(Box::new(result))
    Uu(Box::new(result))
}
*/


fn main() {
    println!("Hello, world!");
}
