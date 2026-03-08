# 제1주: 프로그래밍과 컴퓨팅 개론

> **첫 번째 프로그램 - 게임 제목 화면**
>
> 본 장에서는 프로그래밍과 컴퓨터 과학의 기본 개념을 이해하고, Rust 개발 환경을 설정하며, 첫 번째 프로그램을 작성하여 게임 제목 화면을 출력합니다.

---

## 목차

1. [컴퓨터 과학이란?](#1-컴퓨터-과학이란)
2. [프로그래밍 언어의 역사와 분류](#2-프로그래밍-언어의-역사와-분류)
3. [Rust 언어 소개](#3-rust-언어-소개)
4. [컴파일러와 인터프리터](#4-컴파일러와-인터프리터)
5. [Rust 개발 환경 설정](#5-rust-개발-환경-설정)
6. [Hello World 프로그램 분석](#6-hello-world-프로그램-분석)
7. [println! 매크로 상세 분석](#7-println-매크로-상세-분석)
8. [과제 #1: 게임 시작 화면 구현](#8-과제-1-게임-시작-화면-구현)

---

## 1. 컴퓨터 과학이란?

### 1.1 정의

컴퓨터 과학(Computer Science)은 **컴퓨터와 계산(computation)에 관한 학문**입니다. 단순히 컴퓨터를 사용하는 법을 배우는 것이 아니라, **"어떻게 효율적으로 문제를 해결할 것인가?"**를 연구하는 학문입니다.

### 1.2 핵심 연구 분야

| 분야 | 설명 | 예시 |
|------|------|------|
| **알고리즘** | 문제 해결 절차 | 정렬, 탐색, 최단 경로 |
| **자료구조** | 데이터的组织化 방법 | 배열, 리스트, 트리, 그래프 |
| **프로그래밍 언어** | 인간과 컴퓨터의 소통 도구 | Rust, Python, C++ |
| **컴퓨터 구조** | 하드웨어의 동작 원리 | CPU, 메모리, 저장장치 |
| **운영체제** | 시스템 자원 관리 | Linux, Windows, macOS |
| **네트워크** | 데이터 통신 | 인터넷, TCP/IP |
| **인공지능** | 지능적인 시스템 | 머신러닝, 딥러닝 |
| **보안** | 시스템 보호 | 암호화, 방화벽 |

### 1.3 프로그래밍의 본질

프로그래밍은 **"컴퓨터에게 일을 시키는 방법"**입니다. 더 정확히는:

```
문제 정의 → 해결책 설계 → 코드로 구현 → 테스트 → 유지보수
```

**프로그래밍의 핵심 요소:**
1. **추상화(Abstraction)**: 복잡한 것을 단순하게 만들기
2. **자동화(Automation)**: 반복 작업을 자동으로 처리
3. **알고리즘적 사고**: 단계별 문제 해결 능력

---

## 2. 프로그래밍 언어의 역사와 분류

### 2.1 프로그래밍 언어의 진화

```
기계어 → 어셈블리어 → 고급 언어 → 현대 언어
(0/1)    (니모닉)      (C, Fortran)  (Rust, Python)
```

| 세대 | 언어 | 특징 |
|------|------|------|
| 1세대 | 기계어 | 0과 1로만 구성, 직접 하드웨어 제어 |
| 2세대 | 어셈블리어 | ADD, MOV 같은 니모닉 사용 |
| 3세대 | 고급 언어 | C, Fortran, Java - 사람이 이해하기 쉬움 |
| 4세대 | 선언형 언어 | SQL, Prolog - 무엇을 할지 선언 |
| 5세대 | 현대 언어 | Rust, Python, Go - 고수준 추상화 |

### 2.2 언어 분류

#### 실행 방식에 따른 분류

**컴파일 언어 (Compiled Languages):**
```
소스 코드 → 컴파일러 → 기계어 → 실행
```
- 예시: C, C++, Rust, Go
- 장점: 실행 속도가 빠름
- 단점: 컴파일 시간이 걸림

**인터프리터 언어 (Interpreted Languages):**
```
소스 코드 → 인터프리터 → 한 줄씩 실행
```
- 예시: Python, JavaScript, Ruby
- 장점: 즉시 실행 가능, 개발이 빠름
- 단점: 실행 속도가 상대적으로 느림

#### 타입 시스템에 따른 분류

| 특성 | 정적 타이핑 | 동적 타이핑 |
|------|------------|-------------|
| 타입 검사 시점 | 컴파일 타임 | 런타임 |
| 예시 | Rust, C++, Java | Python, JavaScript |
| 장점 | early error detection | 유연함, 빠른 개발 |
| 단점 | 타입 명시가 번거로울 수 있음 | 런타임 에러 가능성 |

#### 메모리 관리에 따른 분류

**수동 메모리 관리:**
- 예시: C, C++
- `malloc()`, `free()` 직접 호출
- 메모리 누수, 이중 해제 등의 버그 가능

**가비지 컬렉션 (Garbage Collection):**
- 예시: Java, Python, Go
- 자동으로 사용하지 않는 메모리 해제
- 일시적인 성능 저하(Pause time) 발생

**소유권 기반 (Ownership):**
- 예시: Rust
- 컴파일 타임에 메모리 안전성 보장
- 가비지 컬렉션 오버헤드 없음

### 2.3 주요 프로그래밍 언어 비교

| 언어 | 출시연 | 주요 용도 | 패러다임 |
|------|--------|-----------|----------|
| C | 1972 | 시스템, 임베디드 | 절차적 |
| C++ | 1985 | 시스템, 게임 | 다중 패러다임 |
| Java | 1995 | 엔터프라이즈, 안드로이드 | 객체지향 |
| Python | 1991 | 데이터 과학, 웹 | 다중 패러다임 |
| JavaScript | 1995 | 웹 프론트엔드 | 멀티패러다임 |
| Rust | 2015 | 시스템, 웹어셈블리 | 다중 패러다임 |
| Go | 2009 | 클라우드, 서버 | 절차적, 동시성 |

---

## 3. Rust 언어 소개

### 3.1 Rust의 역사

```
2006: Graydon Hoare가 개발 시작 (Mozilla 연구소)
2010: 오픈 소스로 공개
2015: Rust 1.0 정식 출시
2024: Rust 2024 Edition 출시
```

### 3.2 Rust의 설계 철학

**Rust가 해결하고자 하는 문제:**

```c
// C++의 메모리 안전성 문제
void use_after_free() {
    int* p = new int(42);
    delete p;
    std::cout << *p;  // 미정의 동작! 댕글링 포인터
}

void data_race() {
    int counter = 0;
    // 두 스레드가 동시에 counter에 접근
    // 데이터 레이스 발생!
}
```

**Rust의 해결:**

```rust
// Rust: 컴파일 타임에 댕글링 참조 방지
fn use_after_free() {
    let p = Box::new(42);
    drop(p);
    // println!("{}", *p);  // 컴파일 에러!
}

// Rust: 데이터 레이스 컴파일 타임 방지
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
// Arc<Mutex<_>>는 스레드 안전함
```

### 3.3 Rust의 핵심 특징

| 특징 | 설명 | 이점 |
|------|------|------|
| **메모리 안전성** | 컴파일 타임에 보장 | 댕글링 포인터, 이중 해제 방지 |
| **제로 비용 추상화** | 고수준 기능도 저수준만큼 빠름 | 추상화로 인한 성능 저하 없음 |
| **무공포 동시성** | 데이터 레이스를 컴파일 타임에 감지 | 안전한 멀티스레딩 |
| **현대적 문법** | 패턴 매칭, 타입 추론 등 | 개발 생산성 향상 |
| **훌륭한 도구** | Cargo, rustfmt, clippy | 개발 경험 개선 |

### 3.4 Rust가 적합한 분야

```
┌─────────────────────────────────────────────┐
│         Rust의 주요 사용 분야               │
├─────────────────────────────────────────────┤
│ • 시스템 프로그래밍 (OS, 커널)              │
│ • 임베디드 시스템                           │
│ • 웹 서버 및 백엔드                         │
│ • WebAssembly (웹 프론트엔드)               │
│ • 블록체인 및 크립토                        │
│ • 게임 엔진                                 │
│ • CLI 도구                                  │
│ • DevOps/인프라 (Docker, Kubernetes 구성요소)│
└─────────────────────────────────────────────┘
```

### 3.5 Rust 생태계

**패키지 매니저 및 빌드 도구:**
- **Cargo**: Rust의 표준 패키지 매니저이자 빌드 시스템

**주요 라이브러리:**
- `serde`: 직렬화/역직렬화
- `tokio`: 비동기 런타임
- `clap`: CLI 인자 파싱
- `axum`/`actix-web`: 웹 프레임워크

**커뮤니티:**
- crates.io: 패키지 저장소 (200,000+ 패키지)
- Rust Discord/Reddit
- annual RustConf

---

## 4. 컴파일러와 인터프리터

### 4.1 컴파일러(Compiler)의 작동 원리

```
┌─────────────────────────────────────────────────────────┐
│                   컴파일 과정                           │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  소스 코드        전처리기      어휘/구문 분석          │
│  (main.rs)   →  ────────→   ────────────────          │
│                                                         │
│     ↓                                                      │
│  ┌─────────────┐                                         │
│  │   AST       │        추상 구문 트리                   │
│  └─────────────┘                                         │
│     ↓                                                      │
│  ┌─────────────┐                                         │
│  │   IR        │        중간 코드 (LLVM IR)             │
│  └─────────────┘                                         │
│     ↓                                                      │
│  ┌─────────────┐                                         │
│  │   최적화    │        인라인화, 불필요한 코드 제거     │
│  └─────────────┘                                         │
│     ↓                                                      │
│  ┌─────────────┐                                         │
│  │   기계어    │        실행 파일                        │
│  └─────────────┘                                         │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Rust 컴파일 파이프라인

```
Source Code (.rs)
      ↓
   Tokenizer (렉서)
      ↓
    AST (추상 구문 트리)
      ↓
    HIR (High-level IR)
      ↓
    MIR (Mid-level IR)
      ↓
LLVM IR
      ↓
   Machine Code
```

### 4.3 컴파일러 vs 인터프리터 비교

| 특성 | 컴파일러 | 인터프리터 |
|------|----------|-----------|
| 번역 방식 | 전체를 한 번에 번역 | 한 줄씩 번역 후 실행 |
| 실행 속도 | 빠름 | 상대적으로 느림 |
| 오류 발견 | 컴파일 타임 | 런타임 |
| 배포 | 실행 파일만 배포 | 소스 코드 배포 |
| 예시 | Rust, C, C++ | Python, Ruby, JavaScript |

### 4.4 하이브리드 방식 (JIT)

**JIT(Just-In-Time) 컴파일러:**
```
소스 코드 → 바이트코드 → JIT 컴파일 → 기계어 → 실행
```

- 예시: Java (JVM), JavaScript (V8)
- 인터프리터의 편리함 + 컴파일러의 성능
- Warm-up 시간 필요

### 4.5 LLVM과 Rust

**LLVM(Low Level Virtual Machine):**
- 모던 컴파일러 인프라
- C++, Rust, Swift 등이 사용

```
Rust 소스
    ↓
rustc (Rust 컴파일러)
    ↓
LLVM IR (중간 표현)
    ↓
LLVM 최적화
    ↓
기계어 (x86, ARM, etc.)
```

Rust는 LLVM을 백엔드로 사용하여 다양한 아키텍처를 지원합니다.

---

## 5. Rust 개발 환경 설정

### 5.1 필수 도구 설치

#### 5.1.1 rustup (Rust Toolchain Installer)

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# https://rustup.rs/ 다운로드 후 설치
```

**rustup이 설치하는 것:**
- `rustc`: Rust 컴파일러
- `cargo`: 패키지 매니저이자 빌드 시스템
- `rustfmt`: 코드 포맷터
- `rust-analyzer`: 언어 서버 (IDE 지원)

#### 5.1.2 설치 확인

```bash
# 버전 확인
rustc --version
cargo --version

# 업데이트
rustup update

# 문서 열기
rustup doc --book
```

### 5.2 Cargo 프로젝트 생성

#### 5.2.1 새 프로젝트 생성

```bash
# 새 바이너리 프로젝트 생성
cargo new dungeon_crawler

# 또는 기존 디렉토리에서 초기화
mkdir dungeon_crawler
cd dungeon_crawler
cargo init
```

**생성된 프로젝트 구조:**

```
dungeon_crawler/
├── Cargo.toml          # 프로젝트 설정 파일
├── .git/               # Git 저장소
├── .gitignore          # Git 무시 파일
└── src/
    └── main.rs         # 메인 소스 코드
```

#### 5.2.2 Cargo.toml 구조

```toml
[package]
name = "dungeon_crawler"     # 프로젝트 이름
version = "0.1.0"            # 버전 (SemVer)
edition = "2024"             # Rust 에디션
authors = ["Your Name"]      # 작성자

[dependencies]
# 외부 라이브러리 의존성
```

### 5.3 개발 도구

#### 5.3.1 VS Code 확장

추천 확장:
- **rust-analyzer**: 공식 Rust 언어 서버
- **CodeLLDB**: 디버거
- **Even Better TOML**: Cargo.toml 지원
- **Error Lens**: 인라인 에러 표시

#### 5.3.2 유용한 Cargo 명령어

```bash
# 빌드 (디버그 모드)
cargo build

# 빌드 (릴리즈 모드 - 최적화)
cargo build --release

# 실행
cargo run

# 실행 (릴리즈 모드)
cargo run --release

# 체크 (컴파일만 확인, 바이너리 생성 안 함)
cargo check

# 테스트
cargo test

# 문서 생성
cargo doc --open

# 포맷팅
cargo fmt

# 린트 (Linter)
cargo clippy
```

### 5.4 개발 워크플로우

```
1. 코드 작성
2. cargo check로 빠르게 확인
3. cargo run으로 실행
4. cargo test로 테스트
5. cargo fmt로 포맷팅
6. cargo clippy로 린트 확인
7. git commit으로 커밋
```

---

## 6. Hello World 프로그램 분석

### 6.1 첫 번째 Rust 프로그램

```rust
fn main() {
    println!("Hello, world!");
}
```

### 6.2 상세 분석

#### 6.2.1 `fn main() { }`

```rust
fn main() {
// ^^^^^ 함수 정의 키워드와 이름
//      main은 프로그램의 진입점
}
```

- `fn`: 함수 정의 키워드 (function의 약어)
- `main`: 특별한 함수 이름 - 프로그램 실행 시작점
- `()`: 매개변수가 없음
- `{ }`: 함수 본체 (block)

**main 함수의 특징:**
- 항상 첫 번째로 실행됨
- 반환 타입은 명시하지 않으면 `()`
- 명시적으로 반환하려면 `()` 타입 사용

```rust
fn main() -> () {  // 위와 동일
    println!("Hello, world!");
}
```

#### 6.2.2 `println!("Hello, world!");`

```rust
println!("Hello, world!");
//^^^^^^^ 매크로 이름
//       ^^^^^^^^^^^^^^^ 문자열 리터럴
//                       ! 매크로 호출임을 나타냄
//                        ; 문장 종료
```

- `println!`: 매크로 (함수 아님!)
- `!`: 매크로 호출임을 나타내는 Rust 문법
- `;`: 세미콜론 (문장 종료)

### 6.3 함수 vs 매크로

| 특성 | 함수 | 매크로 |
|------|------|--------|
| 호출 | `func()` | `macro!()` |
| 인자 개수 | 고정됨 | 가변적 |
| 컴파일 시점 | 컴파일 타임에 호출 | 컴파일 타임에 확장 |
| 용도 | 재사용 가능한 코드 | 메타프로그래밍 |

### 6.4 주석

```rust
// 한 줄 주석

/// 문서 주석 (doc comment)
/// 주로 함수/구조체 앞에서 사용

/*
 * 여러 줄 주석
 */

//! 모듈 레벨 문서 주석
//! crate 루트에서 사용
```

---

## 7. println! 매크로 상세 분석

### 7.1 기본 사용법

```rust
fn main() {
    // 간단한 문자열 출력
    println!("Hello, world!");

    // 여러 줄 출력
    println!("Line 1");
    println!("Line 2");
}
```

### 7.2 문자열 보간 (String Interpolation)

```rust
fn main() {
    let name = "철수";
    let level = 5;

    // {} 플레이스홀더
    println!("용사: {}", name);
    println!("레벨: {}", level);

    // 여러 개의 플레이스홀더
    println!("{} 레벨 {}의 용사", name, level);

    // 인자 번호로 순서 지정
    println!("{1} 레벨 {0}의 용사", level, name);
    // 출력: 철수 레벨 5의 용사
}
```

### 7.3 포맷팅 옵션

```rust
fn main() {
    let number = 42;

    // 다양한 진법
    println!("10진수: {}", number);      // 42
    println!("2진수: {:b}", number);     // 101010
    println!("8진수: {:o}", number);     // 52
    println!("16진수: {:x}", number);    // 2a
    println!("16진수(대문자): {:X}", number);  // 2A

    // 정밀도
    let pi = 3.14159;
    println!("기본: {}", pi);            // 3.14159
    println!("소수점 2자리: {:.2}", pi); // 3.14

    // 정렬과 패딩
    println!("오른쪽 정렬: |{:5}|", 12);     // |   12|
    println!("왼쪽 정렬: |{:<5}|", 12);      // |12   |
    println!("가운데 정렬: |{:^5}|", 12);     // | 12  |

    // 패딩 문자
    println!("0으로 패딩: {:05}", 12);       // 00012

    // + 기호 표시
    println!("+ 표시: {:+}", 12);            // +12
    println!("+ 표시: {:+}", -12);           // -12
}
```

### 7.4 디버깅 포맷

```rust
fn main() {
    // :? - Debug 포맷 (derive Debug 필요 없는 기본 타입)
    println!("디버그: {:?}", (42, "hello"));

    // :#? - 예쁜 Debug 포맷 (여러 줄)
    let v = vec![1, 2, 3];
    println!("벡터: {:#?}", v);
    // 출력:
    // [
    //     1,
    //     2,
    //     3,
    // ]
}
```

### 7.5 특수 문자

```rust
fn main() {
    // 이스케이프 시퀀스
    println!("줄바꿈\n입니다");
    println!("탭\t입니다");
    println!("큰따옴표\"입니다");
    println!("백슬래시\\입니다");

    // 원시 문자열 (Raw string literal)
    println!(r"C:\Users\name");  // 이스케이프 처리 안 함

    // 원시 문자열 (따옴표 포함)
    println!(r#"이것은 "따옴표" 포함"#);
}
```

### 7.6 println!의 내부 동작

```rust
// println!은 실제로 다음과 같이 확장됨

// 원본 코드:
println!("Hello, {}", name);

// 컴파일러가 확장한 코드:
use std::io::Write;
{
    let mut _arg0 = match "Hello, {}" {
        _ => ::std::fmt::ArgumentV1::new(name, ::std::fmt::Display::fmt),
    };
    let _locales = ::std::fmt::Arguments::new_v1(
        &["Hello, "],
        &[_arg0],
    );
    ::std::io::_print(_locales);
}
```

---

## 8. 과제 #1: 게임 시작 화면 구현

### 8.1 요구사항

1. 게임 제목 "DUNGEON CRAWLER" 출력
2. 장식선(`=`)으로 테두리 출력
3. 개발자 정보 출력 (본인 이름, 학번)
4. "Press Enter to start..." 메시지

### 8.2 구현 예시

```rust
fn main() {
    // 상단 장식선
    println!("====================================");

    // 게임 제목 (가운데 정렬)
    println!("         DUNGEON CRAWLER");

    // 하단 장식선
    println!("====================================");
    println!();

    // 개발자 정보
    println!("Developed by: 홍길동 (202412345)");
    println!("             컴퓨터 공학과");
    println!();

    // 시작 메시지
    println!("Press Enter to start...");
}
```

### 8.3 실행 결과

```
====================================
         DUNGEON CRAWLER
====================================

Developed by: 홍길동 (202412345)
             컴퓨터 공학과

Press Enter to start...
```

### 8.4 추가 개선 아이디어

```rust
fn main() {
    // 컬러 출력 (터미널 지원 시)
    // ANSI 색상 코드 사용
    println!("\x1b[33m====================================\x1b[0m"); // 노란색
    println!("\x1b[33m         DUNGEON CRAWLER\x1b[0m");
    println!("\x1b[33m====================================\x1b[0m");
    println!();

    println!("Developed by: 홍길동 (202412345)");
    println!("             컴퓨터 공학과");
    println!();

    println!("\x1b[36mVersion: 1.0.0-alpha\x1b[0m"); // 청록색
    println!();
    println!("Press Enter to start...");
}
```

### 8.5 프로젝트 저장

```bash
# Git 초기화
git add .
git commit -m "feat: Week 1 - 게임 제목 화면 구현"

# 원격 저장소에 푸시 (선택사항)
git remote add origin https://github.com/username/dungeon-crawler.git
git push -u origin main
```

---

## 요약 및 핵심 개념

### 핵심 개념 정리

| 개념 | 설명 |
|------|------|
| **컴퓨터 과학** | 문제 해결과 계산에 관한 학문 |
| **컴파일러** | 소스 코드를 기계어로 번역하는 프로그램 |
| **Rust** | 메모리 안전성과 고성능을 갖춘 현대 언어 |
| **Cargo** | Rust의 패키지 매니저이자 빌드 시스템 |
| **fn main()** | 프로그램의 진입점 함수 |
| **println!** | 텍스트 출력 매크로 |

### 다음 단계

제1주를 완료한 후, 다음 주차에서는 다음을 학습합니다:
- **제2주**: 변수와 기본 타입 - 플레이어 정보 저장하기
- 변수, 상수, 타입 시스템
- 캐릭터 생성 화면 구현

---

## 참고 자료

### 공식 문서
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

### 학습 자료
- [Rust Cheatsheet](https://cheats.rs/)
- [Rustlings](https://rustlings.cool/) - 연습 문제
- [Rust Playground](https://play.rust-lang.org/)

### 커뮤니티
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust subreddit](https://reddit.com/r/rust)
- [Rust Users Forum](https://users.rust-lang.org/)

---

*연구 문서 작성일: 2026-03-08*
*작성자: Claude*
*버전: 1.0*
