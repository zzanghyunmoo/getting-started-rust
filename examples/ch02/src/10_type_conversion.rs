//! Example 10: Type Conversion
//!
//! Demonstrates:
//! - as 키워드로 명시적 타입 변환
//! - 안전한 vs 위험한 변환
//! - 암시적 변환 없음

fn main() {
    println!("=== 타입 변환 예시 ===");
    println!();

    // 기본 변환
    let hp_float: f64 = 100.0;
    let hp_int: u32 = 100;

    println!("=== 기본 변환 ===");
    println!("HP (f64): {}", hp_float);
    println!("HP (u32): {}", hp_int);

    // 명시적 변환
    let hp_total = hp_int + (hp_float as u32);
    println!("총 HP: {}", hp_total);

    println!();

    // 안전한 변환
    println!("=== 안전한 변환 ===");

    let damage: i32 = -15;
    let abs_damage = damage.unsigned_abs();
    println!("데미지 (i32): {}", damage);
    println!("절대값 데미지 (u32): {}", abs_damage);

    // 크기 확대 (안전)
    let small: u8 = 100;
    let large: u32 = small as u32;
    println!("u8 {} → u32 {}", small, large);

    println!();

    // 손실 가능 변환 (주의 필요)
    println!("=== 손실 가능 변환 (주의!) ===");

    let big: u32 = 300;
    let small_u8: u8 = big as u8; // 300 % 256 = 44
    println!("u32 {} → u8 {} (손실 발생!)", big, small_u8);

    let float_val: f64 = 3.9;
    let int_val: i32 = float_val as i32; // 소수점 버림
    println!("f64 {} → i32 {} (소수점 손실)", float_val, int_val);

    println!();

    // 문자로 변환
    println!("=== 문자로 변환 ===");

    let num: u8 = 65;
    let char_val: char = num as char;
    println!("u8 {} → char '{}'", num, char_val);

    // 한글 유니코드 (u32 → char는 char::from_u32 사용)
    let korean: u32 = 0xD55C; // '한'
    let korean_char = char::from_u32(korean).unwrap();
    println!("u32 {} → char '{}'", korean, korean_char);

    println!();

    // 변환 호환성
    println!("=== 변환 호환성 ===");

    // 정수 → 실수 (가능)
    let int_val: i32 = 42;
    let float_val: f64 = int_val as f64;
    println!("i32 {} → f64 {}", int_val, float_val);

    // 실수 → 정수 (가능하지만 손실)
    let float_val: f64 = 42.7;
    let int_val: i32 = float_val as i32;
    println!("f64 {} → i32 {}", float_val, int_val);

    // char → 정수 (가능)
    let letter: char = 'A';
    let num: u32 = letter as u32;
    println!("char '{}' → u32 {}", letter, num);
}
