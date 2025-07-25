// 열거형(enum) 정의 - 러스트에서 유한한 선택지를 표현하는 방법
use crate::price::Price;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Small, // 단순한 variant
    Medium,
    Large,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Size::Small => write!(f, "Small"),
            Size::Medium => write!(f, "Medium"),
            Size::Large => write!(f, "Large"),
        }
    }
}

// 열거형에 데이터를 포함할 수 있음 - 러스트의 강력한 특징
#[derive(Debug, Clone)]
pub enum Coffee {
    Espresso,    // 데이터를 포함하지 않는 variant
    Latte(Size), // Size enum을 포함하는 variant (튜플 스타일)
    Cappuccino,
}

impl fmt::Display for Coffee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

// impl 블록 - 타입에 대한 메서드 구현
impl Coffee {
    // 연관 함수(associated function) - &self 매개변수 없음
    fn describe_latte(size: &Size) -> String {
        // match 표현식 - 패턴 매칭을 통한 분기 처리
        // 러스트의 match는 모든 경우를 다뤄야 함 (exhaustive)
        match size {
            Size::Small => "Latte (Small)".to_string(),
            Size::Medium => "Latte (Medium)".to_string(),
            Size::Large => "Latte (Large)".to_string(),
        }
    }

    fn latte_price(size: &Size) -> Price {
        // 다시 한번 match 사용 - 각 size에 따른 가격 반환
        match size {
            Size::Small => Price::new(2300),
            Size::Medium => Price::new(2500),
            Size::Large => Price::new(2700),
        }
    }

    // 공개 메서드(public method) - &self를 받아 인스턴스에서 호출 가능
    pub fn get_name(&self) -> String {
        // self의 variant에 따라 다른 처리
        match self {
            Coffee::Espresso => format!("Espresso"),
            // 패턴 매칭으로 내부 데이터 추출 - size는 Latte variant 내부의 Size 값
            Coffee::Latte(size) => Self::describe_latte(size),
            Coffee::Cappuccino => format!("Cappuccino"),
        }
    }

    pub fn get_price(&self) -> Price {
        match self {
            Coffee::Espresso => Price::new(2000),
            // Self::latte_price - 같은 타입의 연관 함수 호출
            Coffee::Latte(size) => Self::latte_price(size),
            Coffee::Cappuccino => Price::new(3000),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coffee_prices() {
        use crate::price::Price;

        let espresso = Coffee::Espresso;
        let latte_small = Coffee::Latte(Size::Small);
        let latte_large = Coffee::Latte(Size::Large);
        let cappuccino = Coffee::Cappuccino;

        assert_eq!(espresso.get_price(), Price::new(2000));
        assert_eq!(latte_small.get_price(), Price::new(2300));
        assert_eq!(latte_large.get_price(), Price::new(2700));
        assert_eq!(cappuccino.get_price(), Price::new(3000));
    }

    #[test]
    fn test_coffee_names() {
        let espresso = Coffee::Espresso;
        let latte_medium = Coffee::Latte(Size::Medium);
        let cappuccino = Coffee::Cappuccino;

        assert_eq!(espresso.get_name(), "Espresso");
        assert_eq!(latte_medium.get_name(), "Latte (Medium)");
        assert_eq!(cappuccino.get_name(), "Cappuccino");
    }

    #[test]
    fn test_size_display() {
        assert_eq!(format!("{}", Size::Small), "Small");
        assert_eq!(format!("{}", Size::Medium), "Medium");
        assert_eq!(format!("{}", Size::Large), "Large");
    }
}
