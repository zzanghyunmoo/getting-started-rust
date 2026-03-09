//! Example 2: Function Analysis
//!
//! 함수 구조를 상세히 분석합니다.
//!
//! Demonstrates:
//! - fn 키워드와 함수 정의
//! - main 함수의 특별한 의미
//! - 매개변수와 반환 타입

/// 프로그램의 진입점
///
/// # Note
/// main 함수는 항상 첫 번째로 실행됩니다.
fn main() {
    println!("함수 분석 예제");

    // 함수 호출 예시
    greet();
}

/// 간단한 인사 함수
fn greet() {
    println!("안녕하세요, Rust!");
}
