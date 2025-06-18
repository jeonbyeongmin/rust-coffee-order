// lib.rs에서 정의한 타입들을 사용하기 위해 import
use option_match::coffee::{Coffee, Size};
use option_match::order::{Order, OrderItem};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("커피 주문 시스템 예제");

    // 다양한 커피 생성
    let espresso = Coffee::Espresso;
    let large_latte = Coffee::Latte(Size::Large);
    let cappuccino = Coffee::Cappuccino;

    // OrderItem 생성 - 적절한 에러 처리
    let item1 = OrderItem::new(espresso, 2)?;
    let item2 = OrderItem::new(large_latte, 1)?;
    let item3 = OrderItem::new(cappuccino, 3)?;

    // 주문 생성
    let order = Order::new(vec![item1, item2, item3]);

    // 주문 정보 출력
    order.print_order_info();

    Ok(())
}
