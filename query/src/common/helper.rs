// export the macro to the crate root and the macro could be referenced via crate::fail just like
// normal symbols
#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => {
        panic!($($arg)*)
    }
}

#[cfg(test)]
mod test{
    #[should_panic]
    #[test]
    pub fn test_fail() {
        fail!("panic");
    }
}
