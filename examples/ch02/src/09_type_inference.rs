//! Example 9: Type Inference
//!
//! Demonstrates:
//! - 컴파일러의 타입 추론
//! - 타입 어노테이션이 필요한 경우
//! - 추론의 한계

fn main() {
    println!("=== 타입 추론 예시 ===");
    println!();

    // 컴파일러가 자동으로 타입 추론
    let hp = 100; // i32로 추론
    let damage = 15.5; // f64로 추론
    let is_alive = true; // bool로 추론

    println!("=== 자동 추론 ===");
    println!("HP: {} (타입: i32)", hp);
    println!("데미지: {} (타입: f64)", damage);
    println!("생존: {} (타입: bool)", is_alive);

    println!();

    // 타입 어노테이션이 필요한 경우
    println!("=== 타입 어노테이션 필요 ===");

    // 1. 모호한 경우 (제네릭 메서드)
    let guess: u32 = "42".parse().expect("숫자가 아닙니다!");
    println!("추측: {} (u32)", guess);

    // 2. 특정 타입으로 지정
    let max_hp: u32 = 100; // u32로 명시
    let large_number: i64 = 100; // i64로 명시
    println!("최대 HP: {} (u32)", max_hp);
    println!("큰 수: {} (i64)", large_number);

    println!();

    // 타입 추론 활용
    println!("=== 스마트한 추론 ===");

    // 벡터의 요소 타입 추론
    let numbers = vec![1, 2, 3, 4, 5]; // Vec<i32>로 추론
    println!("숫자 벡터: {:?} (Vec<i32>)", numbers);

    // 튜플의 타입 추론
    let player = ("철수", 100, 50); // (&str, i32, i32)로 추론
    println!("플레이어: {:?} (튜플)", player);

    // 수식의 결과 타입 추론
    let result = 10 + 20; // i32 + i32 = i32
    println!("결과: {} (i32)", result);

    let float_result = 10.0 + 20.0; // f64 + f64 = f64
    println!("실수 결과: {} (f64)", float_result);
}
