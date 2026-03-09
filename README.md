# Rust 프로그래밍 기초 실습 프로젝트

> **Rust 언어의 기초부터 실전 프로젝트까지**
>
> 이 저장소는 Rust 프로그래밍 기초 강의를 위한 예제 코드와 실습 프로젝트를 포함합니다.

![Rust](https://img.shields.io/badge/Rust-2024.0-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Status](https://img.shields.io/badge/status-active-success.svg)

---

## 📚 저장소 개요

이 저장소는 Rust 프로그래밍을 처음 시작하는 학습자를 위해 구성되었습니다. 챕터별 예제 코드와 실전 프로젝트를 통해 Rust의 기초 개념부터 실전 응용까지 단계적으로 학습할 수 있습니다.

### 📁 구조

```
getting-started-rust/
├── examples/            # 챕터별 예제 코드
│   ├── ch01/           # 제1주: Rust 시작하기
│   ├── ch02/           # 제2주: 변수와 기본 타입
│   └── ...
├── dungeon_crawler/    # 실전 프로젝트: 텍스트 RPG 게임
├── docs/               # 강의 노트
├── context/            # 기술적 연구 노트
└── README.md           # 이 파일
```

---

## 🚀 빠른 시작

### 전제 조건

- **Rust 1.90.0 이상**
- **Cargo** (Rust 빌드 도구)

### Rust 설치

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 설치 확인

```bash
rustc --version
cargo --version
```

---

## 📖 사용법

### 1. 챕터별 예제 실행

```bash
# 예제 디렉토리로 이동
cd examples/ch01

# 전체 예제 목록 보기
cargo run

# 개별 예제 실행
cargo run --bin 01_hello_world
cargo run --bin 09_game_title_screen
```

### 2. 던전 크롤러 프로젝트

```bash
# 프로젝트 디렉토리로 이동
cd dungeon_crawler

# 게임 실행
cargo run --release
```

---

## 📚 커리큘럼

### 챕터별 학습 로드맵

| 주차 | 주제 | 학습 개념 | 상태 |
|------|------|----------|------|
| 1 | Rust 시작하기 | Hello World, `println!`, 함수 구조 | ✅ 완료 |
| 2 | 변수와 기본 타입 | `let`, `const`, 타입 추론, 튜플, 배열 | ✅ 완료 |
| 3 | 연산자와 표현식 | 산술, 비교, 논리 연산자 | 🔜 예정 |
| 4 | 제어 흐름 | `if`, `match`, `loop`, `while`, `for` | 🔜 예정 |
| 5 | 함수 | 매개변수, 반환값, 문 vs 표현식 | 🔜 예정 |
| 6 | 소유권 | 스택 vs 힙, 빌림, 참조자 | 🔜 예정 |
| 7 | 구조체 | `struct`, `impl`, 메서드 | 🔜 예정 |
| 8 | 열거형과 패턴 매칭 | `enum`, `Option<T>`, `Result<T,E>` | 🔜 예정 |
| 9 | 컬렉션 | `Vec<T>`, `HashMap<K,V>`, `String` | 🔜 예정 |
| 10 | 에러 처리 | `panic!`, `Result`, `unwrap`, `?` | 🔜 예정 |
| 11 | 제네릭과 트레이트 | 타입 매개변수, 트레이트 바운드 | 🔜 예정 |
| 12 | 알고리즘 | 정렬, 검색, 재귀 | 🔜 예정 |
| 13-15 | 프로젝트 완성 | 통합, 테스트, 최적화 | 🔜 예정 |

---

## 🎮 던전 크롤러 프로젝트

[projects/dungeon_crawler/](projects/dungeon_crawler/) - 텍스트 기반 RPG 게임

이 프로젝트는 학습한 개념들을 실전에 적용해볼 수 있는 통합 프로젝트입니다. 한 학기 동안 점진적으로 기능을 추가하며 완성해 나갑니다.

### 주요 기능

- 🏰 캐릭터 생성 및 육성
- ⚔️ 턴 기반 전투 시스템
- 🎒 인벤토리 관리
- 🏪 상점 시스템
- 🏰 던전 탐험
- 💾 세이브/로드

**[자세히 보기 →](dungeon_crawler/README.md)**

---

## 📁 디렉토리 안내

### `examples/`

각 챕터의 핵심 개념을 독립적으로 실행하고 실험할 수 있는 예제 코드들입니다.

```bash
# 예제 목록 구조
examples/
├── ch01/              # Rust 시작하기
│   └── src/
│       ├── 01_hello_world.rs
│       ├── 02_function_analysis.rs
│       └── ...
├── ch02/              # 변수와 기본 타입
│   └── src/
│       ├── 01_variables.rs
│       ├── 02_mutability.rs
│       └── ...
└── README.md
```

### `dungeon_crawler/`

실전 통합 프로젝트입니다. 학습한 개념들을 게임 개발에 적용해볼 수 있습니다.

### `docs/`

강의 노트와 이론적 배경이 정리된 문서들입니다.

### `context/`

각 챕터의 기술적 연구 내용과 심화 학습 자료가 포함되어 있습니다.

---

## 🛠️ 개발 가이드

### 코드 스타일

이 프로젝트는 Rust 공식 스타일 가이드를 따릅니다:

```rust
// 구조체는 UpperCamelCase
struct Player { ... }

// 함수는 snake_case
fn calculate_damage() -> i32 { ... }

// 상수는 UPPER_SNAKE_CASE
const MAX_HP: u32 = 100;
```

### 유용한 명령어

```bash
# 빌드
cargo build

# 실행
cargo run

# 테스트
cargo test

# 린트 검사
cargo clippy

# 포맷팅
cargo fmt

# 문서 생성
cargo doc --open

# 릴리즈 빌드
cargo build --release

# clean
cargo clean
```

---

## 📝 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자유롭게 수정하고 배포하실 수 있습니다.

---

## 🙋 기여 가이드

기여를 환영합니다! 다음 단계를 따라주세요:

1. 이 저장소를 포크합니다
2. 기능 브랜치를 생성합니다 (`git checkout -b feature/amazing-feature`)
3. 변경사항을 커밋합니다 (`git commit -m 'Add amazing feature'`)
4. 브랜치에 푸시합니다 (`git push origin feature/amazing-feature`)
5. Pull Request를 생성합니다

---

## 📚 참고 자료

### 공식 문서

- [The Rust Programming Language](https://doc.rust-lang.org/book/) - "The Book"
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 예제 중심 학습
- [Rust Standard Library](https://doc.rust-lang.org/std/) - 표준 라이브러리 문서
- [The Rust Reference](https://doc.rust-lang.org/reference/) - 언어 레퍼런스

### 커뮤니티

- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust subreddit](https://reddit.com/r/rust)

### 도구

- [Rust Playground](https://play.rust-lang.org/) - 온라인 Rust 실행 환경
- [Cargo Docs](https://doc.rust-lang.org/cargo/) - 패키지 매니저 문서

---

## 📮 연락처

이 프로젝트에 관한 문의나 버그 리포트는 GitHub Issues를 통해 제출해주세요.

---

*Happy Rustacean! 🦀*

**© 2024. All rights reserved.**
