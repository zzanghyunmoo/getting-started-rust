# 제2장 강의 문서 작성 계획

## Context

본 문서는 "프로그래밍 기초 (Rust)" 강의의 제2주 차 강의 내용을 담은 `docs/ch02.md` 작성을 위한 구현 계획입니다. 제2주의 주제는 **"변수와 기본 타입"**이며, 학생들이 Dungeon Crawler 게임의 캐릭터 생성 시스템을 구현하며 Rust의 변수 시스템과 타입 시스템을 자연스럽게 습득할 수 있도록 설계되었습니다.

**작성 대상 파일:** `/home/gurumee/Workspaces/projects/getting-started-rust/docs/ch02.md`

---

## Chapter Structure

```markdown
# 제2주: 변수와 기본 타입

> **캐릭터 정보 저장하기**
>
> 본 장에서는 변수와 상수의 개념을 이해하고, Rust의 기본 타입 시스템을 학습하여 캐릭터 생성 화면을 구현합니다.

## 학습 목표

| 유형 | 목표 |
|------|------|
| **지식** | 변수와 불변성, 상수, 섀도잉, 기본 데이터 타입 이해 |
| **기술** | let 키워드, mut 키워드, 타입 어노테이션, 타입 추론 활용 |
| **실습** | 캐릭터 이름, 직업, 능력치 변수로 저장하고 출력 |
```

---

## Content Sections

### 1. 변수와 불변성 (Variables and Immutability)
- **1.1 변수 선언 기본**: `let` 키워드, 변수 명명 규칙
- **1.2 불변성의 기본**: 기본적으로 불변인 이유, 안전성 보장
- **1.3 가변 변수**: `mut` 키워드 사용법
- **1.4 메모리 표현**: 스택 메모리 다이어그램 (ASCII art)

### 2. 상수 (Constants)
- **2.1 상수 선언 문법**: `const` 키워드, 타입 어노테이션 필수
- **2.2 상수 vs 불변 변수**: 비교 표
- **2.3 게임 밸런스 상수 예시**: MAX_HP, STARTING_GOLD 등

### 3. 섀도잉 (Shadowing)
- **3.1 섀도잉의 정의와 문법**
- **3.2 섀도잉 vs mut**: 비교 표
- **3.3 실전 사용 사례**: 데이터 변환 파이프라인 (trim → parse → validate)
- **3.4 스코프 시각화**: ASCII art 다이어그램

### 4. 기본 데이터 타입 (Basic Data Types)
- **4.1 정수형**: i8~i128, u8~u128, isize/usize (범위 표)
- **4.2 부동소수점형**: f32, f64
- **4.3 불리언형**: bool
- **4.4 문자형**: char (Unicode Scalar Value)
- **4.5 메모리 레이아웃**: 각 타입의 메모리 표현 다이어그램

### 5. 타입 추론과 어노테이션 (Type Inference & Annotation)
- **5.1 타입 추론**: Hindley-Milner 스타일 추론 설명
- **5.2 타입 어노테이션**: 언제 필요한가?
- **5.3 추론의 한계**: 제네릭 메서드, 빈 컬렉션

### 6. 타입 변환 (Type Conversion)
- **6.1 명시적 변환**: `as` 키워드
- **6.2 안전한 vs 위험한 변환**: 변환 호환성 표

### 7. 복합 타입 기초 (Compound Types)
- **7.1 튜플 (Tuple)**: 선언, 해체, 인덱스 접근
- **7.2 배열 (Array)**: 선언, 접근, 안전성
- **7.3 유닛 타입**: `()`

### 8. 실습 - 캐릭터 생성 화면 구현
- **8.1 과제 요구사항**
- **8.2 단계별 구현 가이드**
- **8.3 실행 결과 확인**
- **8.4 코드 포맷팅 & 린트 검사**
- **8.5 Git 커밋 가이드**
- **8.6 도전 과제 (직업별 스탯 비교)**

---

## Key Game Context Integration

모든 코드 예시는 Dungeon Crawler 게임 컨텍스트와 연결:

| 개념 | 게임 사용 예시 |
|------|----------------|
| 변수 | 플레이어 이름, 직업 저장 |
| 상수 | MAX_HP (100), MAX_MP (50), STARTING_GOLD (100) |
| 섀도잉 | 이름 입력 → trim → 유효성 검사 |
| 정수형 (u32) | HP, MP, 골드 (음수 없음) |
| 정수형 (i32) | 공격력, 방어력 (디버프 가능) |
| 불리언 | is_alive, has_key, is_poisoned |
| 문자 | 첫 글자 추출, 등급 표시 |
| 튜플 | 좌표 (x, y) - 향후 사용 |
| 배열 | 직업별 스탯 [HP, MP, 공격, 방어] |

### 직업별 능력치 (from README.md)

| 직업 | HP | MP | 공격력 | 방어력 |
|------|----|----|--------|--------|
| 전사 | 100 | 50 | 15 | 5 |
| 마법사 | 60 | 100 | 10 | 3 |
| 도적 | 80 | 60 | 12 | 8 |

