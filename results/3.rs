// #[allow(text_direction_codepoint_in_literal)]
fn main() {
    let t = vec![
        /// ‮test⁦ RTL in doc in vec!
        //  ICE (Sadly)
        1
    ];
}

// https://github.com/rust-lang/rust/issues/140281