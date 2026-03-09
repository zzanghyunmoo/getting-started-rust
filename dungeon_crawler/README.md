# 던전 크롤러 (Dungeon Crawler)

> **Rust 프로그래밍 기초 수강생 프로젝트**
>
> 한 학기 동안 Rust를 배우며 만드는 텍스트 RPG 게임

![Rust](https://img.shields.io/badge/Rust-2024.0-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Progress](https://img.shields.io/badge/progress-in%20development-yellow.svg)

---

## 🎮 게임 소개

던전 크롤러는 텍스트 기반의 RPG 게임입니다. 플레이어는 용사를 육성하며 던전을 탐험하고 몬스터와 전투하여 경험치를 쌓습니다.

### 주요 기능 (구현 계획)

- 🏰 **캐릭터 시스템**: 이름, 직업(전사/마법사/도적) 선택, 스탯 커스터마이징
- ⚔️ **전투 시스템**: 턴 기반 전투, 크리티컬 히트, 스킬 사용
- 🎒 **인벤토리**: 아이템 획득, 장착, 소모품 사용
- 🏪 **상점**: 장비와 소모품 구매
- 🏰 **던전 탐험**: 랜덤 몬스터 조우, 보물 상자 발견
- 📈 **성장 시스템**: 레벨업, 스탯 상승, 새로운 스킬 획득
- 💾 **저장/로드**: 게임 진행 저장

---

## 🚀 빠른 시작

### 전제 조건

- Rust 1.90.0 이상
- Cargo (Rust 빌드 도구)

### 설치 및 실행

```bash
# 프로젝트 디렉토리로 이동
cd dungeon_crawler

# 실행
cargo run --release
```

### 개발 모드 실행

```bash
cargo run
```

---

## 📖 현재 구현 상태

### ✅ Week 1-2: 기본 화면 및 캐릭터 생성

```
====================================
         DUNGEON CRAWLER
====================================

Developed by: 짱현무 (202412345)
             컴퓨터 공학과

Version: 1.0.0-alpha

Press Enter to start...
```

#### 구현된 기능

- 🎨 **게임 제목 화면**: ANSI 색상 코드를 활용한 시각적 화면
- 👤 **캐릭터 생성 시스템**:
  - 직업별 스탯 상수 정의
  - 플레이어 이름, 직업 설정
  - 직업별 능력치 매칭 (전사/마법사/도적)
  - 캐릭터 정보 출력

#### 직업별 스탯

| 직업 | HP | MP | 공격력 | 방어력 | 특징 |
|------|-----|-----|--------|--------|------|
| 전사 | 100 | 50 | 15 | 5 | 높은 체력과 방어력 |
| 마법사 | 60 | 100 | 10 | 3 | 높은 마나, 낮은 체력 |
| 도적 | 80 | 60 | 12 | 8 | 균형 잡힌 스탯 |

---

## 🏗️ 프로젝트 구조

```
dungeon_crawler/
├── Cargo.toml           # 프로젝트 설정
├── src/
│   └── main.rs          # 메인 게임 코드
├── README.md            # 이 파일
└── .gitignore
```

---

## 📚 학습 로드맵

이 프로젝트는 프로그래밍 기초 강의의 실습 과제로 설계되었습니다. 각 주차별로 새로운 기능이 추가됩니다:

| 주차 | 추가 기능 | 학습 개념 | 상태 |
|------|-----------|----------|------|
| 1 | 게임 제목 화면 | Hello World, `println!` | ✅ 완료 |
| 2 | 캐릭터 생성 | 변수, 타입, `const`, `match` | ✅ 완료 |
| 3 | 전투 데미지 계산 | 연산자, 표현식 | 🔜 예정 |
| 4 | 승리/패배 조건 | `if`, `match` | 🔜 예정 |
| 5 | 게임 루프 | `loop`, `while`, `for` | 🔜 예정 |
| 7 | 전투 함수화 | 함수, 매개변수, 반환값 | 🔜 예정 |
| 8 | 보물 상자 | 재귀 함수 | 🔜 예정 |
| 9 | 구조체 도입 | `struct`, `impl`, 메서드 | 🔜 예정 |
| 10 | 열거형과 Option | `enum`, `Option<T>`, 패턴 매칭 | 🔜 예정 |
| 11 | 인벤토리 | `Vec<T>`, `String` | 🔜 예정 |
| 12 | 도감 & 상점 | `HashMap<K, V>`, `Result<T, E>` | 🔜 예정 |
| 13 | 정렬 시스템 | 버블 정렬 등 | 🔜 예정 |
| 14 | 검색 기능 | 선형 탐색, 이진 탐색 | 🔜 예정 |
| 15 | 최종 완성 | 전체 통합 | 🔜 예정 |

---

## 🎯 코드 스타일

이 프로젝트는 Rust 공식 스타일 가이드를 따릅니다:

```rust
// 구조체는 UpperCamelCase
struct Player { ... }

// 함수는 snake_case
fn calculate_damage() -> i32 { ... }

// 상수는 UPPER_SNAKE_CASE
const MAX_HP: u32 = 100;
```

### 개발 명령어

```bash
# 빌드
cargo build

# 실행
cargo run

# 테스트
cargo test

# 린트 확인
cargo clippy

# 포맷팅
cargo fmt

# 릴리즈 빌드
cargo build --release
```

---

## 🎨 커스터마이징

게임 설정을 수정하려면 `src/main.rs`에 있는 상수를 변경하세요:

```rust
// 게임 밸런스
const MAX_HP: u32 = 100;
const MAX_MP: u32 = 50;
const STARTING_GOLD: u32 = 100;

// 직업별 스탯
const WARRIOR_HP: u32 = 100;
const WARRIOR_MP: u32 = 50;
// ...
```

---

## 📝 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자유롭게 수정하고 배포하실 수 있습니다.

---

## 📚 참고 자료

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

---

## 🔄 버전 관리

- **v0.1.0** (Week 1-2): 기본 화면 출력 및 캐릭터 생성 시스템

---

*此项目 © 2024. All rights reserved.*
