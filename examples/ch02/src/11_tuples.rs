//! Example 11: Tuples
//!
//! Demonstrates:
//! - 튜플 선언과 사용
//! - 해체 (destructuring)
//! - 인덱스로 접근

fn main() {
    println!("=== 튜플 예시 ===");
    println!();

    // 위치 좌표 튜플
    let position: (i32, i32) = (10, 20);
    println!("위치: {:?}", position);

    // 해체 (destructuring)
    let (x, y) = position;
    println!("X 좌표: {}", x);
    println!("Y 좌표: {}", y);

    println!();

    // 다양한 타입의 튜플
    let player: (&str, u32, u32, i32, i32) = ("철수", 100, 50, 15, 5);
    println!("플레이어 정보: {:?}", player);
    println!("이름: {}", player.0);
    println!("HP: {}", player.1);
    println!("MP: {}", player.2);
    println!("공격력: {}", player.3);
    println!("방어력: {}", player.4);

    println!();

    // 튜플 활용: 함수가 여러 값 반환
    fn calculate_stats(base_hp: u32, level: u32) -> (u32, u32, u32) {
        let hp = base_hp + (level * 10);
        let mp = level * 5;
        let exp = level * 100;
        (hp, mp, exp)
    }

    let (hp, mp, exp) = calculate_stats(100, 5);
    println!("=== 레벨 5 스탯 ===");
    println!("HP: {}", hp);
    println!("MP: {}", mp);
    println!("경험치: {}", exp);

    println!();

    // 유닛 타입
    let unit: () = ();
    println!("유닛 타입: {:?}", unit);

    fn do_nothing() {
        // 아무것도 반환 안 함
    }

    do_nothing();
    println!("do_nothing() 호출 완료");
}
