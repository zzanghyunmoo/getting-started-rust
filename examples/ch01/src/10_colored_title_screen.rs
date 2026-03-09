//! Example 10: Colored Title Screen (Bonus)
//!
//! 도전 과제: ANSI 색상 코드를 사용한 컬러 출력
//!
//! Demonstrates:
//! - ANSI 색상 코드 사용
//! - 색상 코드와 리셋 코드

/// 게임 진입점
fn main() {
    // ANSI 색상 코드
    let yellow = "\x1b[33m"; // 노란색
    let cyan = "\x1b[36m"; // 청록색
    let green = "\x1b[32m"; // 초록색
    let reset = "\x1b[0m"; // 리셋

    // 상단 장식선 (노란색)
    println!("{}===================================={}", yellow, reset);

    // 게임 제목 (노란색)
    println!("{}         DUNGEON CRAWLER{}", yellow, reset);

    // 하단 장식선 (노란색)
    println!("{}===================================={}", yellow, reset);
    println!();

    // 개발자 정보
    println!("Developed by: 홍길동 (202412345)");
    println!("             컴퓨터 공학과");
    println!();

    // 버전 정보 (청록색)
    println!("{}Version: 1.0.0-alpha{}", cyan, reset);
    println!();

    // 시작 메시지 (초록색)
    println!("{}Press Enter to start...{}", green, reset);
}
