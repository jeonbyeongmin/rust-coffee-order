// 열거형(enum) 정의 - 러스트에서 유한한 선택지를 표현하는 방법
pub enum Size {
    Small, // 단순한 variant
    Medium,
    Large,
}

// 열거형에 데이터를 포함할 수 있음 - 러스트의 강력한 특징
pub enum Coffee {
    Espresso,    // 데이터를 포함하지 않는 variant
    Latte(Size), // Size enum을 포함하는 variant (튜플 스타일)
    Cappuccino,
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

    fn latte_price(size: &Size) -> u32 {
        // 다시 한번 match 사용 - 각 size에 따른 가격 반환
        match size {
            Size::Small => 2300,
            Size::Medium => 2500,
            Size::Large => 2700,
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

    pub fn get_price(&self) -> u32 {
        match self {
            Coffee::Espresso => 2000,
            // Self::latte_price - 같은 타입의 연관 함수 호출
            Coffee::Latte(size) => Self::latte_price(size),
            Coffee::Cappuccino => 3000,
        }
    }
}