---

## Code Examples to Include

### Example 1: Basic Variable Declaration
```rust
fn main() {
    let player_name = "철수";
    let player_class = "전사";

    println!("용사의 이름: {}", player_name);
    println!("직업: {}", player_class);
}
```

### Example 2: Immutability Error (Show the Error)
```rust
let player_name = "철수";
// player_name = "민수";  // 컴파일 에러!
```

### Example 3: Mutable Variables
```rust
let mut player_hp = 100;
println!("현재 HP: {}", player_hp);
player_hp = 85;  // 전투에서 피해
println!("전투 후 HP: {}", player_hp);
```

### Example 4: Constants
```rust
const MAX_HP: u32 = 100;
const MAX_MP: u32 = 50;
const STARTING_GOLD: u32 = 100;

fn main() {
    println!("초기 HP: {}", MAX_HP);
    println!("초기 골드: {}", STARTING_GOLD);
}
```

### Example 5: Shadowing
```rust
let name = "  철수  ";
let name = name.trim();  // 섀도잉: 공백 제거
let name = if name.is_empty() {
    "모험가"
} else {
    name
};
println!("용사의 이름: {}", name);
```

### Example 6: Integer Types with Game Stats
```rust
let hp: u32 = 100;          // 양수만
let mp: u32 = 50;
let gold: u32 = 150;
let attack: i32 = 15;       // 음수 가능 (디버프)
let defense: i32 = 5;
let exp: u64 = 1_234_567_890;  // 큰 값

println!("HP: {} | MP: {}", hp, mp);
println!("공격력: {} | 방어력: {}", attack, defense);
```

### Example 7: Boolean Type
```rust
let is_alive: bool = true;
let is_poisoned: bool = false;
let has_key: bool = true;

if is_alive && has_key {
    println!("문을 열 수 있습니다!");
}
```

### Example 8: Type Annotation
```rust
let guess: u32 = "42".parse().expect("숫자가 아닙니다!");
let max_hp: u32 = 100;
```

### Example 9: Tuples
```rust
let position: (i32, i32) = (10, 20);
let (x, y) = position;
println!("위치: ({}, {})", x, y);
```

### Example 10: Arrays (Job Stats)
```rust
let warrior_stats: [i32; 4] = [100, 50, 15, 5];  // HP, MP, 공격, 방어
let mage_stats: [i32; 4] = [60, 100, 10, 3];
let rogue_stats: [i32; 4] = [80, 60, 12, 8];

println!("전사 HP: {}", warrior_stats[0]);
println!("마법사 MP: {}", mage_stats[1]);
```

---

## Visual Elements to Include

### Table 1: Integer Types
| 타입 | 크기 | 범위 | 게임 사용 예시 |
|------|------|------|---------------|
| i32 | 32-bit | ±21억 | 공격력, 방어력 |
| u32 | 32-bit | 0~42억 | HP, MP, 골드 |
| u64 | 64-bit | 0~1.8×10¹⁹ | 누적 경험치 |

### Table 2: const vs let
| 특성 | const | let |
|------|-------|-----|
| 타입 어노테이션 | 필수 | 선택적 |
| 할당 시점 | 컴파일 타임 | 런타임 |
| 값 변경 | 불가능 | mut으로 가능 |

### ASCII Art 1: Memory Layout
```
┌─────────────────────────────────────┐
│         스택 메모리 (Stack)          │
├─────────────────────────────────────┤
│  [player_name] → "철수" (힙 참조)    │
│  [player_hp] → 100                   │
│  [player_level] → 1                  │
└─────────────────────────────────────┘
```

### ASCII Art 2: Shadowing Scope
```
스코프 레벨 0:
    let x = 5;           ━━ x는 5
    {
        let x = x * 2;   ━━ 내부: x는 10
        {
            let x = x + 1; ━━ 더 내부: x는 11
        }
    }
    println!("{}", x);    ━━ 5 출력
```

---

## Exercise Specification

### Main Exercise: Character Creation System

**요구사항:**
```rust
// 구현해야 할 내용:
// 1. 상수 정의: MAX_HP, MAX_MP, STARTING_GOLD
// 2. 플레이어 이름 변수 (불변)
// 3. 직업 변수 (전사/마법사/도적 중 하나)
// 4. 직업별 초기 능력치
//    - 전사: HP 100, MP 50, 공격력 15, 방어력 5
//    - 마법사: HP 60, MP 100, 공격력 10, 방어력 3
//    - 도적: HP 80, MP 60, 공격력 12, 방어력 8
// 5. 캐릭터 정보 출력
```

**실행 예시:**
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

### Challenge Exercise: Multiple Characters
```rust
// 3개의 직업을 각각 변수로 만들고 모두 출력
// 실행 예시:
// ====================================
// 직업별 스탯 비교
// ====================================
// [전사] HP:100 MP:50 공격:15 방어:5
// [마법사] HP:60 MP:100 공격:10 방어:3
// [도적] HP:80 MP:60 공격:12 방어:8
// ====================================
```

