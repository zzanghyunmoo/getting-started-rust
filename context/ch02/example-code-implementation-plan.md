# 제2장 예제 코드 구현 계획

## Context

본 문서는 `docs/ch02.md`의 내용을 기반으로 **예제 프로젝트와 예제 코드**를 구현하기 위한 계획입니다. Chapter 2는 "변수와 기본 타입"을 다루며, 학습자가 Dungeon Crawler 게임의 캐릭터 생성 시스템을 구현하며 Rust의 변수 시스템과 타입 시스템을 학습합니다.

**작성 대상:** 예제 프로젝트 및 예제 코드 구현

---

## Overview

### 목표

1. **examples/ch02** 디렉토리에 Chapter 2 관련 예제 코드 생성
2. 각 예제는 독립적으로 실행 가능하도록 구성
3. 예제는 docs/ch02.md의 순서와 구조를 따름
4. 각 예제는 해당 개념을 명확하게 보여주고 게임 컨텍스트와 연결

### 프로젝트 구조

```
getting-started-rust/
├── examples/
│   └── ch02/                          # Chapter 2 예제 디렉토리
│       ├── Cargo.toml                 # 예제 프로젝트 설정
│       └── src/
│           ├── main.rs                # 메인 진입점
│           ├── 01_variables.rs        # 변수 선언과 불변성
│           ├── 02_mutability.rs       # 가변 변수
│           ├── 03_constants.rs        # 상수
│           ├── 04_shadowing.rs        # 섀도잉
│           ├── 05_integer_types.rs    # 정수형 타입
│           ├── 06_float_types.rs      # 부동소수점 타입
│           ├── 07_boolean.rs          # 불리언 타입
│           ├── 08_character.rs        # 문자 타입
│           ├── 09_type_inference.rs   # 타입 추론
│           ├── 10_type_conversion.rs  # 타입 변환
│           ├── 11_tuples.rs           # 튜플
│           ├── 12_arrays.rs           # 배열
│           └── 13_character_creation.rs  # 실습: 캐릭터 생성
├── docs/
│   └── ch02.md                        # Chapter 2 문서
└── dungeon_crawler/                   # 메인 프로젝트
```

---

## Phase 1: 예제 프로젝트 설정

### 1.1 프로젝트 생성

**파일:** `examples/ch02/Cargo.toml`

```toml
[package]
name = "ch02-examples"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
```

### 1.2 메인 진입점

**파일:** `examples/ch02/src/main.rs`

```rust
//! Chapter 2: Variables and Basic Types - Examples
//!
//! This module contains all examples from Chapter 2 of the Rust tutorial.

// Examples are organized by topic
mod 01_variables;
mod 02_mutability;
mod 03_constants;
mod 04_shadowing;
mod 05_integer_types;
mod 06_float_types;
mod 07_boolean;
mod 08_character;
mod 09_type_inference;
mod 10_type_conversion;
mod 11_tuples;
mod 12_arrays;
mod 13_character_creation;

fn main() {
    println!("====================================");
    println!("Chapter 2 Examples");
    println!("Variables and Basic Types");
    println!("====================================");
    println!();
    println!("Run individual examples with:");
    println!("  cargo run --bin 01_variables");
    println!("  cargo run --bin 02_mutability");
    println!("  ...");
}
```

---

## Phase 2: 개념별 예제 구현

### 2.1 변수 선언과 불변성 (01_variables.rs)

**파일:** `examples/ch02/src/01_variables.rs`

```rust
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
```

### 2.2 가변 변수 (02_mutability.rs)

**파일:** `examples/ch02/src/02_mutability.rs`

```rust
//! Example 2: Mutable Variables
//!
//! Demonstrates:
//! - mut 키워드로 가변 변수 선언
//! - 값 변경 가능
//! - 게임 상태 변화 예시

fn main() {
    println!("=== 가변 변수 예시 ===");
    println!();

    // 가변 변수로 HP 선언
    let mut player_hp = 100;
    println!("초기 HP: {}", player_hp);

    // 전투에서 피해를 입음
    player_hp = 85;
    println!("전투 후 HP: {}", player_hp);

    // 포션으로 회복
    player_hp = 100;
    println!("회복 후 HP: {}", player_hp);

    println!();
    println!("=== 불변 변수와 비교 ===");

    // 불변 변수는 변경 불가능
    let max_hp = 100;
    println!("최대 HP: {}", max_hp);
    // max_hp = 150;  // 컴파일 에러!
}
```

