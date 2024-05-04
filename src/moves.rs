struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}

pub fn run_move() {
    // let s = S;
    // let hello = s;
    // print!("2");
    // guess output

}

// s is not moved