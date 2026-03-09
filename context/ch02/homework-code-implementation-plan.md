# 제2장 과제 코드 구현 계획

## Context

본 문서는 `docs/ch02.md`의 **8. 실습 - 캐릭터 생성 화면 구현** 섹션에 따라 Dungeon Crawler 게임의 캐릭터 생성 시스템을 구현하기 위한 계획입니다.

**작성 대상:** `getting-started-rust/dungeon_crawler/src/main.rs`

---

## Overview

### 목표

Chapter 2의 핵심 개념을 종합 적용하여 캐릭터 생성 시스템을 구현합니다:
- 상수 (Constants) 정의
- 변수 (Variables) 선언과 불변성
- 타입 어노테이션 (Type Annotations)
- 튜플과 match 표현식을 활용한 패턴 매칭

### 학습 개념 정리

| 개념 | 설명 | 과제 활용 |
|------|------|-----------|
| **상수** | `const` 키워드, 컴파일 타임 상수 | MAX_HP, MAX_MP, STARTING_GOLD |
| **불변 변수** | `let` 키워드, 값 변경 불가 | player_name, player_class |
| **튜플** | 여러 타입의 값을 묶음 | (hp, mp, attack, defense) |
| **match 표현식** | 패턴 매칭으로 값 분기 | 직업별 스탯 선택 |
| **타입 어노테이션** | 명시적 타입 지정 | `: u32`, `: i32` |

---

## 구현 요구사항

### 1. 과제 요구사항 (docs/ch02.md Section 8.1)

다음 내용을 출력하는 프로그램을 작성하세요:

```
====================================
캐릭터가 생성되었습니다!
====================================
이름: 철수
직업: 전사
HP: 100 / MP: 50
공격력: 15 / 방어력: 5
골드: 100G
====================================
```

### 2. 직업별 능력치

| 직업 | HP | MP | 공격력 | 방어력 |
|------|----|----|--------|--------|
| 전사 | 100 | 50 | 15 | 5 |
| 마법사 | 60 | 100 | 10 | 3 |
| 도적 | 80 | 60 | 12 | 8 |

---

## Phase 1: 상수 정의

### 1.1 게임 밸런스 상수

**위치:** `main.rs` 파일 상단 (함수 외부)

```rust
/// 게임 밸런스 상수
const MAX_HP: u32 = 100;
const MAX_MP: u32 = 50;
const STARTING_GOLD: u32 = 100;
```

**설명:**
- `const` 키워드로 상수 선언
- **반드시 타입 어노테이션** 필수 (`: u32`)
- 상수 이름은 **UPPER_SNAKE_CASE** 규칙
- 컴파일 타임에 값 결정됨

### 1.2 직업별 스탯 상수 (선택사항 - 도전 과제용)

```rust
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
```

---

## Phase 2: main() 함수 수정

### 2.1 기존 코드 보존

Chapter 1에서 작성한 타이틀 화면 코드는 그대로 유지합니다:

```rust
fn main() {
    // ANSI 색상 코드
    let yellow = "\x1b[33m";
    let cyan = "\x1b[36m";
    let reset = "\x1b[0m";

    // ... (기존 타이틀 화면 코드 유지)

    println!("Press Enter to start...");
}
```

### 2.2 캐릭터 생성 코드 추가

타이틀 화면 **뒤에** 다음 코드를 추가합니다:

```rust
fn main() {
    // ... (기존 타이틀 화면 코드)

    println!("Press Enter to start...");
    println!();  // 빈 줄 추가

    // ========================================
    // Chapter 2: 캐릭터 생성 시스템
    // ========================================

    // 1. 상수 출력
    println!("=== 게임 밸런스 ===");
    println!("최대 HP: {}", MAX_HP);
    println!("최대 MP: {}", MAX_MP);
    println!("초기 골드: {}G", STARTING_GOLD);
    println!();

    // 2. 플레이어 이름 (불변 변수)
    let player_name = "철수";

    // 3. 직업 (불변 변수)
    let player_class = "전사";

    // 4. 직업별 초기 능력치 (match와 튜플)
    let (hp, mp, attack, defense) = match player_class {
        "전사" => (100, 50, 15, 5),
        "마법사" => (60, 100, 10, 3),
        "도적" => (80, 60, 12, 8),
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
}
```

---

## Phase 3: 핵심 개념 설명