### 2.3 상수 (03_constants.rs)

**파일:** `examples/ch02/src/03_constants.rs`

```rust
//! Example 3: Constants
//!
//! Demonstrates:
//! - const 키워드로 상수 선언
//! - 타입 어노테이션 필수
//! - 컴파일 타임 상수

/// 게임 밸런스 상수
const MAX_HP: u32 = 100;
const MAX_MP: u32 = 50;
const STARTING_GOLD: u32 = 100;
const LEVEL_UP_EXP: u32 = 100;

/// 직업별 스탯 상수
const WARRIOR_HP: u32 = 100;
const WARRIOR_MP: u32 = 50;
const WARRIOR_ATTACK: i32 = 15;
const WARRIOR_DEFENSE: i32 = 5;

const MAGE_HP: u32 = 60;
const MAGE_MP: u32 = 100;
const MAGE_ATTACK: i32 = 10;
const MAGE_DEFENSE: i32 = 3;

const ROGUE_HP: u32 = 80;
const ROGUE_MP: u32 = 60;
const ROGUE_ATTACK: i32 = 12;
const ROGUE_DEFENSE: i32 = 8;

fn main() {
    println!("=== 게임 밸런스 설정 ===");
    println!("최대 HP: {}", MAX_HP);
    println!("최대 MP: {}", MAX_MP);
    println!("초기 골드: {}G", STARTING_GOLD);
    println!("레벨업 경험치: {}", LEVEL_UP_EXP);

    println!();
    println!("=== 직업별 스탯 ===");
    println!("전사: HP {} MP {} 공격력 {} 방어력 {}",
        WARRIOR_HP, WARRIOR_MP, WARRIOR_ATTACK, WARRIOR_DEFENSE);
    println!("마법사: HP {} MP {} 공격력 {} 방어력 {}",
        MAGE_HP, MAGE_MP, MAGE_ATTACK, MAGE_DEFENSE);
    println!("도적: HP {} MP {} 공격력 {} 방어력 {}",
        ROGUE_HP, ROGUE_MP, ROGUE_ATTACK, ROGUE_DEFENSE);
}
```

### 2.4 섀도잉 (04_shadowing.rs)

**파일:** `examples/ch02/src/04_shadowing.rs`

```rust
//! Example 4: Shadowing
//!
//! Demonstrates:
//! - 섀도잉으로 같은 이름의 새 변수 선언
//! - 타입 변경 가능
//! - 데이터 변환 파이프라인

fn main() {
    println!("=== 섀도잉 기본 예시 ===");

    let x = 5;
    println!("초기 x: {}", x);

    let x = x + 1;   // 섀도잉: x는 6
    println!("섀도잉 1 (x + 1): {}", x);

    let x = x * 2;   // 또 다른 섀도잉: x는 12
    println!("섀도잉 2 (x * 2): {}", x);

    println!();
    println!("=== 섀도잉으로 타입 변경 ===");

    let spaces = "   ";           // &str 타입
    println!("문자열 공백: '{}'", spaces);

    let spaces = spaces.len();    // usize 타입으로 변경
    println!("공백 개수: {}", spaces);

    println!();
    println!("=== 데이터 변환 파이프라인 ===");

    // 1단계: 입력 (공백 포함)
    let name = "  철수  ";
    println!("1. 입력: '{}'", name);

    // 2단계: 섀도잉 - 공백 제거
    let name = name.trim();
    println!("2. trim(): '{}'", name);

    // 3단계: 섀도잉 - 유효성 검사 후 기본값
    let name = if name.is_empty() {
        "모험가"
    } else {
        name
    };
    println!("3. 최종 이름: {}", name);

    println!();
    println!("=== 스코프와 섀도잉 ===");

    let x = 5;           // 스코프 1: x는 5
    println!("외부 x: {}", x);

    {
        let x = x * 2;   // 스코프 2: 내부 섀도잉, x는 10
        println!("내부 x: {}", x);

        {
            let x = x + 1;  // 스코프 3: 더 내부, x는 11
            println!("더 내부 x: {}", x);
        }
    }

    println!("다시 외부 x: {}", x);
}
```