---

## Critical Files Reference

### Source Materials
- **Research:** `/home/gurumee/Workspaces/projects/getting-started-rust/context/ch02/research.md` - 모든 기술 내용
- **Template:** `/home/gurumee/Workspaces/projects/getting-started-rust/docs/ch01.md` - 문서 스타일과 구조
- **Week 2 Requirements:** `/home/gurumee/Workspaces/projects/getting-started-rust/context/lecture-plan.md` (L215-L283)
- **Game Context:** `/home/gurumee/Workspaces/projects/getting-started-rust/README.md` - 직업별 스탯 정보

### Output File
- **Target:** `/home/gurumee/Workspaces/projects/getting-started-rust/docs/ch02.md`

---

## Implementation Checklist

### Structure Requirements
- [ ] Header: "제2주: 변수와 기본 타입"
- [ ] Blockquote: "캐릭터 정보 저장하기"
- [ ] 학습 목표 표 (지식, 기술, 실습)
- [ ] 8개의 주요 섹션 (변수~실습)
- [ ] 각 섹션에 게임 컨텍스트 예시 포함

### Style Requirements (matching ch01.md)
- [ ] 한국어 + 영어 기술 용어
- [ ] 표로 비교/분류 (최소 3개)
- [ ] ASCII art 다이어그램 (최소 2개)
- [ ] 코드 예시는 `rust` syntax highlighting
- [ ] 파일 경로 참조: `[main.rs](src/main.rs)` 형식
- [ ] 실습 섹션: 요구사항 → 구현 → 확인 → 포맷팅 → 린트 → 커밋

### Content Requirements
- [ ] research.md의 모든 주제 다룸
- [ ] lecture-plan.md의 Week 2 목표 충족
- [ ] 직업별 스탯 (전사/마법사/도적) 정확히 반영
- [ ] 진행 난이도: 기본 → 복잡 (progressive complexity)
- [ ] "따라치기만 해도 이해되도록" 설명

### Exercise Requirements
- [ ] 명확한 요구사항
- [ ] 단계별 구현 가이드
- [ ] 실행 결과 예시
- [ ] cargo fmt, cargo clippy 안내
- [ ] Git commit 메시지 예시
- [ ] 도전 과제 포함

---

## Verification Plan

작성 완료 후 다음을 확인:
1. `cargo fmt` 실행하여 모든 코드 예시가 포맷팅됨
2. `cargo clippy` 실행하여 경고 없음 확인
3. 모든 링크와 참조가 올바른지 확인
4. ch01.md와 스타일 일치성 확인
5. lecture-plan.md Week 2 요구사항 모두 충족 확인

---

## Expected Output Structure

```
docs/ch02.md
├── Header (제2주: 변수와 기본 타입)
├── 학습 목표 (표)
├── Section 1: 변수와 불변성
│   ├── 1.1 변수 선언 기본
│   ├── 1.2 불변성의 기본
│   ├── 1.3 가변 변수
│   └── 1.4 메모리 표현 (ASCII art)
├── Section 2: 상수
│   ├── 2.1 상수 선언 문법
│   ├── 2.2 상수 vs 불변 변수 (표)
│   └── 2.3 게임 밸런스 상수 예시
├── Section 3: 섀도잉
│   ├── 3.1 섀도잉의 정의와 문법
│   ├── 3.2 섀도잉 vs mut (표)
│   ├── 3.3 실전 사용 사례
│   └── 3.4 스코프 시각화 (ASCII art)
├── Section 4: 기본 데이터 타입
│   ├── 4.1 정수형 (표)
│   ├── 4.2 부동소수점형
│   ├── 4.3 불리언형
│   ├── 4.4 문자형
│   └── 4.5 메모리 레이아웃
├── Section 5: 타입 추론과 어노테이션
├── Section 6: 타입 변환 (표)
├── Section 7: 복합 타입 기초
├── Section 8: 실습 - 캐릭터 생성 화면
│   ├── 8.1 과제 요구사항
│   ├── 8.2 단계별 구현 가이드
│   ├── 8.3 실행 결과 확인
│   ├── 8.4 코드 포맷팅 & 린트 검사
│   ├── 8.5 Git 커밋 가이드
│   └── 8.6 도전 과제
├── 요약 (핵심 개념 표, 학습 체크리스트, 다음 단계)
└── 참고 자료
```

---

## Summary

본 계획은 `context/ch02/research.md`의 기술 내용을 `docs/ch01.md`의 문서 스타일에 맞춰 학생 친화적인 강의 문서로 변환합니다. 모든 코드 예시는 Dungeon Crawler 게임 컨텍스트(캐릭터 생성, 직업별 스탯)와 연결되어 학생들이 자연스럽게 변수와 타입 시스템을 습득할 수 있도록 설계되었습니다.