### 3.1 상수 (Constants)

```rust
const MAX_HP: u32 = 100;
```

**특징:**
- `const` 키워드 사용
- **타입 어노테이션 필수** (`: u32`)
- 컴파일 타임에 값 결정
- 실행 중 변경 불가능

### 3.2 불변 변수 (Immutable Variables)

```rust
let player_name = "철수";
let player_class = "전사";
```

**특징:**
- `let` 키워드 사용
- 기본적으로 **불변** (immutable)
- 타입 추론 가능 (`&str`로 자동 추론)

### 3.3 match와 튜플

```rust
let (hp, mp, attack, defense) = match player_class {
    "전사" => (100, 50, 15, 5),
    "마법사" => (60, 100, 10, 3),
    "도적" => (80, 60, 12, 8),
    _ => (80, 50, 10, 5),  // 기본값
};
```

**특징:**
- `match`는 표현식으로 값을 반환
- 튜플 `(100, 50, 15, 5)`을 반환
- **튜플 해체** (tuple destructuring)로 변수에 할당
- `_`는 기본값 (wildcard pattern)

---

## Phase 4: 도전 과제 (선택사항)

### 4.1 직업별 스탯 비교 표시

```rust
// 도전 과제: 직업별 스탯 비교
println!();
println!("====================================");
println!("직업별 스탯 비교");
println!("====================================");
println!("[전사]   HP:{} MP:{} 공격:{} 방어:{}",
    WARRIOR_HP, WARRIOR_MP, WARRIOR_ATTACK, WARRIOR_DEFENSE);
println!("[마법사] HP:{} MP:{} 공격:{} 방어:{}",
    MAGE_HP, MAGE_MP, MAGE_ATTACK, MAGE_DEFENSE);
println!("[도적]   HP:{} MP:{} 공격:{} 방어:{}",
    ROGUE_HP, ROGUE_MP, ROGUE_ATTACK, ROGUE_DEFENSE);
println!("====================================");
```

---

## Verification

### 1. 빌드 및 실행 확인

```bash
cd dungeon_crawler
cargo run
```

**기대 출력:**
```
====================================
         DUNGEON CRAWLER
====================================

Developed by: 짱현무 (202412345)
             컴퓨터 공학과

Version: 1.0.0-alpha

Press Enter to start...

=== 게임 밸런스 ===
최대 HP: 100
최대 MP: 50
초기 골드: 100G

====================================
캐릭터가 생성되었습니다!
====================================
이름: 철수
직업: 전사
HP: 100 / MP: 50
공격력: 15 / 방어력: 5
골드: 100G
====================================
```

### 2. 코드 포맷팅

```bash
cargo fmt
```

### 3. 린트 검사

```bash
cargo clippy
```

### 4. Git 커밋

```bash
git add .
git commit -m "feat: Week 2 - 캐릭터 생성 시스템 구현

- 게임 밸런스 상수 추가 (MAX_HP, MAX_MP, STARTING_GOLD)
- 플레이어 이름과 직업 변수 선언
- match 표현식으로 직업별 스탯 선택
- 캐릭터 생성 화면 출력
- 도전 과제: 직업별 스탯 비교 표
"
```

---

## Summary

이 과제를 통해 학습하는 내용:

1. **상수 정의**: 게임 밸런스 설정을 상수로 관리
2. **변수 선언**: 불변 변수로 변경되지 않는 데이터 저장
3. **타입 시스템**: 정수형(`u32`, `i32`)과 문자열(`&str`) 타입 이해
4. **패턴 매칭**: `match` 표현식으로 조건부 처리
5. **튜플 활용**: 여러 값을 묶어서 반환하고 해체

---

## 주의사항

1. **Chapter 1 코드 보존**: 기존 타이틀 화면은 그대로 유지
2. **상수 이름 규칙**: 반드시 `UPPER_SNAKE_CASE` 사용
3. **타입 어노테이션**: 상수는 반드시 타입 명시 필요
4. **match의 기본값**: `_` 패턴으로 처리되지 않은 경우 대비
5. **코드 포맷팅**: `cargo fmt`로 일관된 스타일 유지

---

## 참고 자료

- [docs/ch02.md](../../docs/ch02.md) - Chapter 2 강의 문서
- [context/ch02/research.md](research.md) - 기술 연구 문서
- [Rust Book - Chapter 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) - 공식 문서