### 2.5 정수형 타입 (05_integer_types.rs)

**파일:** `examples/ch02/src/05_integer_types.rs`

```rust
//! Example 5: Integer Types
//!
//! Demonstrates:
//! - 부호 있는 정수형 (i8, i16, i32, i64, i128, isize)
//! - 부호 없는 정수형 (u8, u16, u32, u64, u128, usize)
//! - 게임에서의 활용 예시

fn main() {
    println!("=== 정수형 타입 예시 ===");
    println!();

    // 플레이어 스탯 (음수 없음 - unsigned)
    let hp: u32 = 100;
    let mp: u32 = 50;
    let gold: u32 = 150;

    println!("=== Unsigned Integers (음수 없음) ===");
    println!("HP: {} (u32)", hp);
    println!("MP: {} (u32)", mp);
    println!("Gold: {}G (u32)", gold);

    println!();

    // 공격력 (음수 가능: 디버프 - signed)
    let attack: i32 = 15;
    let defense: i32 = 5;
    let attack_buff: i32 = 5;
    let attack_debuff: i32 = -3;

    println!("=== Signed Integers (음수 가능) ===");
    println!("기본 공격력: {} (i32)", attack);
    println!("방어력: {} (i32)", defense);
    println!("공격력 버프: +{} (i32)", attack_buff);
    println!("공격력 디버프: {} (i32)", attack_debuff);

    println!();

    // 경험치 (매우 큰 값 가능)
    let exp: u64 = 1_234_567_890;
    let total_exp: u128 = 3_000_000_000_000;

    println!("=== Large Integers ===");
    println!("현재 경험치: {} (u64)", exp);
    println!("총 경험치: {} (u128)", total_exp);

    println!();

    // 정수 리터럴 표기법
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("=== 정수 리터럴 표기법 ===");
    println!("10진수: {}", decimal);
    println!("16진수: {} (0xff)", hex);
    println!("8진수: {} (0o77)", octal);
    println!("2진수: {} (0b1111_0000)", binary);
    println!("바이트: {} (b'A')", byte);
}
```

### 2.6 부동소수점 타입 (06_float_types.rs)

**파일:** `examples/ch02/src/06_float_types.rs`

```rust
//! Example 6: Floating-Point Types
//!
//! Demonstrates:
//! - f32 (단일 정밀도)
//! - f64 (배정밀도, 기본값)
//! - 데미지 계산 예시

fn main() {
    println!("=== 부동소수점 타입 예시 ===");
    println!();

    // f64로 자동 추론
    let damage = 15.5;
    let heal_amount: f32 = 25.0;

    println!("데미지: {} (f64)", damage);
    println!("회복량: {} (f32)", heal_amount);

    println!();

    // 계산 예시
    let current_hp: f64 = 80.0;
    let max_hp: f64 = 100.0;
    let final_hp = current_hp - damage + heal_amount as f64;

    println!("=== HP 계산 ===");
    println!("현재 HP: {}", current_hp);
    println!("받은 데미지: -{}", damage);
    println!("회복량: +{}", heal_amount as f64);
    println!("최종 HP: {}", final_hp);

    println!();

    // 퍼센트 계산
    let hp_percent = (current_hp / max_hp) * 100.0;
    println!("HP 퍼센트: {:.1}%", hp_percent);

    println!();

    // 크리티컬 히트 계산 (1.5배 데미지)
    let base_damage = 20.0;
    let critical_multiplier = 1.5;
    let critical_damage = base_damage * critical_multiplier;

    println!("=== 크리티컬 시스템 ===");
    println!("기본 데미지: {}", base_damage);
    println!("크리티컬 배수: {}x", critical_multiplier);
    println!("크리티컬 데미지: {}", critical_damage);
}
```

