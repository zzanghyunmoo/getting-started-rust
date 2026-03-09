//! Example 2: Mutable Variables
//!
//! Demonstrates:
//! - mut 키워드로 가변 변수 선언
//! - 값 변경 가능
//! - 게임 상태 변화 예시

fn main() {
    println!("=== 가변 변수 예시 ===");
    println!();

    // 가변 변수로 HP 선언
    let mut player_hp = 100;
    println!("초기 HP: {}", player_hp);

    // 전투에서 피해를 입음
    player_hp = 85;
    println!("전투 후 HP: {}", player_hp);

    // 포션으로 회복
    player_hp = 100;
    println!("회복 후 HP: {}", player_hp);

    println!();
    println!("=== 불변 변수와 비교 ===");

    // 불변 변수는 변경 불가능
    let max_hp = 100;
    println!("최대 HP: {}", max_hp);
    // max_hp = 150;  // 컴파일 에러!
}
