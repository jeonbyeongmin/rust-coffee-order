use std::fmt;

/// 금액을 표현하는 타입 (원 단위)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub u32);

impl Price {
    pub fn new(amount: u32) -> Self {
        Self(amount)
    }

    pub fn amount(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "₩{}", self.0)
    }
}

impl std::ops::Add for Price {
    type Output = Price;

    fn add(self, other: Price) -> Price {
        Price(self.0 + other.0)
    }
}

impl std::ops::AddAssign for Price {
    fn add_assign(&mut self, other: Price) {
        self.0 += other.0;
    }
}

impl std::ops::Mul<u32> for Price {
    type Output = Price;

    fn mul(self, quantity: u32) -> Price {
        Price(self.0 * quantity)
    }
}

impl std::iter::Sum for Price {
    fn sum<I: Iterator<Item = Price>>(iter: I) -> Self {
        iter.fold(Price::new(0), |acc, price| acc + price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_display() {
        let price = Price::new(2500);
        assert_eq!(format!("{}", price), "₩2500");
    }

    #[test]
    fn test_price_addition() {
        let price1 = Price::new(2000);
        let price2 = Price::new(3000);
        let total = price1 + price2;
        assert_eq!(total.amount(), 5000);
    }

    #[test]
    fn test_price_multiplication() {
        let price = Price::new(2500);
        let total = price * 3;
        assert_eq!(total.amount(), 7500);
    }
}