### 2.7 불리언 타입 (07_boolean.rs)

**파일:** `examples/ch02/src/07_boolean.rs`

```rust
//! Example 7: Boolean Type
//!
//! Demonstrates:
//! - bool 타입 (true, false)
//! - 논리 연산 (&&, ||, !)
//! - 게임 상태 플래그

fn main() {
    println!("=== 불리언 타입 예시 ===");
    println!();

    // 게임 상태 플래그
    let is_alive: bool = true;
    let is_poisoned: bool = false;
    let has_key: bool = true;
    let has_treasure: bool = false;

    println!("=== 플레이어 상태 ===");
    println!("생존 여부: {}", is_alive);
    println!("중독 상태: {}", is_poisoned);
    println!("키 보유: {}", has_key);
    println!("보물 획득: {}", has_treasure);

    println!();

    // 논리 연산
    println!("=== 논리 연산 ===");

    // AND 연산: 둘 다 true면 true
    if is_alive && has_key {
        println!("문을 열 수 있습니다! (생존 && 키 보유)");
    }

    // OR 연산: 하나라도 true면 true
    if is_poisoned || !has_treasure {
        println!("아직 던전을 탐험해야 합니다!");
    }

    // NOT 연산: true/false 반전
    println!("키 보유 안 함: {}", !has_key);
    println!("보물 미획득: {}", !has_treasure);

    println!();

    // 복합 조건
    let can_open_door = is_alive && has_key && !is_poisoned;
    println!("문 열기 가능 (생존 && 키 && !중독): {}", can_open_door);

    // 전투 가능 여부
    let can_fight = is_alive && !is_poisoned;
    println!("전투 가능 (생존 && !중독): {}", can_fight);
}
```

### 2.8 문자 타입 (08_character.rs)

**파일:** `examples/ch02/src/08_character.rs`

```rust
//! Example 8: Character Type
//!
//! Demonstrates:
//! - char 타입 (4바이트 Unicode)
//! - 한글, 이모지 지원
//! - Unicode Scalar Value

fn main() {
    println!("=== 문자 타입 예시 ===");
    println!();

    // 다양한 문자 타입
    let first_char: char = '철';
    let emoji: char = '⚔️';
    let grade: char = 'A';
    let star: char = '★';
    let heart: char = '❤️';

    println!("=== 다양한 문자 ===");
    println!("이름의 첫 글자: {}", first_char);
    println!("무기 이모지: {}", emoji);
    println!("등급: {}", grade);
    println!("별: {}", star);
    println!("하트: {}", heart);

    println!();

    // 문자 vs 문자열
    println!("=== 문자 vs 문자열 ===");

    let single_char: char = 'A';        // 4 bytes, 1 character
    let string_slice: &str = "A";       // 문자열 슬라이스

    println!("char: '{}' (타입: char)", single_char);
    println!("&str: \"{}\" (타입: &str)", string_slice);

    println!();

    // 아스키 코드 변환
    let letter: char = 'A';
    let ascii_value = letter as u32;

    println!("=== 아스키 코드 ===");
    println!("문자: {}", letter);
    println!("아스키 값: {}", ascii_value);

    let b: char = 'B';
    println!("다음 문자: {} (아스키: {})", b, b as u32);

    println!();

    // 특수 문자
    let newline = '\n';
    let tab = '\t';
    let null = '\0';

    println!("=== 특수 문자 ===");
    println!("줄바꿈: '\\n' (아스키: {})", newline as u32);
    println!("탭: '\\t' (아스키: {})", tab as u32);
    println!("널 문자: '\\0' (아스키: {})", null as u32);
}
```

### 2.9 타입 추론 (09_type_inference.rs)

**파일:** `examples/ch02/src/09_type_inference.rs`

