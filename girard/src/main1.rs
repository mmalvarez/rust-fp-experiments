enum Bot {}

pub trait Set {
}

impl Set for Bot {

}

pub trait Pow<X : Set + ?Sized, Y: Set + ?Sized> {
    fn pset(&self) -> Y;
}

impl<X : Set, Y : Set> Set for dyn Pow<X, Y> {

}

/*pub trait U<X : Set, Y : Set> {
    fn do_u ( x : (dyn Pow< dyn Pow<X, Y>, Y>) -> X) -> (dyn Pow<dyn Pow<X, Y>, Y>);
}
*/
use std::marker::PhantomData;
pub enum U<X : Set, Y : Set> {
    Uu (PhantomData<X>, Box<dyn Fn(Box<dyn Fn (Box<dyn Pow<dyn Pow<X, Y>, Y>>) -> X>) -> Box<dyn Pow<dyn Pow<X, Y>, Y>>>)
}

impl <X : Set, Y : Set> Set for U<X, Y> {

}

use crate::U::Uu;

fn tao<X : Set, Y : Set> (t : Box<dyn Pow<dyn Pow<X, Y>, Y>>) -> U<X, Y> {
    Uu(PhantomData,
       Box::new(
        (|f|
            Box::new (|p : Box<dyn Fn(Box<dyn Pow<X, Y>, Y>) -> X>|
                (|x| p(f(x(f))))
            )
        ))
    )
}

/* 
pub enum Lift<A> {
    L(Box<dyn Fn(A) -> bool>)
}

use crate::Lift::L;

impl<A> Set<A> for Lift<A> {
    fn el(&self, x:Box<A>) -> bool {
        match self {
            L(f) => f(*x)
        }
    }
}

pub enum Pow<A> {
    P(Box<dyn Fn(Box<dyn Set<A>>) -> bool>)
}

use crate::Pow::P;

impl<A> Set<dyn Set<A>> for Pow<A> {
    fn el(&self, x : Box<dyn Set<A>>) -> bool {
        match self {
            P(f) => f(x)
    }}
}

/*
pub enum Forall<A, S : dyn Set<A>> {
    Fa()
}
*/

use std::marker::PhantomData;

pub enum U<A, X : Set<A>> {
    Uu (PhantomData<A>, Box<dyn Fn(dyn Fn (Box<Pow<Pow<X>>>) -> X) -> Box<Pow<Pow<X>>>>)
}

use crate::U::Uu;

impl<A, X : Set<A>> Set<dyn Fn (dyn Fn (Box<Pow<Pow<X>>>) -> X) -> Box<Pow<Pow<X>>>> for U<A, X> {
    fn el(&self, x) -> bool {
        match self {
            Uu(_,f) => f(x)
        }
    }
}


enum Set<A> {
    St(Box<dyn Fn(A) -> bool>)
}

fn pow<A> (S : Set<A>) -> Set<Set<A>> {
    Set::St(Box::new (|Sub| {
        match S {
            Set::St(S0) => 
                match Sub {
                    Set::St(Sub0) => 
                }
        }
    }))
}
*/
fn main() {
    println!("Hello, world!");
}
