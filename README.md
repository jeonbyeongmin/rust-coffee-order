# Rust 커피 주문 시스템

Rust의 enum, pattern matching, 에러 처리 등의 개념을 학습하기 위한 샘플 프로젝트입니다.

## 기능

- 다양한 커피 종류와 사이즈 지원
- 주문 항목 생성 및 관리
- 총 주문 금액 계산
- 타입 안전한 금액 처리
- 포괄적인 에러 처리

## 코드 구조

```
src/
├── lib.rs          # 라이브러리 루트
├── main.rs         # 메인 실행 파일
├── coffee/         # 커피 관련 타입
│   └── mod.rs
├── order/          # 주문 관련 타입
│   └── mod.rs
└── price.rs        # 금액 타입
```

## 주요 개념

### Enum과 Pattern Matching

- `Coffee`와 `Size` enum을 통한 타입 안전성
- `match` 표현식을 활용한 포괄적인 패턴 매칭

### 에러 처리

- `Result<T, E>`를 활용한 안전한 에러 처리
- `?` 연산자를 통한 에러 전파

### 타입 시스템

- 커스텀 `Price` 타입으로 타입 안전한 계산
- trait 구현을 통한 연산자 오버로딩

### 함수형 프로그래밍

- Iterator와 `map`, `sum`을 활용한 함수형 스타일

## 실행 방법

```bash
# 프로젝트 빌드
cargo build

# 프로그램 실행
cargo run

# 테스트 실행
cargo test
```

## 예제 출력

```
커피 주문 시스템 예제
=== 주문 내역 ===
Espresso x 2 - ₩4000
Latte (Large) x 1 - ₩2700
Cappuccino x 3 - ₩9000
====================
Total: ₩15700
```

## 학습 포인트

1. **타입 안전성**: Rust의 타입 시스템을 활용한 안전한 코드
2. **패턴 매칭**: 모든 경우를 다루는 exhaustive matching
3. **에러 처리**: Result 타입을 활용한 명시적 에러 처리
4. **소유권**: 빌림과 소유권을 통한 메모리 안전성
5. **Trait 시스템**: 코드 재사용과 추상화
