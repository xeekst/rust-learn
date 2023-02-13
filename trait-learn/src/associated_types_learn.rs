trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}

struct Container(i32, i32);

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, n1: &i32, n2: i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
}

