//! Example 8: Character Type
//!
//! Demonstrates:
//! - char 타입 (4바이트 Unicode)
//! - 한글, 이모지 지원
//! - Unicode Scalar Value

fn main() {
    println!("=== 문자 타입 예시 ===");
    println!();

    // 다양한 문자 타입
    let first_char: char = '철';
    let sword: char = '⚔'; // 단일 유니코드 문자
    let grade: char = 'A';
    let star: char = '★';
    let heart: char = '❤'; // 단일 유니코드 문자

    println!("=== 다양한 문자 ===");
    println!("이름의 첫 글자: {}", first_char);
    println!("무기 이모지: {}", sword);
    println!("등급: {}", grade);
    println!("별: {}", star);
    println!("하트: {}", heart);

    println!();

    // 문자 vs 문자열
    println!("=== 문자 vs 문자열 ===");

    let single_char: char = 'A'; // 4 bytes, 1 character
    let string_slice: &str = "A"; // 문자열 슬라이스

    println!("char: '{}' (타입: char)", single_char);
    println!("&str: \"{}\" (타입: &str)", string_slice);

    println!();

    // 아스키 코드 변환
    let letter: char = 'A';
    let ascii_value = letter as u32;

    println!("=== 아스키 코드 ===");
    println!("문자: {}", letter);
    println!("아스키 값: {}", ascii_value);

    let b: char = 'B';
    println!("다음 문자: {} (아스키: {})", b, b as u32);

    println!();

    // 특수 문자
    let newline = '\n';
    let tab = '\t';
    let null = '\0';

    println!("=== 특수 문자 ===");
    println!("줄바꿈: '\\n' (아스키: {})", newline as u32);
    println!("탭: '\\t' (아스키: {})", tab as u32);
    println!("널 문자: '\\0' (아스키: {})", null as u32);
}
