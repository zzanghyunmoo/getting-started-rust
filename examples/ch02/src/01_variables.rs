//! Example 1: Basic Variable Declaration
//!
//! Demonstrates:
//! - let 키워드로 변수 선언
//! - 변수 명명 규칙 (snake_case)
//! - 변수 값 출력

fn main() {
    // 플레이어 이름 변수
    let player_name = "철수";

    // 직업 변수
    let player_class = "전사";

    // 변수 값 출력
    println!("용사의 이름: {}", player_name);
    println!("직업: {}", player_class);

    // 변수 명명 규칙 예시
    let player_hp = 100;
    let max_mp = 50;
    let is_alive = true;

    println!("HP: {}, MP: {}, 생존: {}", player_hp, max_mp, is_alive);
}
