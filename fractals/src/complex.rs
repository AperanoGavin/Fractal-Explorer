#[derive(Clone, Copy)]

pub struct Complex {
    pub a: f32, 
    pub b: f32,
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex { 
            a: self.a * rhs.a - self.b * rhs.b, 
            b: self.a * rhs.b + self.b * rhs.a ,
        }
    }
}

impl Complex {
   pub  fn arg_sq(self) -> f32 {
        self.a * self.a + self.b * self.b
    }
}

impl Complex {
   pub fn abs(self) -> Self {
        Complex { 
            a: self.a.abs(), 
            b: self.b.abs()
        }
    }
}