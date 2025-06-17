use crate::coffee::Coffee;

// 구조체(struct) 정의 - 관련된 데이터를 그룹화
pub struct OrderItem {
    pub coffee: Coffee, // Coffee enum 타입의 필드
    pub quantity: u32,  // 부호 없는 32비트 정수
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
    pub fn line_total(&self) -> u32 {
        // 메서드 체이닝 - coffee의 get_price() 호출 후 quantity와 곱셈
        self.coffee.get_price() * self.quantity
    }
}

// 주문을 나타내는 구조체
pub struct Order {
    items: Vec<OrderItem>, // OrderItem들의 벡터 (동적 배열)
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

    // 전체 주문 금액 계산
    pub fn get_total(&self) -> u32 {
        let mut total = 0; // 가변 변수 선언
        // for 루프로 벡터의 각 아이템에 대해 반복
        // &self.items - items에 대한 불변 참조
        for item in &self.items {
            let line_total = item.line_total();
            total += line_total // 누적 합계
        }
        total // 마지막 표현식이 반환값 (세미콜론 없음)
    }

    // 주문 정보 출력 메서드
    pub fn print_order_info(&self) {
        // 각 아이템에 대해 반복하며 정보 출력
        for item in &self.items {
            let line_total = item.line_total();
            // println! 매크로 사용 - 콘솔에 출력
            println!(
                "{} x {} - {}",
                item.coffee.get_name(), // 커피 이름
                item.quantity,          // 수량
                line_total              // 라인 총액
            );
        }
        // 전체 총액 출력
        println!("Total: {}", &self.get_total())
    }
}
