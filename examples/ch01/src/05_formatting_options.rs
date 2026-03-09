//! Example 5: Formatting Options
//!
//! 다양한 포맷팅 옵션을 학습합니다.
//!
//! Demonstrates:
//! - 다양한 진법 (2, 8, 10, 16진수)
//! - 정렬과 패딩
//! - 소수점 정밀도

use std::f64::consts::PI;

fn main() {
    let number = 42;
    let gold = 12345;

    println!("=== 진법 변환 ===");
    println!("10진수: {}", number);
    println!("2진수: {:b}", number);
    println!("8진수: {:o}", number);
    println!("16진수(소문자): {:x}", number);
    println!("16진수(대문자): {:X}", number);

    println!();
    println!("=== 정렬과 패딩 ===");
    println!("오른쪽 정렬: |{:5}|", 12);
    println!("왼쪽 정렬: |{:<5}|", 12);
    println!("가운데 정렬: |{:^5}|", 12);
    println!("0으로 패딩: {:05}", 12);

    println!();
    println!("=== 게임에서의 활용 ===");
    println!("골드: {:>8}G", gold); // 오른쪽 정렬로 숫자 정리
    println!("HP: {:>3} / {:>3}", 100, 100);
    println!("MP: {:>3} / {:>3}", 50, 50);

    println!();
    println!("=== 소수점 정밀도 ===");
    println!("기본: {}", PI);
    println!("소수점 2자리: {:.2}", PI);
    println!("소수점 4자리: {:.4}", PI);
}
