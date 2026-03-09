//! Example 8: Comments
//!
//! 다양한 주석 작성법을 학습합니다.
//!
//! Demonstrates:
//! - 한 줄 주석 (//)
//! - 여러 줄 주석 (/* */)
//! - 문서 주석 (///, //!)

/// 게임 캐릭터 정보를 담는 구조체 (예시)
///
/// # Fields
///
/// * `name` - 캐릭터 이름
/// * `level` - 캐릭터 레벨
///
/// # Example
///
/// ```
/// let player = CharacterInfo {
///     name: "철수",
///     level: 5,
/// };
/// ```
#[allow(dead_code)]
struct CharacterInfo {
    name: &'static str,
    level: u32,
}

fn main() {
    // 한 줄 주석
    let player_name = "철수"; // 변수 뒤에도 주석 가능

    /*
     * 여러 줄 주석
     * 게임 설정 관련 상수들
     */
    #[allow(dead_code)]
    const MAX_HP: u32 = 100;
    #[allow(dead_code)]
    const MAX_MP: u32 = 50;

    /// 플레이어 정보 출력 함수
    fn print_player_info(name: &str, level: u32) {
        println!("캐릭터: {} (Lv.{})", name, level);
    }

    print_player_info(player_name, 5);

    // TODO: 인벤토리 시스템 구현
    // FIXME: 레벨업 로직 수정 필요
    // NOTE: 테스트용 데이터
}