```rust
//! Example 9: Type Inference
//!
//! Demonstrates:
//! - 컴파일러의 타입 추론
//! - 타입 어노테이션이 필요한 경우
//! - 추론의 한계

fn main() {
    println!("=== 타입 추론 예시 ===");
    println!();

    // 컴파일러가 자동으로 타입 추론
    let hp = 100;        // i32로 추론
    let damage = 15.5;   // f64로 추론
    let is_alive = true; // bool로 추론

    println!("=== 자동 추론 ===");
    println!("HP: {} (타입: i32)", hp);
    println!("데미지: {} (타입: f64)", damage);
    println!("생존: {} (타입: bool)", is_alive);

    println!();

    // 타입 어노테이션이 필요한 경우
    println!("=== 타입 어노테이션 필요 ===");

    // 1. 모호한 경우 (제네릭 메서드)
    let guess: u32 = "42".parse().expect("숫자가 아닙니다!");
    println!("추측: {} (u32)", guess);

    // 2. 특정 타입으로 지정
    let max_hp: u32 = 100;  // u32로 명시
    let large_number: i64 = 100;  // i64로 명시
    println!("최대 HP: {} (u32)", max_hp);
    println!("큰 수: {} (i64)", large_number);

    println!();

    // 타입 추론 활용
    println!("=== 스마트한 추론 ===");

    // 벡터의 요소 타입 추론
    let numbers = vec![1, 2, 3, 4, 5];  // Vec<i32>로 추론
    println!("숫자 벡터: {:?} (Vec<i32>)", numbers);

    // 튜플의 타입 추론
    let player = ("철수", 100, 50);  // (&str, i32, i32)로 추론
    println!("플레이어: {:?} (튜플)", player);

    // 수식의 결과 타입 추론
    let result = 10 + 20;  // i32 + i32 = i32
    println!("결과: {} (i32)", result);

    let float_result = 10.0 + 20.0;  // f64 + f64 = f64
    println!("실수 결과: {} (f64)", float_result);
}
```

### 2.10 타입 변환 (10_type_conversion.rs)

**파일:** `examples/ch02/src/10_type_conversion.rs`

```rust
//! Example 10: Type Conversion
//!
//! Demonstrates:
//! - as 키워드로 명시적 타입 변환
//! - 안전한 vs 위험한 변환
//! - 암시적 변환 없음

fn main() {
    println!("=== 타입 변환 예시 ===");
    println!();

    // 기본 변환
    let hp_float: f64 = 100.0;
    let hp_int: u32 = 100;

    println!("=== 기본 변환 ===");
    println!("HP (f64): {}", hp_float);
    println!("HP (u32): {}", hp_int);

    // 명시적 변환
    let hp_total = hp_int + (hp_float as u32);
    println!("총 HP: {}", hp_total);

    println!();

    // 안전한 변환
    println!("=== 안전한 변환 ===");

    let damage: i32 = -15;
    let abs_damage = damage.abs() as u32;
    println!("데미지 (i32): {}", damage);
    println!("절대값 데미지 (u32): {}", abs_damage);

    // 크기 확대 (안전)
    let small: u8 = 100;
    let large: u32 = small as u32;
    println!("u8 {} → u32 {}", small, large);

    println!();

    // 손실 가능 변환 (주의 필요)
    println!("=== 손실 가능 변환 (주의!) ===");

    let big: u32 = 300;
    let small_u8: u8 = big as u8;  // 300 % 256 = 44
    println!("u32 {} → u8 {} (손실 발생!)", big, small_u8);

    let float_val: f64 = 3.9;
    let int_val: i32 = float_val as i32;  // 소수점 버림
    println!("f64 {} → i32 {} (소수점 손실)", float_val, int_val);

    println!();

    // 문자로 변환
    println!("=== 문자로 변환 ===");

    let num: u32 = 65;
    let char_val: char = num as char;
    println!("u32 {} → char '{}'", num, char_val);

    // 한글 유니코드
    let korean: u32 = 0xD55C;  // '한'
    let korean_char: char = korean as char;
    println!("u32 {} → char '{}'", korean, korean_char);

    println!();

    // 변환 불가능한 경우
    println!("=== 변환 호환성 ===");

    // 정수 → 실수 (가능)
    let int_val: i32 = 42;
    let float_val: f64 = int_val as f64;
    println!("i32 {} → f64 {}", int_val, float_val);

    // 실수 → 정수 (가능하지만 손실)
    let float_val: f64 = 42.7;
    let int_val: i32 = float_val as i32;
    println!("f64 {} → i32 {}", float_val, int_val);

    // char → 정수 (가능)
    let letter: char = 'A';
    let num: u32 = letter as u32;
    println!("char '{}' → u32 {}", letter, num);

    // 실수 → char (불가능, 복잡한 변환 필요)
    // let float_val: f64 = 65.0;
    // let char_val: char = float_val as char;  // 에러!
}
```

