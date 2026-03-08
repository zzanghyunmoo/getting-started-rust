fn main() {
    // ANSI 색상 코드
    let yellow = "\x1b[33m";
    let cyan = "\x1b[36m";
    let reset = "\x1b[0m";

    // 상단 장식선 (노란색)
    println!("{}===================================={}", yellow, reset);

    // 게임 제목 (노란색)
    println!("{}         DUNGEON CRAWLER{}", yellow, reset);

    // 하단 장식선 (노란색)
    println!("{}===================================={}", yellow, reset);
    println!();

    // 개발자 정보
    println!("Developed by: 짱현무 (202412345)");
    println!("             컴퓨터 공학과");
    println!();

    // 버전 정보 (청록색)
    println!("{}Version: 1.0.0-alpha{}", cyan, reset);
    println!();

    // 시작 메시지
    println!("Press Enter to start...");
}
