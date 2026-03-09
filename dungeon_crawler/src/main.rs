/// 게임 밸런스 상수
const MAX_HP: u32 = 100;
const MAX_MP: u32 = 50;
const STARTING_GOLD: u32 = 100;

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
    // ANSI 색상 코드
    let yellow = "\x1b[33m";
    let cyan = "\x1b[36m";
    let reset = "\x1b[0m";

    // 상단 장식선 (노란색)
    println!("{}===================================={}", yellow, reset);

    // 게임 제목 (노란색)
    println!("{}         DUNGEON CRAWLER{}", yellow, reset);

    // 하단 장식선 (노란색)
    println!("{}===================================={}", yellow, reset);
    println!();

    // 개발자 정보
    println!("Developed by: 짱현무 (202412345)");
    println!("             컴퓨터 공학과");
    println!();

    // 버전 정보 (청록색)
    println!("{}Version: 1.0.0-alpha{}", cyan, reset);
    println!();

    // 시작 메시지
    println!("Press Enter to start...");
    println!();

    // ========================================
    // Chapter 2: 캐릭터 생성 시스템
    // ========================================

    // 1. 상수 출력
    println!("=== 게임 밸런스 ===");
    println!("최대 HP: {}", MAX_HP);
    println!("최대 MP: {}", MAX_MP);
    println!("초기 골드: {}G", STARTING_GOLD);
    println!();

    // 2. 플레이어 이름 (불변 변수)
    let player_name = "철수";

    // 3. 직업 (불변 변수)
    let player_class = "전사";

    // 4. 직업별 초기 능력치 (match와 튜플)
    let (hp, mp, attack, defense) = match player_class {
        "전사" => (WARRIOR_HP, WARRIOR_MP, WARRIOR_ATTACK, WARRIOR_DEFENSE),
        "마법사" => (MAGE_HP, MAGE_MP, MAGE_ATTACK, MAGE_DEFENSE),
        "도적" => (ROGUE_HP, ROGUE_MP, ROGUE_ATTACK, ROGUE_DEFENSE),
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
        WARRIOR_HP, WARRIOR_MP, WARRIOR_ATTACK, WARRIOR_DEFENSE
    );
    println!(
        "[마법사] HP:{} MP:{} 공격:{} 방어:{}",
        MAGE_HP, MAGE_MP, MAGE_ATTACK, MAGE_DEFENSE
    );
    println!(
        "[도적]   HP:{} MP:{} 공격:{} 방어:{}",
        ROGUE_HP, ROGUE_MP, ROGUE_ATTACK, ROGUE_DEFENSE
    );
    println!("====================================");
}