### 2.11 튜플 (11_tuples.rs)

**파일:** `examples/ch02/src/11_tuples.rs`

```rust
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

    fn do_nothing() -> () {
        // 아무것도 반환 안 함
    }

    do_nothing();
    println!("do_nothing() 호출 완료");
}
```

### 2.12 배열 (12_arrays.rs)

**파일:** `examples/ch02/src/12_arrays.rs`

```rust
//! Example 12: Arrays
//!
//! Demonstrates:
//! - 배열 선언과 사용
//! - 인덱스로 접근
//! - 같은 값으로 초기화

fn main() {
    println!("=== 배열 예시 ===");
    println!();

    // 직업별 능력치 배열
    let warrior_stats: [i32; 4] = [100, 50, 15, 5];  // HP, MP, 공격, 방어
    let mage_stats: [i32; 4] = [60, 100, 10, 3];
    let rogue_stats: [i32; 4] = [80, 60, 12, 8];

    println!("=== 직업별 스탯 ===");
    println!("전사: HP {} MP {} 공격 {} 방어 {}",
        warrior_stats[0], warrior_stats[1], warrior_stats[2], warrior_stats[3]);
    println!("마법사: HP {} MP {} 공격 {} 방어 {}",
        mage_stats[0], mage_stats[1], mage_stats[2], mage_stats[3]);
    println!("도적: HP {} MP {} 공격 {} 방어 {}",
        rogue_stats[0], rogue_stats[1], rogue_stats[2], rogue_stats[3]);

    println!();

    // 같은 값으로 초기화
    let inventory = [0; 20];  // 빈 인벤토리 20칸
    println!("인벤토리 크기: {}칸", inventory.len());
    println!("첫 번째 슬롯: {}", inventory[0]);

    println!();

    // 배열 반복
    let numbers = [10, 20, 30, 40, 50];
    println!("=== 배열 요소 ===");

    for i in 0..numbers.len() {
        println!("numbers[{}] = {}", i, numbers[i]);
    }

    println!();

    // 안전한 접근 방법
    println!("=== 안전한 접근 ===");

    match numbers.get(10) {
        Some(&value) => println!("인덱스 10: {}", value),
        None => println!("인덱스 10: 존재하지 않음"),
    }

    // 배열 길이
    println!("=== 배열 정보 ===");
    println!("길이: {}", numbers.len());
    println!("메모리 크기: {} bytes", std::mem::size_of_val(&numbers));
    println!("요소당 크기: {} bytes", std::mem::size_of::<i32>());

    println!();

    // 배열 수정 (mut)
    let mut mutable_stats = [100, 50, 15, 5];
    println!("변경 전: {:?}", mutable_stats);

    mutable_stats[0] = 90;  // HP 감소
    println!("변경 후: {:?}", mutable_stats);
}
```

### 2.13 실습: 캐릭터 생성 (13_character_creation.rs)

**파일:** `examples/ch02/src/13_character_creation.rs`

