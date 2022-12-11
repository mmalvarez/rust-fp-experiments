// \ x . x x


enum Recfun<'a, A : 'static> {
    Rf(&'a fn(Recfun<A>, A) -> A)
}

fn Y<A>(f : fn(A) -> A) -> Box<dyn Fn(A) -> A> {

    let omega =
        |x : Recfun<A>| -> Box<dyn Fn(A) -> A> {
            Box::new (|v| {
                match x {
                    Recfun::Rf(xi) => f((xi(x))(v))
                }
            })
        };
    Box::new(omega(Recfun::Rf(Box::new(omega))))
}


/* 
fn eta<A, B>(x : Recfun <A, B>) -> fn(A) -> B {
    match x {
        Recfun::Rf(xi) => (xi(x)),
    }
}

fn omega<A, B>(f : Recfun<A, B>, x : Recfun<A, B>) -> fn(A) -> B {
    match f {
        Recfun::Rf(fi) => (fi(eta(x))),
    }
}
*/

/*
fn Z<A, B>(f : Recfun<A, B>) -> A -> B {
    let omega<A, B> = (|x : Recfun<A, B>| -> Recfun<A, B> {
        let eta<A> = (|v : A| -> B {
            match x {
                Recfun::rf(xi) => (xi(x))(v)
            }
        });

    });
}
*/

/*
fn Y<A, B>(f : Recfun<A, B>) -> A -> B {
    let omega<A, B> = (|x : Recfun<A, B>| -> Recfun<A, B> {
        match x {
            Recfun::Rf(xi) => 
        }
    });
}
*/

/*
fn Z<A, B> (f : Recfun<A, B>) -> fn(A) -> B {
    fn omega<A, B>(x : Recfun<A, B>) -> fn(A) -> B {
        fn eta<A, B>(v : A) -> B {
            let result1 = match x {
                Recfun::Rf(xi) => (xi(x))(v),
            }
        }
    }
}
*/

fn main() {
    println!("Hello, world!");
}
