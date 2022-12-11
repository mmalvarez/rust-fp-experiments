pub trait Type {
    type T : Type + ?Sized; // : Type?
}

pub trait Absurd {
    type T<X : Type> : Absurd; // TODO: we need to return X::T somehow
}

// enum absurd ...?

pub trait Pred<A : Type> {
    type T<X> : Type;
}

//impl<A> Type for Pred<A> {
//    type T = 
//}


pub trait Eq<A : Type, X, Y> {
    type P : Pred<A>;
    fn get(&self, _ : <<Self as Eq<A, X, Y>>::P as Pred<A>>::T<X>) -> <<Self as Eq<A, X, Y>>::P as Pred<A>>::T<Y>;
}


fn main() {
    println!("Hello, world!");
}
