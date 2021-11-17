use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    fn new(real: f64, imagine: f64) -> Self {
        Self {
            real, imagine,
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}

impl Add for &Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;
    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        Complex::new(real, self.imagine)
    }
}

#[test]
fn add_should_work() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);
    let c3 = &c1 + &c2;
    assert_eq!(c3.real, 3.0);
    assert_eq!(c3.imagine, 4.0);

    let c4 = c1 + c2;
    assert_eq!(c4.real, 3.0);
    assert_eq!(c4.imagine, 4.0);

    let c5 = Complex::new(1.0, 1f64);
    let c6 = &c5 + 1.0;
    assert_eq!(c6.real, 2.0);
    assert_eq!(c6.imagine, 1.0);
}