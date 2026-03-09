//! Example 9: Game Title Screen (Main Exercise)
//!
//! 제1주 메인 실습: 게임 제목 화면 구현
//!
//! Demonstrates:
//! - println!을 활용한 텍스트 UI 구성
//! - 문자열 정렬과 장식선 활용
//! - 여러 줄 출력으로 화면 구성

/// 게임 진입점
fn main() {
    // 상단 장식선
    println!("====================================");

    // 게임 제목 (가운데 정렬)
    println!("         DUNGEON CRAWLER");

    // 하단 장식선
    println!("====================================");
    println!();

    // 개발자 정보 (학생들이 본인 정보로 수정)
    println!("Developed by: 홍길동 (202412345)");
    println!("             컴퓨터 공학과");
    println!();

    // 시작 메시지
    println!("Press Enter to start...");
}
