# Examples

이 디렉토리는 각 챕터의 예제 코드를 포함합니다. 각 챕터의 개념을 독립적으로 실행하고 실험할 수 있습니다.

## 디렉토리 구조

```
examples/
├── README.md           # 이 파일
├── ch01/               # 제1주: Rust 시작하기
├── ch02/               # 제2주: 변수와 기본 타입
├── ch03/               # 제3주: 연산자와 표현식 (예정)
└── ...
```

## 챕터별 예제 실행 방법

### 1. 챕터 디렉토리로 이동

```bash
cd examples/ch02
```

### 2. 전체 예제 목록 보기

```bash
cargo run
```

### 제1주: Rust 시작하기

```bash
cd examples/ch01

# 전체 예제 목록
cargo run

# 개별 예제 실행
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

### 제2주: 변수와 기본 타입

```bash
cd examples/ch02

# 개별 예제 실행

```bash
# 예제 번호로 실행
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
```

## 새로운 챕터 예제 추가 방법

### 1. 새로운 챕터 디렉토리 생성

```bash
mkdir -p examples/chXX/src
```

### 2. Cargo.toml 작성

```toml
[package]
name = "chXX-examples"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]

# 개별 예제를 binary로 추가
[[bin]]
name = "01_concept_name"
path = "src/01_concept_name.rs"

[[bin]]
name = "02_another_concept"
path = "src/02_another_concept.rs"
```

### 3. 메인 파일 작성 (src/main.rs)

```rust
//! Chapter X: Chapter Title - Examples
//!
//! This module contains all examples from Chapter X.

fn main() {
    println!("====================================");
    println!("Chapter X Examples");
    println!("Chapter Title");
    println!("====================================");
    println!();
    println!("Run individual examples with:");
    println!("  cargo run --bin 01_concept_name");
    println!("  cargo run --bin 02_another_concept");
}
```

### 4. 예제 파일 작성 (src/01_concept_name.rs)

```rust
//! Example 1: Concept Name
//!
//! Demonstrates:
//! - Key concept 1
//! - Key concept 2

fn main() {
    // Example code here
}
```

### 5. 빌드 및 실행 확인

```bash
# 빌드
cargo build

# 개별 예제 실행
cargo run --bin 01_concept_name

# 포맷팅
cargo fmt

# 린트 검사
cargo clippy -- -D warnings
```

## 예제 작성 가이드라인

### 파일 명명 규칙

- 메인 파일: `main.rs`
- 예제 파일: `NN_description.rs`
  - `NN`: 두 자리 숫자 (01, 02, 03, ...)
  - `description`: 간단한 설명 (snake_case)

### 코드 스타일

1. **모듈 문서 주석**: 각 파일 상단에 `//!`로 설명 추가
2. **함수/타입 문서 주석**: `///`로 public 항목 설명
3. **인라인 주석**: 복잡한 로직에 `//`로 설명
4. **게임 컨텍스트**: 모든 예제는 Dungeon Crawler 게임과 연결

### 예제 구조

```rust
//! Example NN: Concept Name
//!
//! Demonstrates:
//! - Concept 1
//! - Concept 2
//!
//! 간단한 설명 (可选)

fn main() {
    // 1. 기본 예시
    println!("=== 기본 예시 ===");

    // 2. 응용 예시
    println!();
    println!("=== 응용 예시 ===");

    // 3. 실전 예시
    println!();
    println!("=== 실전 예시 ===");
}
```

## 현재 챕터 상태

| 챕터 | 주제 | 상태 |
|------|------|------|
| Ch01 | Rust 시작하기 | ✅ 완료 |
| Ch02 | 변수와 기본 타입 | ✅ 완료 |
| Ch03 | 연산자와 표현식 | 예정 |
| Ch04 | 제어 흐름 | 예정 |
| Ch05 | 함수 | 예정 |

## 유용한 명령어

```bash
# 모든 예제 빌드
cargo build

# 모든 예제 실행 (스크립트 필요)
for bin in 01 02 03; do
    cargo run --bin ${bin}_example_name
done

# 코드 포맷팅
cargo fmt

# 린트 검사
cargo clippy -- -D warnings

# 릴리즈 빌드
cargo build --release

# clean
cargo clean
```

## 문서 참조

- [메인 문서](../../docs/)
- [Chapter 2](../../docs/ch02.md) - 변수와 기본 타입
- [연구 노트](../../context/) - 각 챕터의 기술적 연구
