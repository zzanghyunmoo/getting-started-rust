//! Example 5: Integer Types
//!
//! Demonstrates:
//! - 부호 있는 정수형 (i8, i16, i32, i64, i128, isize)
//! - 부호 없는 정수형 (u8, u16, u32, u64, u128, usize)
//! - 게임에서의 활용 예시

fn main() {
    println!("=== 정수형 타입 예시 ===");
    println!();

    // 플레이어 스탯 (음수 없음 - unsigned)
    let hp: u32 = 100;
    let mp: u32 = 50;
    let gold: u32 = 150;

    println!("=== Unsigned Integers (음수 없음) ===");
    println!("HP: {} (u32)", hp);
    println!("MP: {} (u32)", mp);
    println!("Gold: {}G (u32)", gold);

    println!();

    // 공격력 (음수 가능: 디버프 - signed)
    let attack: i32 = 15;
    let defense: i32 = 5;
    let attack_buff: i32 = 5;
    let attack_debuff: i32 = -3;

    println!("=== Signed Integers (음수 가능) ===");
    println!("기본 공격력: {} (i32)", attack);
    println!("방어력: {} (i32)", defense);
    println!("공격력 버프: +{} (i32)", attack_buff);
    println!("공격력 디버프: {} (i32)", attack_debuff);

    println!();

    // 경험치 (매우 큰 값 가능)
    let exp: u64 = 1_234_567_890;
    let total_exp: u128 = 3_000_000_000_000;

    println!("=== Large Integers ===");
    println!("현재 경험치: {} (u64)", exp);
    println!("총 경험치: {} (u128)", total_exp);

    println!();

    // 정수 리터럴 표기법
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("=== 정수 리터럴 표기법 ===");
    println!("10진수: {}", decimal);
    println!("16진수: {} (0xff)", hex);
    println!("8진수: {} (0o77)", octal);
    println!("2진수: {} (0b1111_0000)", binary);
    println!("바이트: {} (b'A')", byte);
}
