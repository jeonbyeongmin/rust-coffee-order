use crate::coffee::Coffee;
use crate::price::Price;
use std::fmt;

// 구조체(struct) 정의 - 관련된 데이터를 그룹화
#[derive(Debug, Clone)]
pub struct OrderItem {
    pub coffee: Coffee, // Coffee enum 타입의 필드
    pub quantity: u32,  // 부호 없는 32비트 정수
}

impl fmt::Display for OrderItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} x {} - {}", self.coffee, self.quantity, self.line_total())
    }
}

impl OrderItem {
    // 생성자 함수 - Result<T, E> 타입 반환 (에러 처리)
    pub fn new(coffee: Coffee, quantity: u32) -> Result<Self, String> {
        if quantity > 0 {
            // Ok는 성공 케이스를 나타냄
            Ok(Self { coffee, quantity })
        } else {
            // Err는 에러 케이스를 나타냄
            Err("Quantity must be greater than 0".to_string())
        }
    }

    // 라인 총액 계산 메서드
    pub fn line_total(&self) -> Price {
        // 메서드 체이닝 - coffee의 get_price() 호출 후 quantity와 곱셈
        self.coffee.get_price() * self.quantity
    }
}

// 주문을 나타내는 구조체
#[derive(Debug)]
pub struct Order {
    items: Vec<OrderItem>, // OrderItem들의 벡터 (동적 배열)
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== 주문 내역 ===")?;
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        writeln!(f, "====================")?;
        write!(f, "Total: {}", self.get_total())
    }
}

impl Order {
    // 새로운 Order 인스턴스 생성
    pub fn new(items: Vec<OrderItem>) -> Self {
        Self { items }
    }

    // items 필드에 대한 불변 참조 반환 (getter 메서드)
    // &Vec<OrderItem> 반환 - 소유권을 이전하지 않고 빌려줌
    pub fn items(&self) -> &Vec<OrderItem> {
        &self.items
    }

    // 전체 주문 금액 계산 - 함수형 프로그래밍 스타일
    pub fn get_total(&self) -> Price {
        self.items
            .iter()
            .map(|item| item.line_total())
            .sum()
    }

    // 주문 정보 출력 메서드 - Display trait을 활용
    pub fn print_order_info(&self) {
        println!("{}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coffee::{Coffee, Size};

    #[test]
    fn test_order_item_creation() {
        let coffee = Coffee::Espresso;
        let item = OrderItem::new(coffee, 2).unwrap();
        assert_eq!(item.quantity, 2);
        assert_eq!(item.line_total(), Price::new(4000)); // 2000 * 2
    }

    #[test]
    fn test_order_item_invalid_quantity() {
        let coffee = Coffee::Cappuccino;
        let result = OrderItem::new(coffee, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_order_total() {
        let item1 = OrderItem::new(Coffee::Espresso, 1).unwrap(); // 2000
        let item2 = OrderItem::new(Coffee::Latte(Size::Large), 2).unwrap(); // 2700 * 2 = 5400
        let order = Order::new(vec![item1, item2]);
        
        assert_eq!(order.get_total(), Price::new(7400));
    }

    #[test]
    fn test_empty_order() {
        let order = Order::new(vec![]);
        assert_eq!(order.get_total(), Price::new(0));
    }
}
