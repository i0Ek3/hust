pub mod hust {
    // undefined module name _1/_2/_3
    pub mod _1 {
        //
        pub fn _1f() -> u32 {
            return 1;
        }
    }

    pub mod _2 {
        //
        pub fn _2f() {}
    }

    pub mod _3 {
        //
        fn _3f() {
            super::_2::_2f();
        }
    }
}

/*
fn main() {
    hust::_1::_1f();
}*/
