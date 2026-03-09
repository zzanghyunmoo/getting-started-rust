//! Example 6: Debug Format
//!
//! 디버그 포맷을 사용하여 복잡한 데이터를 출력합니다.
//!
//! Demonstrates:
//! - :? Debug 포맷
//! - :#? 예쁜 Debug 포맷

fn main() {
    // 튜플 출력
    println!("=== 튜플 출력 ===");
    println!("디버그: {:?}", (42, "hello", true));

    println!();

    // 배열 출력
    println!("=== 배열 출력 ===");
    let numbers = [1, 2, 3, 4, 5];
    println!("디버그: {:?}", numbers);
    println!("예쁜 출력: {:#?}", numbers);

    println!();

    // 게임 데이터 예시
    println!("=== 게임 데이터 ===");
    let player_stats = ("철수", 5, 100, 50);
    println!("플레이어 정보: {:?}", player_stats);

    let party = ["전사", "마법사", "도적"];
    println!("파티 구성: {:#?}", party);
}