```rust
//! Example 13: Character Creation System (Practice)
//!
//! This is the main exercise from Chapter 2.
//! Demonstrates all concepts learned:
//! - Variables (mutable and immutable)
//! - Constants
//! - Type annotations
//! - Tuples for multiple values
//! - Arrays for job stats

/// 게임 밸런스 상수
const MAX_HP: u32 = 100;
const MAX_MP: u32 = 50;
const STARTING_GOLD: u32 = 100;

/// 직업별 초기 능력치 배열
const WARRIOR_STATS: [i32; 4] = [100, 50, 15, 5];  // HP, MP, 공격, 방어
const MAGE_STATS: [i32; 4] = [60, 100, 10, 3];
const ROGUE_STATS: [i32; 4] = [80, 60, 12, 8];

fn main() {
    // 1. 상수 출력
    println!("=== 게임 밸런스 ===");
    println!("최대 HP: {}", MAX_HP);
    println!("최대 MP: {}", MAX_MP);
    println!("초기 골드: {}G", STARTING_GOLD);
    println!();

    // 2. 플레이어 이름 (불변)
    let player_name = "철수";

    // 3. 직업 (불변)
    let player_class = "전사";

    // 4. 직업별 초기 능력치 (튜플로 받기)
    let (hp, mp, attack, defense) = match player_class {
        "전사" => (WARRIOR_STATS[0], WARRIOR_STATS[1], WARRIOR_STATS[2], WARRIOR_STATS[3]),
        "마법사" => (MAGE_STATS[0], MAGE_STATS[1], MAGE_STATS[2], MAGE_STATS[3]),
        "도적" => (ROGUE_STATS[0], ROGUE_STATS[1], ROGUE_STATS[2], ROGUE_STATS[3]),
        _ => (80, 50, 10, 5),  // 기본값
    };

    // 5. 캐릭터 정보 출력
    println!("====================================");
    println!("캐릭터가 생성되었습니다!");
    println!("====================================");
    println!("이름: {}", player_name);
    println!("직업: {}", player_class);
    println!("HP: {} / MP: {}", hp, mp);
    println!("공격력: {} / 방어력: {}", attack, defense);
    println!("골드: {}G", STARTING_GOLD);
    println!("====================================");

    println!();

    // 도전 과제: 직업별 스탯 비교
    println!("====================================");
    println!("직업별 스탯 비교");
    println!("====================================");
    println!("[전사]   HP:{} MP:{} 공격:{} 방어:{}",
        WARRIOR_STATS[0], WARRIOR_STATS[1], WARRIOR_STATS[2], WARRIOR_STATS[3]);
    println!("[마법사] HP:{} MP:{} 공격:{} 방어:{}",
        MAGE_STATS[0], MAGE_STATS[1], MAGE_STATS[2], MAGE_STATS[3]);
    println!("[도적]   HP:{} MP:{} 공격:{} 방어:{}",
        ROGUE_STATS[0], ROGUE_STATS[1], ROGUE_STATS[2], ROGUE_STATS[3]);
    println!("====================================");
}
```

---

## Phase 3: 실행 가능한 예제 설정

### 3.1 개별 예제 실행 설정

**Cargo.toml에 binary 추가:**

```toml
[[bin]]
name = "01_variables"
path = "src/01_variables.rs"

[[bin]]
name = "02_mutability"
path = "src/02_mutability.rs"

[[bin]]
name = "03_constants"
path = "src/03_constants.rs"

[[bin]]
name = "04_shadowing"
path = "src/04_shadowing.rs"

[[bin]]
name = "05_integer_types"
path = "src/05_integer_types.rs"

[[bin]]
name = "06_float_types"
path = "src/06_float_types.rs"

[[bin]]
name = "07_boolean"
path = "src/07_boolean.rs"

[[bin]]
name = "08_character"
path = "src/08_character.rs"

[[bin]]
name = "09_type_inference"
path = "src/09_type_inference.rs"

[[bin]]
name = "10_type_conversion"
path = "src/10_type_conversion.rs"

[[bin]]
name = "11_tuples"
path = "src/11_tuples.rs"

[[bin]]
name = "12_arrays"
path = "src/12_arrays.rs"

[[bin]]
name = "13_character_creation"
path = "src/13_character_creation.rs"
```

### 3.2 실행 방법

