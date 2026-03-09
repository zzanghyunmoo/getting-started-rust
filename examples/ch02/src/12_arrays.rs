//! Example 12: Arrays
//!
//! Demonstrates:
//! - 배열 선언과 사용
//! - 인덱스로 접근
//! - 같은 값으로 초기화

fn main() {
    println!("=== 배열 예시 ===");
    println!();

    // 직업별 능력치 배열
    let warrior_stats: [i32; 4] = [100, 50, 15, 5]; // HP, MP, 공격, 방어
    let mage_stats: [i32; 4] = [60, 100, 10, 3];
    let rogue_stats: [i32; 4] = [80, 60, 12, 8];

    println!("=== 직업별 스탯 ===");
    println!(
        "전사: HP {} MP {} 공격 {} 방어 {}",
        warrior_stats[0], warrior_stats[1], warrior_stats[2], warrior_stats[3]
    );
    println!(
        "마법사: HP {} MP {} 공격 {} 방어 {}",
        mage_stats[0], mage_stats[1], mage_stats[2], mage_stats[3]
    );
    println!(
        "도적: HP {} MP {} 공격 {} 방어 {}",
        rogue_stats[0], rogue_stats[1], rogue_stats[2], rogue_stats[3]
    );

    println!();

    // 같은 값으로 초기화
    let inventory = [0; 20]; // 빈 인벤토리 20칸
    println!("인벤토리 크기: {}칸", inventory.len());
    println!("첫 번째 슬롯: {}", inventory[0]);

    println!();

    // 배열 반복
    let numbers = [10, 20, 30, 40, 50];
    println!("=== 배열 요소 ===");

    for (i, &value) in numbers.iter().enumerate() {
        println!("numbers[{}] = {}", i, value);
    }

    println!();

    // 안전한 접근 방법
    println!("=== 안전한 접근 ===");

    match numbers.get(10) {
        Some(&value) => println!("인덱스 10: {}", value),
        None => println!("인덱스 10: 존재하지 않음"),
    }

    // 배열 길이
    println!("=== 배열 정보 ===");
    println!("길이: {}", numbers.len());
    println!("메모리 크기: {} bytes", std::mem::size_of_val(&numbers));
    println!("요소당 크기: {} bytes", std::mem::size_of::<i32>());

    println!();

    // 배열 수정 (mut)
    let mut mutable_stats = [100, 50, 15, 5];
    println!("변경 전: {:?}", mutable_stats);

    mutable_stats[0] = 90; // HP 감소
    println!("변경 후: {:?}", mutable_stats);
}
