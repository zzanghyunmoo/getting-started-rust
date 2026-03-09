# 제1주 예제 코드 구현 계획 (Ch01 Example Code Implementation Plan)

## 개요 (Overview)

본 계획서는 [docs/ch01.md](../../../docs/ch01.md)와 [context/ch01](../)의 연구 내용을 바탕으로, 제1주 "프로그래밍과 컴퓨팅 개론"의 예제 코드 구현을 위한 상세 가이드를 제공합니다.

**목표**: 첫 번째 Rust 프로그램 작성과 게임 제목 화면 구현을 위한 예제 코드 작성

**예상 소요 시간**: 1-2시간

---

## 섹션 1: 프로젝트 구조 설정 (Project Structure Setup)

### 1.1 디렉토리 생성

```bash
mkdir -p examples/ch01/src
```

### 1.2 Cargo.toml 작성

**파일**: [examples/ch01/Cargo.toml](../../examples/ch01/Cargo.toml)

```toml
[package]
name = "ch01-examples"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]

# Hello World
[[bin]]
name = "01_hello_world"
path = "src/01_hello_world.rs"

# Function Analysis
[[bin]]
name = "02_function_analysis"
path = "src/02_function_analysis.rs"

# Println Basics
[[bin]]
name = "03_println_basics"
path = "src/03_println_basics.rs"

# String Interpolation
[[bin]]
name = "04_string_interpolation"
path = "src/04_string_interpolation.rs"

# Formatting Options
[[bin]]
name = "05_formatting_options"
path = "src/05_formatting_options.rs"

# Debug Format
[[bin]]
name = "06_debug_format"
path = "src/06_debug_format.rs"

# Special Characters
[[bin]]
name = "07_special_characters"
path = "src/07_special_characters.rs"

# Comments
[[bin]]
name = "08_comments"
path = "src/08_comments.rs"

# Main Exercise: Game Title Screen
[[bin]]
name = "09_game_title_screen"
path = "src/09_game_title_screen.rs"

# Bonus: Colored Title Screen
[[bin]]
name = "10_colored_title_screen"
path = "src/10_colored_title_screen.rs"
```

---

## 섹션 2: 예제 파일별 구현 계획 (Example Implementation Plans)

### 2.1 Example 01: Hello World

**파일**: [examples/ch01/src/01_hello_world.rs](../../examples/ch01/src/01_hello_world.rs)

**목표**: 가장 기본적인 Rust 프로그램 구조 이해

**구현할 내용**:
```rust
//! Example 1: Hello World
//!
//! 첫 번째 Rust 프로그램입니다.
//! 프로그램의 진입점과 기본 출력을 학습합니다.

fn main() {
    println!("Hello, world!");
}
```

**학습 포인트**:
- `fn main()` 함수가 프로그램의 진입점임
- `println!`은 매크로임 (함수가 아님, `!` 표시)
- 세미콜론(`;`)으로 문장 종료

---

### 2.2 Example 02: Function Analysis

**파일**: [examples/ch01/src/02_function_analysis.rs](../../examples/ch01/src/02_function_analysis.rs)

**목표**: 함수 구조 상세 분석

**구현할 내용**:
```rust
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
```

**학습 포인트**:
- `fn` 키워드로 함수 정의
- `main()`은 특별한 함수 이름
- 함수 내에서 다른 함수 호출 가능

---

### 2.3 Example 03: Println Basics

**파일**: [examples/ch01/src/03_println_basics.rs](../../examples/ch01/src/03_println_basics.rs)

**목표**: println! 매크로 기본 사용법

**구현할 내용**:
```rust
//! Example 3: Println Basics
//!
//! println! 매크로의 기본 사용법을 학습합니다.
//!
//! Demonstrates:
//! - 간단한 문자열 출력
//! - 여러 줄 출력
//! - 빈 줄 출력

fn main() {
    // 간단한 문자열 출력
    println!("Hello, world!");

    // 여러 줄 출력
    println!("첫 번째 줄");
    println!("두 번째 줄");

    // 빈 줄 출력
    println!();  // 빈 줄
    println!("빈 줄 후에 출력");
}
```