```bash
# 전체 예제 목록
cargo run --bin ch02-examples

# 개별 예제 실행
cargo run --bin 01_variables
cargo run --bin 02_mutability
cargo run --bin 03_constants
cargo run --bin 04_shadowing
cargo run --bin 05_integer_types
cargo run --bin 06_float_types
cargo run --bin 07_boolean
cargo run --bin 08_character
cargo run --bin 09_type_inference
cargo run --bin 10_type_conversion
cargo run --bin 11_tuples
cargo run --bin 12_arrays
cargo run --bin 13_character_creation

# 포맷팅
cargo fmt

# 린트 검사
cargo clippy

# 빌드 확인
cargo build
```

---

## Phase 4: 문서와 예제 연동

### 4.1 docs/ch02.md에 예제 참조 추가

각 섹션의 코드 예시에 해당 예제 파일에 대한 링크 추가:

```markdown
### 1.1 변수 선언 기본

예제 코드: [`01_variables.rs`](../../examples/ch02/src/01_variables.rs)

```rust
fn main() {
    let player_name = "철수";
    let player_class = "전사";
    // ...
}
```
```

---

## Implementation Steps

1. **프로젝트 생성**
   - [ ] `examples/ch02/` 디렉토리 생성
   - [ ] `Cargo.toml` 작성
   - [ ] `src/main.rs` 작성

2. **예제 파일 작성**
   - [ ] `01_variables.rs`
   - [ ] `02_mutability.rs`
   - [ ] `03_constants.rs`
   - [ ] `04_shadowing.rs`
   - [ ] `05_integer_types.rs`
   - [ ] `06_float_types.rs`
   - [ ] `07_boolean.rs`
   - [ ] `08_character.rs`
   - [ ] `09_type_inference.rs`
   - [ ] `10_type_conversion.rs`
   - [ ] `11_tuples.rs`
   - [ ] `12_arrays.rs`
   - [ ] `13_character_creation.rs`

3. **바이너리 설정**
   - [ ] `Cargo.toml`에 각 예제를 binary로 추가

4. **테스트**
   - [ ] 모든 예제 실행 가능한지 확인
   - [ ] `cargo fmt` 실행
   - [ ] `cargo clippy` 실행

5. **문서 연동**
   - [ ] `docs/ch02.md`에 예제 링크 추가

---

## Verification Plan

### 빌드 및 실행 확인

```bash
# 디렉토리 이동
cd examples/ch02

# 전체 빌드
cargo build

# 개별 실행 테스트
for i in {01..13}; do
    echo "=== Running $i ==="
    cargo run --bin $i
    echo
done

# 포맷팅 확인
cargo fmt --check

# 린트 확인
cargo clippy -- -D warnings
```

### 예제 출력 확인

각 예제의 실행 결과가 문서와 일치하는지 확인:
- [ ] 01_variables: 이름과 직업 출력
- [ ] 02_mutability: HP 변화 출력
- [ ] 03_constants: 상수값 출력
- [ ] 04_shadowing: 섀도잉 단계별 값 출력
- [ ] 05_integer_types: 다양한 정수형 출력
- [ ] 06_float_types: 데미지 계산 출력
- [ ] 07_boolean: 논리 연산 결과 출력
- [ ] 08_character: 다양한 문자 출력
- [ ] 09_type_inference: 추론 타입 출력
- [ ] 10_type_conversion: 변환 결과 출력
- [ ] 11_tuples: 튜플 해체 출력
- [ ] 12_arrays: 배열 요소 출력
- [ ] 13_character_creation: 캐릭터 생성 화면 출력

---

## Summary

이 계획에 따라 예제 프로젝트와 13개의 예제 파일을 구현하면, 학습자는 다음을 할 수 있습니다:

1. 각 개념을 별도의 예제로 실행하며 학습
2. docs/ch02.md의 코드 예시와 실제 실행 가능한 코드 비교
3. `cargo run --bin <name>`으로 간단하게 개별 예제 실행
4. Chapter 2의 모든 핵심 개념을 게임 컨텍스트로 체험

모든 예제는 Dungeon Crawler 게임 컨텍스트를 유지하며, 학습자가 실제 게임 개발로 자연스럽게 이어질 수 있도록 설계되었습니다.
