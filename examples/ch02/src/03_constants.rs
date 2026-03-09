//! Example 3: Constants
//!
//! Demonstrates:
//! - const 키워드로 상수 선언
//! - 타입 어노테이션 필수
//! - 컴파일 타임 상수

/// 게임 밸런스 상수
const MAX_HP: u32 = 100;
const MAX_MP: u32 = 50;
const STARTING_GOLD: u32 = 100;
const LEVEL_UP_EXP: u32 = 100;

/// 직업별 스탯 상수
const WARRIOR_HP: u32 = 100;
const WARRIOR_MP: u32 = 50;
const WARRIOR_ATTACK: i32 = 15;
const WARRIOR_DEFENSE: i32 = 5;

const MAGE_HP: u32 = 60;
const MAGE_MP: u32 = 100;
const MAGE_ATTACK: i32 = 10;
const MAGE_DEFENSE: i32 = 3;

const ROGUE_HP: u32 = 80;
const ROGUE_MP: u32 = 60;
const ROGUE_ATTACK: i32 = 12;
const ROGUE_DEFENSE: i32 = 8;

fn main() {
    println!("=== 게임 밸런스 설정 ===");
    println!("최대 HP: {}", MAX_HP);
    println!("최대 MP: {}", MAX_MP);
    println!("초기 골드: {}G", STARTING_GOLD);
    println!("레벨업 경험치: {}", LEVEL_UP_EXP);

    println!();
    println!("=== 직업별 스탯 ===");
    println!(
        "전사: HP {} MP {} 공격력 {} 방어력 {}",
        WARRIOR_HP, WARRIOR_MP, WARRIOR_ATTACK, WARRIOR_DEFENSE
    );
    println!(
        "마법사: HP {} MP {} 공격력 {} 방어력 {}",
        MAGE_HP, MAGE_MP, MAGE_ATTACK, MAGE_DEFENSE
    );
    println!(
        "도적: HP {} MP {} 공격력 {} 방어력 {}",
        ROGUE_HP, ROGUE_MP, ROGUE_ATTACK, ROGUE_DEFENSE
    );
}
