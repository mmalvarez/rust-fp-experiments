enum Bot {}

pub trait Set {
    type S;
    fn inj(&self) -> S;
}

impl Set for Bot {
}

/*
impl<X : Set + ?Sized> Set for Box<X> {

}

pub trait Pow<X: Set + ?Sized, Y: Set + ?Sized> {
    fn pw(&self) -> Box<dyn Fn(X) -> Y>;
}

impl<X: Set + ?Sized, Y: Set + ?Sized> Set for dyn Pow<X, Y> {

}


use std::marker::PhantomData;
pub enum U<X:Set + ?Sized, Y: Set + ?Sized> {
    Uu(Box<dyn Fn(Box<dyn Fn(Box<dyn Pow<Box<dyn Pow<X, Y>>, Y>>) -> X>) -> Box<dyn Pow<Box<dyn Pow<X, Y>>, Y>>>)
    //Uu(Box<dyn Fn(Box<dyn Fn(Box<dyn Pow<Box<dyn Pow<X, Y>>, Y>) -> X>) -> Box<dyn Pow<Box<dyn Pow<X, Y>>, Y>>>);
}


impl<X: Set, Y: Set> Set for U<X, Y> {

}

fn tao<X:Set, Y:Set> (t : Box<dyn Pow<Box<dyn Pow<Box<U<X, Y>>, Y>>, Y>>) -> Box< U<X, Y>> {
    // t(\lam x . p (f (x X f)))

    
    let result =
    |f: Box<dyn Fn(Box<dyn Pow<Box<dyn Pow<X, Y>>, Y>>) -> X> |
        (|p : Box<dyn Fn(Box<dyn Pow<X, Y>>) -> Y>|
            t.pw()(|x : Box<dyn Fn(Box<dyn Fn(Box<dyn Pow<Box<dyn Pow<X, Y>>, Y>>) -> X>) -> Box<dyn Pow<Box<dyn Pow<X, Y>>, Y>>>|
                p(f(x(f)))
            //t.pw()(|x|
            //    p(f(x(f)))
            )
        );

    Box::new(result)
}
*/

/*
impl Set for dyn Fn(X) -> Y {

}
*/

/*pub enum Pow<X : Set, Y : Set> {
    Pw ()
}
*/

fn main() {
    println!("Hello, world!");
}
