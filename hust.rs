//
// use struct or enum to represents class
//

pub struct Hust {
    field: String,
}

impl Hust {
    pub fn new(str: String) -> Hust {
        Hust {
            field: str
        }
    }

    pub fn getter(&self) {
        println!("---> getter()");
        self.get();
    }

    fn get(&self) {
        println!("---> get()");
    }
}

/*
 * this code's effect same with the function we write above
pub enum Hust {
    A,
    B,
}

pub impl Hust {
    fn getter(&self) {
    
    }
}
*/