**학습 포인트**:
- `println!()`로 빈 줄 출력
- 한 줄에 하나의 `println!` 호출

---

### 2.4 Example 04: String Interpolation

**파일**: [examples/ch01/src/04_string_interpolation.rs](../../examples/ch01/src/04_string_interpolation.rs)

**목표**: 문자열 보간 (변수 값 출력)

**구현할 내용**:
```rust
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
    println!("{} 레벨 {}의 {}가 탄생했습니다!", player_name, player_level, player_class);

    println!();

    // 인자 번호로 순서 지정
    println!("{1} 레벨 {0}의 {2}", player_level, player_name, player_class);
    // 출력: 철수 레벨 5의 전사
}
```

**학습 포인트**:
- `{}` 플레이스홀더 사용
- 여러 변수를 한 번에 출력
- `{0}`, `{1}`로 순서 지정

---

### 2.5 Example 05: Formatting Options

**파일**: [examples/ch01/src/05_formatting_options.rs](../../examples/ch01/src/05_formatting_options.rs)

**목표**: 다양한 포맷팅 옵션

**구현할 내용**:
```rust
//! Example 5: Formatting Options
//!
//! 다양한 포맷팅 옵션을 학습합니다.
//!
//! Demonstrates:
//! - 다양한 진법 (2, 8, 10, 16진수)
//! - 정렬과 패딩
//! - 소수점 정밀도

fn main() {
    let number = 42;
    let pi = 3.14159;
    let gold = 12345;

    println!("=== 진법 변환 ===");
    println!("10진수: {}", number);
    println!("2진수: {:b}", number);
    println!("8진수: {:o}", number);
    println!("16진수(소문자): {:x}", number);
    println!("16진수(대문자): {:X}", number);

    println!();
    println!("=== 정렬과 패딩 ===");
    println!("오른쪽 정렬: |{:5}|", 12);
    println!("왼쪽 정렬: |{:<5}|", 12);
    println!("가운데 정렬: |{:^5}|", 12);
    println!("0으로 패딩: {:05}", 12);

    println!();
    println!("=== 게임에서의 활용 ===");
    println!("골드: {:>8}G", gold);      // 오른쪽 정렬로 숫자 정리
    println!("HP: {:>3} / {:>3}", 100, 100);
    println!("MP: {:>3} / {:>3}", 50, 50);

    println!();
    println!("=== 소수점 정밀도 ===");
    println!("기본: {}", pi);
    println!("소수점 2자리: {:.2}", pi);
    println!("소수점 4자리: {:.4}", pi);
}
```

**학습 포인트**:
- `{:b}`, `{:o}`, `{:x}`, `{:X}` 진법 변환
- `:>5`, `:<5`, `:^5` 정렬
- `:.2` 소수점 정밀도

---

### 2.6 Example 06: Debug Format

**파일**: [examples/ch01/src/06_debug_format.rs](../../examples/ch01/src/06_debug_format.rs)

**목표**: 디버그 포맷

**구현할 내용**:
```rust
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
```

**학습 포인트**:
- `{:?}`로 간단한 디버그 출력
- `{:#?}`로 예쁜 출력 (여러 줄)

---

### 2.7 Example 07: Special Characters

**파일**: [examples/ch01/src/07_special_characters.rs](../../examples/ch01/src/07_special_characters.rs)

**목표**: 특수 문자와 원시 문자열

**구현할 내용**:
```rust
//! Example 7: Special Characters
//!
//! 특수 문자와 원시 문자열을 학습합니다.
//!
//! Demonstrates:
//! - 이스케이프 시퀀스
//! - 원시 문자열 (raw string)

fn main() {
    println!("=== 이스케이프 시퀀스 ===");
    println!("줄바꿈\\n입니다");
    println!("탭\\t입니다");
    println!("큰따옴표\\\"입니다");
    println!("백슬래시\\\\입니다");

    println!();

    // 실제 이스케이프 문자 적용
    println!("실제 적용:");
    println!("줄바꿈\n입니다");
    println!("탭\t입니다");
    println!("큰따옴표\"입니다");
    println!("백슬래시\\입니다");

    println!();
    println!("=== 원시 문자열 (Raw String) ===");
    println!(r"C:\Users\name");
    println!(r#"이것은 "따옴표" 포함"#);

    println!();
    println!("=== 게임 메시지 예시 ===");
    println!("공격! 데미지: {}\n\\n이스케이프 연습", 100);
    println!(r"경로: C:\Games\Dungeon\Crawler");
}
```

