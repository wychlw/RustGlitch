fn f() -> &'static str
where
    Self: Sized,
{
    ""
}

// https://github.com/rust-lang/rust/issues/140365