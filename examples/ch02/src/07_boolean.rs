//! Example 7: Boolean Type
//!
//! Demonstrates:
//! - bool 타입 (true, false)
//! - 논리 연산 (&&, ||, !)
//! - 게임 상태 플래그

fn main() {
    println!("=== 불리언 타입 예시 ===");
    println!();

    // 게임 상태 플래그
    let is_alive: bool = true;
    let is_poisoned: bool = false;
    let has_key: bool = true;
    let has_treasure: bool = false;

    println!("=== 플레이어 상태 ===");
    println!("생존 여부: {}", is_alive);
    println!("중독 상태: {}", is_poisoned);
    println!("키 보유: {}", has_key);
    println!("보물 획득: {}", has_treasure);

    println!();

    // 논리 연산
    println!("=== 논리 연산 ===");

    // AND 연산: 둘 다 true면 true
    if is_alive && has_key {
        println!("문을 열 수 있습니다! (생존 && 키 보유)");
    }

    // OR 연산: 하나라도 true면 true
    if is_poisoned || !has_treasure {
        println!("아직 던전을 탐험해야 합니다!");
    }

    // NOT 연산: true/false 반전
    println!("키 보유 안 함: {}", !has_key);
    println!("보물 미획득: {}", !has_treasure);

    println!();

    // 복합 조건
    let can_open_door = is_alive && has_key && !is_poisoned;
    println!("문 열기 가능 (생존 && 키 && !중독): {}", can_open_door);

    // 전투 가능 여부
    let can_fight = is_alive && !is_poisoned;
    println!("전투 가능 (생존 && !중독): {}", can_fight);
}
