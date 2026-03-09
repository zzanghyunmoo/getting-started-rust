//! Example 7: Special Characters
//!
//! 특수 문자와 원시 문자열을 학습합니다.
//!
//! Demonstrates:
//! - 이스케이프 시퀀스
//! - 원시 문자열 (raw string)

fn main() {
    println!("=== 이스케이프 시퀀스 ===");
    println!("줄바꿈\\n입니다");
    println!("탭\\t입니다");
    println!("큰따옴표\\\"입니다");
    println!("백슬래시\\\\입니다");

    println!();

    // 실제 이스케이프 문자 적용
    println!("실제 적용:");
    println!("줄바꿈\n입니다");
    println!("탭\t입니다");
    println!("큰따옴표\"입니다");
    println!("백슬래시\\입니다");

    println!();
    println!("=== 원시 문자열 (Raw String) ===");
    println!(r"C:\Users\name");
    println!(r#"이것은 "따옴표" 포함"#);

    println!();
    println!("=== 게임 메시지 예시 ===");
    println!("공격! 데미지: {}\n\\n이스케이프 연습", 100);
    println!(r"경로: C:\Games\Dungeon\Crawler");
}
