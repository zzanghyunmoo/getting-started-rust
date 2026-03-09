//! Example 4: Shadowing
//!
//! Demonstrates:
//! - 섀도잉으로 같은 이름의 새 변수 선언
//! - 타입 변경 가능
//! - 데이터 변환 파이프라인

fn main() {
    println!("=== 섀도잉 기본 예시 ===");

    let x = 5;
    println!("초기 x: {}", x);

    let x = x + 1; // 섀도잉: x는 6
    println!("섀도잉 1 (x + 1): {}", x);

    let x = x * 2; // 또 다른 섀도잉: x는 12
    println!("섀도잉 2 (x * 2): {}", x);

    println!();
    println!("=== 섀도잉으로 타입 변경 ===");

    let spaces = "   "; // &str 타입
    println!("문자열 공백: '{}'", spaces);

    let spaces = spaces.len(); // usize 타입으로 변경
    println!("공백 개수: {}", spaces);

    println!();
    println!("=== 데이터 변환 파이프라인 ===");

    // 1단계: 입력 (공백 포함)
    let name = "  철수  ";
    println!("1. 입력: '{}'", name);

    // 2단계: 섀도잉 - 공백 제거
    let name = name.trim();
    println!("2. trim(): '{}'", name);

    // 3단계: 섀도잉 - 유효성 검사 후 기본값
    let name = if name.is_empty() { "모험가" } else { name };
    println!("3. 최종 이름: {}", name);

    println!();
    println!("=== 스코프와 섀도잉 ===");

    let x = 5; // 스코프 1: x는 5
    println!("외부 x: {}", x);

    {
        let x = x * 2; // 스코프 2: 내부 섀도잉, x는 10
        println!("내부 x: {}", x);

        {
            let x = x + 1; // 스코프 3: 더 내부, x는 11
            println!("더 내부 x: {}", x);
        }
    }

    println!("다시 외부 x: {}", x);
}