**학습 포인트**:
- `\n`, `\t`, `\\`, `\"` 이스케이프 시퀀스
- `r"..."` 원시 문자열 (이스케이프 처리 안 함)
- `r#"..."#` 따옴표 포함 원시 문자열

---

### 2.8 Example 08: Comments

**파일**: [examples/ch01/src/08_comments.rs](../../examples/ch01/src/08_comments.rs)

**목표**: 주석 작성법

**구현할 내용**:
```rust
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
    const MAX_HP: u32 = 100;
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
```

**학습 포인트**:
- `//` 한 줄 주석
- `/* */` 여러 줄 주석
- `///` 항목 문서 주석
- `//!` 모듈 문서 주석

---

### 2.9 Example 09: Game Title Screen (Main Exercise)

**파일**: [examples/ch01/src/09_game_title_screen.rs](../../examples/ch01/src/09_game_title_screen.rs)

**목표**: 메인 실습 - 게임 제목 화면 구현

**구현할 내용**:
```rust
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
```

**실행 결과**:
```
====================================
         DUNGEON CRAWLER
====================================

Developed by: 홍길동 (202412345)
             컴퓨터 공학과

Press Enter to start...
```

**학습 포인트**:
- 여러 `println!` 호출로 화면 구성
- 빈 `println!()`로 줄 간격 조절
- 공백으로 가운데 정렬

---

### 2.10 Example 10: Colored Title Screen (Bonus)

**파일**: [examples/ch01/src/10_colored_title_screen.rs](../../examples/ch01/src/10_colored_title_screen.rs)

**목표**: 도전 과제 - 색상 추가

**구현할 내용**:
```rust
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
    let yellow = "\x1b[33m";    // 노란색
    let cyan = "\x1b[36m";      // 청록색
    let green = "\x1b[32m";     // 초록색
    let reset = "\x1b[0m";      // 리셋

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
```

**ANSI 색상 코드 참고**:
```
\x1b[30m - 검정
\x1b[31m - 빨강
\x1b[32m - 초록
\x1b[33m - 노랑
\x1b[34m - 파랑
\x1b[35m - 마젠타
\x1b[36m - 청록
\x1b[37m - 흰색
\x1b[0m  - 리셋
```

---

## 섹션 3: main.rs 작성

**파일**: [examples/ch01/src/main.rs](../../examples/ch01/src/main.rs)

```rust
//! Chapter 1: Programming and Computing Introduction - Examples
//!
//! 이 모듈은 제1주 "프로그래밍과 컴퓨팅 개론"의 모든 예제를 포함합니다.
//!
//! # 실행 방법
//!
//! 개별 예제 실행:
//! ```bash
//! cargo run --bin 01_hello_world
//! cargo run --bin 02_function_analysis
//! cargo run --bin 03_println_basics
//! cargo run --bin 04_string_interpolation
//! cargo run --bin 05_formatting_options
//! cargo run --bin 06_debug_format
//! cargo run --bin 07_special_characters
//! cargo run --bin 08_comments
//! cargo run --bin 09_game_title_screen
//! cargo run --bin 10_colored_title_screen
//! ```

