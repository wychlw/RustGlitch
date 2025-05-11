static mut S: [i8] = ["Some thing"; 1];
fn main() {
    let p = (&S, &[0; 1]);
}

// https://github.com/rust-lang/rust/issues/140332
