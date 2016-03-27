const PI: f32 = 3.141592653589793;

pub trait Trig {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
}

impl Trig for f32 {
    fn sin(mut self) -> f32 {
        fn sin_imp(x: f32) -> f32 {
            x - x * x * x / 6.0 + x * x * x * x * x / 120.0 - x * x * x * x * x * x * x / 5040.0 // + x * x * x * x * x * x * x * x * x / 362880.0
        }

        self = self % (2.0 * PI);

        if self.is_sign_negative() {
            -(-self).sin()
        } else if self < PI / 2.0 {
            println!("s < pi / 2");
            sin_imp(self)
        } else if self < PI {
            println!("s < pi");
            1.0 - sin_imp(self -  PI / 2.0)
        } else if self < 3.0 / 2.0 * PI {
            println!("s < 3/2 pi");
            -sin_imp(self - PI)
        } else {
            println!("s > 3/2 pi");
            sin_imp(self - 3.0 / 2.0 * PI) + 1.0
        }
    }

    fn cos(self) -> f32 {
        (self + PI / 2.0).sin()
    }

    fn tan(self) -> f32 {
        self.sin() / self.cos()
    }

}