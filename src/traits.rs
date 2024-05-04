trait Trait {
    fn f(&self);
}

impl Trait for u32 {
    fn f(&self) {
        print!("1");
    }
}

impl<'a> Trait for &'a i32 {
    fn f(&self) {
        print!("2");
    }
}

pub fn run_traits() {
    let x = &0; 
    x.f();

    // guess the ouput
}


// x needs to inferred as &{something} argument x must be of type &Self for some type Self that implements Trait.
// We find that inferring 0: u32 satisfies both the constraint that u32 is an integer as well as u32 implements Trait, 
//so the method call ends up calling <u32 as Trait>::f(x) and prints 1.