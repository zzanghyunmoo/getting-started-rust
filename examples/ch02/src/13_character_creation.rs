//! Example 13: Character Creation System (Practice)
//!
//! This is the main exercise from Chapter 2.
//! Demonstrates all concepts learned:
//! - Variables (mutable and immutable)
//! - Constants
//! - Type annotations
//! - Tuples for multiple values
//! - Arrays for job stats

/// 게임 밸런스 상수
const MAX_HP: u32 = 100;
const MAX_MP: u32 = 50;
const STARTING_GOLD: u32 = 100;

/// 직업별 초기 능력치 배열
const WARRIOR_STATS: [i32; 4] = [100, 50, 15, 5]; // HP, MP, 공격, 방어
const MAGE_STATS: [i32; 4] = [60, 100, 10, 3];
const ROGUE_STATS: [i32; 4] = [80, 60, 12, 8];

fn main() {
    // 1. 상수 출력
    println!("=== 게임 밸런스 ===");
    println!("최대 HP: {}", MAX_HP);
    println!("최대 MP: {}", MAX_MP);
    println!("초기 골드: {}G", STARTING_GOLD);
    println!();

    // 2. 플레이어 이름 (불변)
    let player_name = "철수";

    // 3. 직업 (불변)
    let player_class = "전사";

    // 4. 직업별 초기 능력치 (튜플로 받기)
    let (hp, mp, attack, defense) = match player_class {
        "전사" => (
            WARRIOR_STATS[0],
            WARRIOR_STATS[1],
            WARRIOR_STATS[2],
            WARRIOR_STATS[3],
        ),
        "마법사" => (MAGE_STATS[0], MAGE_STATS[1], MAGE_STATS[2], MAGE_STATS[3]),
        "도적" => (
            ROGUE_STATS[0],
            ROGUE_STATS[1],
            ROGUE_STATS[2],
            ROGUE_STATS[3],
        ),
        _ => (80, 50, 10, 5), // 기본값
    };

    // 5. 캐릭터 정보 출력
    println!("====================================");
    println!("캐릭터가 생성되었습니다!");
    println!("====================================");
    println!("이름: {}", player_name);
    println!("직업: {}", player_class);
    println!("HP: {} / MP: {}", hp, mp);
    println!("공격력: {} / 방어력: {}", attack, defense);
    println!("골드: {}G", STARTING_GOLD);
    println!("====================================");

    println!();

    // 도전 과제: 직업별 스탯 비교
    println!("====================================");
    println!("직업별 스탯 비교");
    println!("====================================");
    println!(
        "[전사]   HP:{} MP:{} 공격:{} 방어:{}",
        WARRIOR_STATS[0], WARRIOR_STATS[1], WARRIOR_STATS[2], WARRIOR_STATS[3]
    );
    println!(
        "[마법사] HP:{} MP:{} 공격:{} 방어:{}",
        MAGE_STATS[0], MAGE_STATS[1], MAGE_STATS[2], MAGE_STATS[3]
    );
    println!(
        "[도적]   HP:{} MP:{} 공격:{} 방어:{}",
        ROGUE_STATS[0], ROGUE_STATS[1], ROGUE_STATS[2], ROGUE_STATS[3]
    );
    println!("====================================");
}
