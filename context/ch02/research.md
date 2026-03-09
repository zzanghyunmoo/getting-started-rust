# 변수와 기본 타입 심층 연구

> 본 문서는 Rust 프로그래밍 언어의 변수 시스템과 기본 타입을 깊이 있게 연구한 결과입니다. 변수의 불변성, 타입 시스템, 메모리 표현, 타입 추론 등 다양한 기술적 특징을 분석합니다.

---

## 목차

1. [변수와 불변성](#1-변수와-불변성)
2. [상수 (Constants)](#2-상수-constants)
3. [섀도잉 (Shadowing)](#3-섀도잉-shadowing)
4. [기본 데이터 타입](#4-기본-데이터-타입)
5. [타입 추론과 어노테이션](#5-타입-추론과-어노테이션)
6. [타입 변환과 Coercion](#6-타입-변환과-coercion)
7. [복합 타입 기초](#7-복합-타입-기초)
8. [타입 시스템의 안전성 보장](#8-타입-시스템의-안전성-보장)

---

## 1. 변수와 불변성

### 1.1 변수 선언 문법

Rust에서 변수는 `let` 키워드로 선언합니다:

```rust
let x = 5;
```

**컴파일러 관점에서의 분석:**

```
소스 코드: let x = 5;
    ↓
HIR (High-level IR):
    let x: i32 = const 5_i32;
    ↓
MIR (Mid-level IR):
    storage_live(x)
    x = const 5_i32
    ↓
LLVM IR:
    %x = alloca i32
    store i32 5, ptr %x
```

### 1.2 불변성 (Immutability)의 기본

Rust의 변수는 **기본적으로 불변(immutable)**입니다:

```rust
let x = 5;
println!("x의 값: {}", x);

// x = 10;  // 컴파일 에러!
// error[E0384]: cannot assign twice to immutable variable `x`
```

**불변성의 형식적 정의:**

```
∀ 변수 v, v가 불변으로 선언됨 → v의 값은 초기화 이후 변경 불가
```

**불변성이 필요한 이유:**

1. **메모리 안전성**: 예기치 않은 변경 방지
2. **동시성 안전성**: 데이터 레이스 방지
3. **컴파일러 최적화**: 값이 변경되지 않음을 보장하여 공격적인 최적화 가능

### 1.3 가변 변수 (Mutable Variables)

`mut` 키워드로 가변 변수를 선언합니다:

```rust
let mut x = 5;
println!("x의 값: {}", x);
x = 10;
println!("x의 새로운 값: {}", x);
```

**mut 키워드의 의미:**

| 특성 | 불변 변수 | 가변 변수 |
|------|-----------|-----------|
| 선언 | `let x = 5;` | `let mut x = 5;` |
| 값 변경 | 불가능 | 가능 |
| 대여 규칙 | 여러 불변 대여 가능 | 단일 가변 대여만 가능 |
| 스레드 안전 | `Sync` 자동 구현 | `Sync` 수동 구현 필요 |

**mut의 메모리 표현:**

```rust
let mut x: i32 = 5;
// 메모리: [스택 주소] = 5

x = 10;
// 메모리: [같은 스택 주소] = 10 (값 덮어쓰기)

// 불변 변수의 경우:
let y: i32 = 5;
// y = 10;  // 컴파일 타임에 차단
```

### 1.4 불변성의 내부 구현

Rust 컴파일러는 **BORROW CHECKER**를 통해 불변성을 강제합니다:

```rust
// 컴파일러 내부 표현 (개념적)
struct Variable {
    name: &'static str,
    is_mutable: bool,
    value: Value,
    borrow_count: usize,
}

// 불변 변수인 경우:
let x = 5;
// Variable { name: "x", is_mutable: false, value: 5, borrow_count: 0 }

// x = 10 시도 시:
// if !x.is_mutable {
//     throw CompileError("cannot assign to immutable variable");
// }
```

---

## 2. 상수 (Constants)

### 2.1 상수 선언 문법

```rust
const MAX_POINTS: u32 = 100_000;
```

**상수의 필수 요소:**
1. `const` 키워드
2. **반드시 타입 어노테이션** 필수
3. 상수 이름은 **UPPER_SNAKE_CASE**
4. 컴파일 타임에 계산 가능한 값만 할당 가능

### 2.2 상수 vs 불변 변수

| 특성 | 상수 (`const`) | 불변 변수 (`let`) |
|------|----------------|-------------------|
| 타입 어노테이션 | 필수 | 선택적 (추론 가능) |
| 할당 시점 | 컴파일 타임 | 런타임 |
| 값 범위 | 상수 표현식만 | 모든 표현식 |
| 메모리 위치 | 코드 섹션 (inline) | 스택/데이터 섹션 |
| 스코프 | 전역 또는 블록 | 블록 |
| 최적화 | 완전히 인라인 | 최적화에 따라 다름 |

**예시:**

```rust
// 상수 - 컴파일 타임에 결정
const MAX_SIZE: usize = 100;

// 불변 변수 - 런타임에 결정 가능
let size = get_user_input();  // 함수 호출 결과
```

### 2.3 상수의 컴파일 타임 평가

```rust
const PI: f64 = 3.14159265359;
const CIRCLE_AREA: f64 = PI * 2.0 * 2.0;  // 컴파일 타임 계산

// 함수 호출은 불가능
// const NOW: DateTime = Utc::now();  // 에러! 런타임 함수
```

**상수 평가 가능성:**

```rust
// ✅ 가능: 리터럴
const A: i32 = 5;

// ✅ 가능: 상수 표현식
const B: i32 = 5 + 3;

// ✅ 가능: 다른 상수 참조
const C: i32 = B * 2;

// ❌ 불가능: 런타임 값
// const D: i32 = get_value();  // 에러

// ❌ 불가능: 힙 할당
// const E: String = String::from("hello");  // 에러
```

### 2.4 상수의 메모리 레이아웃

```rust
static GLOBAL_CONST: i32 = 100;

fn main() {
    const LOCAL_CONST: i32 = 200;

    // 메모리 레이아웃:
    // .rodata (읽기 전용 데이터 섹션):
    //   GLOBAL_CONST: 4 bytes
    //
    // 스택 프레임:
    //   (LOCAL_CONST는 컴파일 타임에 인라인화)
}
```

---

## 3. 섀도잉 (Shadowing)

### 3.1 섀도잉의 정의와 문법

섀도잉은 **이전 변수와 같은 이름의 새 변수를 선언**하는 것입니다:

```rust
let x = 5;
let x = x + 1;  // 섀도잉
let x = x * 2;  // 또 다른 섀도잉

println!("x의 최종 값: {}", x);  // 12
```

### 3.2 섀도잉 vs mut

| 특성 | 섀도잉 | `mut` |
|------|--------|-------|
| 문법 | `let x = x + 1;` | `x = x + 1;` |
| 값 변경 | 새 변수 생성 | 기존 변수 변경 |
| 타입 변경 | 가능 | 불가능 |
| 불변성 유지 | 가능 | 불가능 |
| 스코프 | 새로운 스코프 시작 | 기존 스코프 유지 |

**예시:**

```rust
// 섀도잉 - 타입 변경 가능
let spaces = "   ";      // &str
let spaces = spaces.len(); // usize

// mut - 타입 변경 불가능
let mut spaces = "   ";
// spaces = spaces.len();  // 컴파일 에러!
```

### 3.3 섀도잉의 스코프 규칙

```rust
fn main() {
    let x = 5;           // 스코프 1 시작

    {
        let x = x * 2;   // 스코프 2 시작 (내부 섀도잉)
        println!("내부 x: {}", x);  // 10
    }                    // 스코프 2 종료

    println!("외부 x: {}", x);      // 5
}
```

**섀도잉과 스코프의 관계:**

```
스코프 레벨 0 (전역):
    let x = 5;

    스코프 레벨 1:
        let x = x + 1;  // 레벨 0의 x 섀도잉

        스코프 레벨 2:
            let x = x * 2;  // 레벨 1의 x 섀도잉
```

### 3.4 섀도잉의 내부 표현

```rust
// 원본 코드
let x = 5;
let x = x + 1;

// 컴파일러 내부 표현 (개념적)
{
    let x_0: i32 = 5;
    {
        let x_1: i32 = x_0 + 1;  // 새 변수
        // x_1이 여기서 사용됨
    }
}
```

### 3.5 섀도잉의 실제 사용 사례

```rust
// 1. 데이터 변환 파이프라인
let input = "42";
let number = input.trim();              // "42"
let number = number.parse::<i32>().unwrap();  // 42

// 2. 조건부 초기화
let config = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};

let config = match env::var("CONFIG") {
    Ok(v) => v,
    Err(_) => config.to_string(),  // 기본값 섀도잉
};
```

---

## 4. 기본 데이터 타입

### 4.1 타입 시스템 개요

Rust는 **정적 타이핑(Static Typing)** 언어입니다:

```
정적 타이핑:
    컴파일 타임에 모든 변수의 타입 결정
    런타임 타입 검사 없음 (또는 최소화)
    타입 안전성 보장

강 타이핑(Strong Typing):
    암시적 타입 변환 없음
    명시적 변환만 허용
```

### 4.2 정수형 (Integer Types)

**부호 있는 정수형:**

| 타입 | 크기 | 범위 |
|------|------|------|
| `i8` | 8-bit | -128 ~ 127 |
| `i16` | 16-bit | -32,768 ~ 32,767 |
| `i32` | 32-bit | -2,147,483,648 ~ 2,147,483,647 |
| `i64` | 64-bit | -9,223,372,036,854,775,808 ~ 9,223,372,036,854,775,807 |
| `i128` | 128-bit | -170,141,183,460,469,231,731,687,303,715,884,105,728 ~ 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| `isize` | arch | CPU 아키텍처에 따름 |

**부호 없는 정수형:**

| 타입 | 크기 | 범위 |
|------|------|------|
| `u8` | 8-bit | 0 ~ 255 |
| `u16` | 16-bit | 0 ~ 65,535 |
| `u32` | 32-bit | 0 ~ 4,294,967,295 |
| `u64` | 64-bit | 0 ~ 18,446,744,073,709,551,615 |
| `u128` | 128-bit | 0 ~ 340,282,366,920,938,463,463,374,607,431,768,211,455 |
| `usize` | arch | CPU 아키텍처에 따름 |

**정수형의 메모리 표현:**

```rust
let x: i32 = 42;
// 메모리 (Little Endian):
// [0x2A] [0x00] [0x00] [0x00]

let y: i32 = -42;
// 메모리 (2의 보수):
// [0xD6] [0xFF] [0xFF] [0xFF]
```

**정수 리터럴 표기법:**

```rust
let decimal = 98_222;           // 10진수
let hex = 0xff;                 // 16진수
let octal = 0o77;               // 8진수
let binary = 0b1111_0000;       // 2진수
let byte = b'A';                // u8 타입 (ASCII만)
```

**정수 오버플로우 처리:**

```rust
// 디버그 모드: 패닉
let x: u8 = 255;
// let y: u8 = x + 1;  // panic: arithmetic overflow

// 릴리즈 모드: 래핑 (wrapping)
// y = 0 (2의 8승으로 나눈 나머지)

// 명시적 처리:
let y = x.wrapping_add(1);      // 0
let y = x.checked_add(1);       // None
let y = x.saturating_add(1);    // 255
let y = x.overflowing_add(1);   // (0, true)
```

### 4.3 부동소수점형 (Floating-Point Types)

**타입:**

| 타입 | 크기 | 정밀도 | 표준 |
|------|------|--------|------|
| `f32` | 32-bit | 단일 정밀도 | IEEE 754-2008 |
| `f64` | 64-bit | 배정밀도 (기본값) | IEEE 754-2008 |

**부동소수점 메모리 표현 (IEEE 754):**

```rust
let x: f32 = 3.14159;
// 비트 구성:
// [부호: 1bit] [지수: 8bits] [가수: 23bits]

let y: f64 = 3.14159;
// 비트 구성:
// [부호: 1bit] [지수: 11bits] [가수: 52bits]
```

**특수 값:**

```rust
let inf = f64::INFINITY;     // ∞
let neg_inf = f64::NEG_INFINITY;  // -∞
let nan = f64::NAN;          // NaN (Not a Number)
let max = f64::MAX;          // 최대값
let min = f64::MIN;          // 최소값 (음수)
```

**부동소수점 비교 주의사항:**

```rust
// 직접 비교는 위험
let result = 0.1 + 0.2;  // 0.30000000000000004
// if result == 0.3 { }  // false!

// 근사값 비교
fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

// 또는 f64::EPSILON 사용
(result - 0.3).abs() < f64::EPSILON
```

### 4.4 불리언형 (Boolean Type)

```rust
let t: bool = true;
let f: bool = false;

// 메모리: 1 byte (비트가 아님!)
// 패딩으로 인해 실제로 1바이트 사용
```

**불리언의 메모리 표현:**

```rust
// 메모리 레이아웃
struct BoolRep {
    value: u8,  // 0 = false, 1 = true
}

// 컴파일러 최적화로 비트 단위로 압축 가능
// [bool; 8] → 1 byte (not 8 bytes)
```

**불리언 연산:**

```rust
// 논리 연산
let a = true && false;  // false (AND)
let b = true || false;  // true (OR)
let c = !true;          // false (NOT)

// 단락 평가 (Short-circuit evaluation)
let x = false || expensive_function();  // expensive_function() 호출됨
let y = true && expensive_function();   // expensive_function() 호출됨
let z = false && expensive_function();  // 호출 안 됨!
```

### 4.5 문자형 (Character Type)

```rust
let c: char = 'z';
let emoji = '😻';

// 내부적으로 Unicode Scalar Value
// 4 bytes (32 bits)
```

**문자의 메모리 표현:**

```rust
let c: char = 'A';
// 내부 표현: u32 값
// U+0041 → 0x00000041 (65)

let emoji = '😻';
// 내부 표현: U+1F63B
// 0x0001F63B (128539)
```

**문자 vs 문자열:**

```rust
let c: char = 'a';        // 4 bytes, 1 character
let s: &str = "a";        // 슬라이스 (ptr + len)

let emoji: char = '😀';   // 4 bytes
let emoji_str = "😀";     // 4 bytes (UTF-8)
```

**문자 유효성:**

```rust
// 유효한 Unicode 문자
let valid = 'A';
let emoji = '😀';
let korean = '한';

// 유효하지 않은 값 (컴파일 에러)
// let invalid = '\0';     // 널 문자는 허용
// let surrogate = '\uD800';  // surrogate 쌍의 절반만 있으면 에러
```

---

## 5. 타입 추론과 어노테이션

### 5.1 타입 추론 (Type Inference)

Rust는 **Hindley-Milner** 스타일의 타입 추론을 사용합니다:

```rust
// 타입 추론
let x = 5;        // i32로 추론 (기본 정수형)
let y = 3.14;     // f64로 추론 (기본 부동소수점형)
```

**추론 알고리즘의 기본 원리:**

```
1. 변수 선언 시 타입 변수 T 도입
2. 할당된 값의 타입에서 T 제약 조건 수집
3. 제약 조건 해결로 T 결정
4. 제약 조건을 해결할 수 없으면 컴파일 에러
```

**복잡한 추론 예시:**

```rust
// 컴파일러의 추론 과정
let x = 5;           // x: ?T
let y = x + 1;       // + 연산에서 ?T = i32 (정수 리터럴의 기본)

let vec = vec![1, 2, 3];  // vec: Vec<?T>
                           // 요소가 i32 → vec: Vec<i32>
```

### 5.2 타입 어노테이션 (Type Annotation)

**필수적인 경우:**

```rust
// 1. 모호한 경우
let guess: u32 = "42".parse().expect("Not a number!");
// .parse()는 제네릭 → 타입 명시 필요

// 2. 다른 타입으로 추론되는 것을 방지
let x: i64 = 5;  // i32가 아닌 i64로 명시

// 3. 문서화 목적
let max_connections: usize = 100;
```

**탑 레벨 함수의 파라미터:**

```rust
// 무조건 타입 어노테이션 필요
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// 불가능
// fn add(x, y) { }  // 에러!
```

### 5.3 타입 추론의 한계

**추론이 불가능한 경우:**

```rust
// 1. 제네릭 메서드
let x = "42".parse();
// 에러: type annotations needed

// 해결
let x: i32 = "42".parse().unwrap();

// 2. 빈 컬렉션
let vec = Vec::new();
// 에러: type annotations needed

// 해결
let mut vec: Vec<i32> = Vec::new();
// 또는
let mut vec = Vec::<i32>::new();
// 또는
let mut vec = Vec::new();
vec.push(1);  // 이후 추론 가능
```

### 5.4 타입 추론 최적화

**인라인화와 최적화:**

```rust
// 원본 코드
let x: i32 = 5;
let y = x + 10;

// 최적화 후 (개념적)
// x와 y가 상수로 인라인화될 수 있음
const Y: i32 = 15;
```

---

## 6. 타입 변환과 Coercion

### 6.1 명시적 타입 변환 (Casting)

**`as` 키워드:**

```rust
let decimal = 65.4_f32;
let integer = decimal as u8;  // 65

let character = integer as char;  // 'A'
```

**타입 변환 규칙:**

```rust
// 안전한 변환
let x: i32 = 100;
let y: i64 = x as i64;  // 값 보존

// 손실 가능 변환
let x: i32 = 300;
let y: u8 = x as u8;  // 44 (300 % 256)

// 불가능한 변환
// let x: i32 = 65;
// let y: f32 = x as f32;  // 가능하지만 정밀도 손실 가능

// 크기 변환
let x: i32 = 1;
let y: i64 = x as i64;  // 확장 (sign extension)
let z: i16 = x as i16;  // 축소 (truncation)
```

**포인터 변환:**

```rust
let x: u32 = 42;
let p: *const u8 = &x as *const u32 as *const u8;
// 포인터 크기는 같지만 reinterpret cast
```

### 6.2 수직 타입 변환 (Type Coercion)

Rust는 특정 상황에서 자동으로 타입을 변환합니다:

**수직 변환 사이트:**

1. **함수 인자**

```rust
fn foo(x: i32) { }

let y: i8 = 10;
foo(y);  // i8에서 i32로 자동 변환
```

2. **메서드 수신자**

```rust
let x: &[i8] = &[1, 2, 3];
let y: &[i32] = x;  // 에러! 실제로는 불가능
// 그러나 &mut T에서 &T로는 가능
```

3. **미리 정의된 수직 변환**

```rust
// T to *const T
let x: i32 = 5;
let p: *const i32 = &x as *const i32;

// &mut T to &T
let mut x = 5;
let r1: &mut i32 = &mut x;
let r2: &i32 = r1;  // 불변 참조로 변환
```

### 6.3 수치 변환 테이블

| 원본 타입 | as U8 | as U16 | as I32 | as I64 | as F32 | as F64 |
|-----------|-------|--------|--------|--------|--------|--------|
| i8 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| u8 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| i32 | ⚠️ | ✅ | ✅ | ✅ | ⚠️ | ✅ |
| f64 | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ |

- ✅: 전체 범위 변환 가능
- ⚠️: 범위 초과 시 잘림

---

## 7. 복합 타입 기초

### 7.1 튜플 (Tuple)

**선언과 사용:**

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// 패턴 매칭으로 해체
let (x, y, z) = tup;

// 인덱스로 접근
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;
```

**튜플의 메모리 레이아웃:**

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// 메모리 (padding 포함):
// [i32: 4 bytes] [padding: 4 bytes] [f64: 8 bytes] [u8: 1 byte] [padding: 7 bytes]
// 총 24 bytes

// 메모리 정렬 확인
use std::mem;
assert_eq!(mem::size_of::<(i32, f64, u8)>(), 24);
```

**유닛 타입 ():**

```rust
let unit: () = ();
// 길이가 0인 튜플
// 크기: 0 bytes

// 용도: 반환값이 없는 함수
fn nothing() -> () {
    ()
}
```

### 7.2 배열 (Array)

**선언과 사용:**

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
// [타입; 길이]

let b = [3; 5];  // [3, 3, 3, 3, 3]
```

**배열의 메모리 레이아웃:**

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// 메모리:
// [1] [2] [3] [4] [5]
//  각각 4 bytes
//  총 20 bytes (contiguous)

// 스택에 할당됨
```

**배열 접근과 안전성:**

```rust
let a = [1, 2, 3, 4, 5];

let first = a[0];   // 안전
// let out = a[10]; // 런타임 패닉!

// 안전한 접근 방법
match a.get(10) {
    Some(&value) => println!("{}", value),
    None => println!("인덱스 초과"),
}
```

**배열의 불변성:**

```rust
let mut a = [1, 2, 3];
a[0] = 10;  // 가능

let a = [1, 2, 3];
// a[0] = 10;  // 컴파일 에러
```

---

## 8. 타입 시스템의 안전성 보장

### 8.1 컴파일 타임 타입 검사

```rust
// 타입 불일치 컴파일 에러
let x: i32 = 5;
let y: f64 = x;  // 에러! 명시적 변환 필요
```

**검사 과정:**

```
소스 코드 분석
    ↓
타입 추론 및 제약 조건 수집
    ↓
타입 제약 조건 검증
    ↓
  검증 통과 → 컴파일 계속
  검증 실패 → 컴파일 에러
```

### 8.2 타입 기반 메모리 안전성

**버퍼 오버플로우 방지:**

```rust
let arr = [1, 2, 3];
// arr[3]  // 컴파일 타임에는 감지 안 됨
// 하지만 런타임에 패닉으로 방지

// 컴파일 타임 방지 (const)
const ARR: [i32; 3] = [1, 2, 3];
// ARR[3]  // 컴파일 에러!
```

### 8.3 타입 기반 최적화

**제로 비용 추상화:**

```rust
// 고수준 코드
let x: Vec<i32> = vec![1, 2, 3];

// 컴파일러 최적화로 인해
// 저수준 코드만큼 빠름
```

**인라인화:**

```rust
#[inline]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 컴파일러가 add 호출을 a + b로 대체
```

---

## 결론

Rust의 변수와 타입 시스템의 핵심 특징:

1. **불변성 기본**: 기본적으로 변수는 불변으로 안전성 보장
2. **명시적 가변성**: `mut` 키워드로 가변성 명시
3. **강 타이핑**: 암시적 변환 없이 명시적 변환만 허용
4. **정적 타이핑**: 컴파일 타임에 모든 타입 결정
5. **타입 추론**: 편리한 사용을 위한 스마트한 추론
6. **메모리 안전성**: 타입 시스템을 통한 메모리 안전성 보장

**다음 학습 주제:**
- 연산자와 표현식
- 제어 흐름 (if, match, loop)
- 함수 기초

---

## 참고 자료

- [The Rust Programming Language - Chapter 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [Rust Reference - Types](https://doc.rust-lang.org/reference/types.html)
- [Rustonomicon - Data Layout](https://doc.rust-lang.org/nomicon/data.html)