fn main() {
    println!("====================================");
    println!("Chapter 1: Rust 시작하기");
    println!("Programming and Computing Introduction");
    println!("====================================");
    println!();
    println!("예제 목록:");
    println!("  01. Hello World");
    println!("  02. Function Analysis");
    println!("  03. Println Basics");
    println!("  04. String Interpolation");
    println!("  05. Formatting Options");
    println!("  06. Debug Format");
    println!("  07. Special Characters");
    println!("  08. Comments");
    println!("  09. Game Title Screen (Main Exercise)");
    println!("  10. Colored Title Screen (Bonus)");
    println!();
    println!("개별 예제 실행:");
    println!("  cargo run --bin 01_hello_world");
    println!("  cargo run --bin 09_game_title_screen");
}
```

---

## 섹션 4: 빌드 및 검증 (Build and Verification)

### 4.1 빌드

```bash
cd examples/ch01
cargo build
```

### 4.2 개별 실행 테스트

```bash
# 모든 예제 실행 확인
cargo run --bin 01_hello_world
cargo run --bin 02_function_analysis
cargo run --bin 03_println_basics
cargo run --bin 04_string_interpolation
cargo run --bin 05_formatting_options
cargo run --bin 06_debug_format
cargo run --bin 07_special_characters
cargo run --bin 08_comments
cargo run --bin 09_game_title_screen
cargo run --bin 10_colored_title_screen
```

### 4.3 코드 포맷팅

```bash
cargo fmt
```

### 4.4 린트 검사

```bash
cargo clippy -- -D warnings
```

---

## 섹션 5: examples/README.md 업데이트

**파일**: [examples/README.md](../../examples/README.md) 수정

Ch01 상태 업데이트:
```markdown
| 챕터 | 주제 | 상태 |
|------|------|------|
| Ch01 | Rust 시작하기 | ✅ 완료 |
| Ch02 | 변수와 기본 타입 | ✅ 완료 |
```

Ch01 실행 명령어 추가:
```markdown
### 제1주: Rust 시작하기

```bash
cd examples/ch01

# 전체 예제 목록
cargo run

# 개별 예제 실행
cargo run --bin 01_hello_world
cargo run --bin 09_game_title_screen
```
```

---

## 섹션 6: Git 커밋 메시지

```bash
cd examples/ch01
git add .
git commit -m "feat: Add Chapter 1 examples

- 01_hello_world: Basic Rust program structure
- 02_function_analysis: Function definition and main entry point
- 03_println_basics: Basic println! usage
- 04_string_interpolation: Variable output with placeholders
- 05_formatting_options: Number bases, alignment, and precision
- 06_debug_format: Debug output for complex types
- 07_special_characters: Escape sequences and raw strings
- 08_comments: Various comment types
- 09_game_title_screen: Main exercise - game title screen
- 10_colored_title_screen: Bonus - colored output with ANSI codes
"
```

---

## 요약 및 검증 체크리스트

### 구현 완료 후 확인 사항

- [ ] `examples/ch01/Cargo.toml` 작성 완료
- [ ] `examples/ch01/src/main.rs` 작성 완료
- [ ] `examples/ch01/src/01_hello_world.rs` 작성 완료
- [ ] `examples/ch01/src/02_function_analysis.rs` 작성 완료
- [ ] `examples/ch01/src/03_println_basics.rs` 작성 완료
- [ ] `examples/ch01/src/04_string_interpolation.rs` 작성 완료
- [ ] `examples/ch01/src/05_formatting_options.rs` 작성 완료
- [ ] `examples/ch01/src/06_debug_format.rs` 작성 완료
- [ ] `examples/ch01/src/07_special_characters.rs` 작성 완료
- [ ] `examples/ch01/src/08_comments.rs` 작성 완료
- [ ] `examples/ch01/src/09_game_title_screen.rs` 작성 완료
- [ ] `examples/ch01/src/10_colored_title_screen.rs` 작성 완료
- [ ] `cargo build` 성공
- [ ] 모든 예제 개별 실행 성공
- [ ] `cargo fmt` 포맷팅 완료
- [ ] `cargo clippy` 린트 통과
- [ ] `examples/README.md` 업데이트 완료

---

## 참고 자료

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example - println!](https://doc.rust-lang.org/rust-by-example/hello/print.html)
- [std::fmt documentation](https://doc.rust-lang.org/std/fmt/)

---

*작성일: 2026-03-09*
*버전: 1.0*
