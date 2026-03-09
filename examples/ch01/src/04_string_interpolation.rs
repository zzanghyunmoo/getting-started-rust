//! Example 4: String Interpolation
//!
//! 문자열 보간을 사용하여 변수 값을 출력합니다.
//!
//! Demonstrates:
//! - {} 플레이스홀더
//! - 여러 개의 플레이스홀더
//! - 인자 번호로 순서 지정

fn main() {
    // 게임 관련 변수
    let player_name = "철수";
    let player_level = 5;
    let player_class = "전사";

    // 기본 플레이스홀더
    println!("용사의 이름: {}", player_name);
    println!("레벨: {}", player_level);
    println!("직업: {}", player_class);

    println!();

    // 여러 개의 플레이스홀더
    println!(
        "{} 레벨 {}의 {}가 탄생했습니다!",
        player_name, player_level, player_class
    );

    println!();

    // 인자 번호로 순서 지정
    println!(
        "{1} 레벨 {0}의 {2}",
        player_level, player_name, player_class
    );
    // 출력: 철수 레벨 5의 전사
}
