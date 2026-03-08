# Rust 프로그래밍 언어 심층 연구

> 본 문서는 Rust 프로그래밍 언어의 기술적 측면을 깊이 있게 연구한 결과입니다. Rust의 언어 설계, 메모리 모델, 타입 시스템, 동시성, 비동기 프로그래밍 등 다양한 기술적 특징을 분석합니다.

---

## 목차

1. [Rust 언어 개요 및 설계 철학](#1-rust-언어-개요-및-설계-철학)
2. [타입 시스템](#2-타입-시스템)
3. [소유권 시스템 분석](#3-소유권-시스템-분석)
4. [대여(Borrowing) 시스템](#4-대여borrowing-시스템)
5. [수명(Lifetime) 시스템](#5-수명lifetime-시스템)
6. [스마트 포인터](#6-스마트-포인터)
7. [트레이트 시스템](#7-트레이트-시스템)
8. [동시성 프로그래밍](#8-동시성-프로그래밍)
9. [비동기 프로그래밍](#9-비동기-프로그래밍)
10. [메모리 관리 및 레이아웃](#10-메모리-관리-및-레이아웃)
11. [컴파일러 아키텍처](#11-컴파일러-아키텍처)
12. [패턴 매칭 시스템](#12-패턴-매칭-시스템)
13. [고급 기능](#13-고급-기능)

---

## 1. Rust 언어 개요 및 설계 철학

### 1.1 역사 및 배경

Rust는 2006년 Mozilla 연구소의 Graydon Hoare가 개발을 시작했으며, 2010년 오픈 소스로 공개되었고 2015년 1.0 버전이 출시되었습니다. Rust는 시스템 프로그래밍 언어로서 다음과 같은 문제를 해결하는 것을 목표로 설계되었습니다:

1. **메모리 안전성**: 버퍼 오버플로우, 댕글링 포인터, 이중 해제 등의 C/C++ 계열 언어의 문제점 해결
2. **성능**: 가비지 컬렉션의 오버헤드 없는 제로 비용 추상화
3. **동시성**: 데이터 레이스를 컴파일 타임에 방지

### 1.2 핵심 설계 원칙

| 원칙 | 설명 | 구현 방식 |
|------|------|-----------|
| **메모리 안전성** | 컴파일 타임에 메모리 안전성 보장 | 소유권, 대여 규칙, 수명 시스템 |
| **제로 비용 추상화** | 고수준 추상화도 저수준 코드만큼 빠름 | 모노모피제이션, 인라인화 |
| **무공포 동시성** | 데이터 레이스를 컴파일 타임에 감지 | Send/Sync 트레이트 |
| **실용성** | 사용하기 쉽고 강력한 기능 제공 | 패턴 매칭, 트레이트, 매크로 |

### 1.3 Rust가 해결하는 문제

**C/C++의 문제점:**
```c++
// C++: 댕글링 포인터 가능
int* dangling() {
    int x = 42;
    return &x;  // 미정의 동작
}

// C++: 이중 해제 가능
void double_free() {
    int* p = new int(42);
    delete p;
    delete p;  // 미정의 동작
}

// C++: 데이터 레이스 가능
int counter = 0;
void increment() {
    counter++;  // 경쟁 상태
}
```

**Rust의 해결:**
```rust
// Rust: 컴파일 타임에 댕글링 참조 방지
fn dangling() -> &'static i32 {  // 에러!
    let x = 42;
    &x  // x가 drop됨
}

// Rust: 이중 해제 불가능 (소유권 시스템)
fn double_free() {
    let p = Box::new(42);
    drop(p);
    // drop(p);  // 컴파일 에러! p의 소유권이 이미 이동됨
}

// Rust: 데이터 레이스 컴파일 타임 방지
use std::sync::Arc;
use std::sync::Mutex;

let counter = Arc::new(Mutex::new(0));
// Arc<Mutex<_>>는 스레드 안전함
```

---

## 2. 타입 시스템

### 2.1 타입 시스템 분류

Rust는 **정적 타이핑(Static Typing)**과 **강 타이핑(Strong Typing)**을 따릅니다:

```rust
// 정적 타이핑: 컴파일 타임에 타입 결정
let x: i32 = 5;

// 강 타이핑: 암시적 타입 변환 없음
// let y: i64 = x;  // 에러! 명시적 변환 필요
let y: i64 = x as i64;  // 명시적 변환
```

### 2.2 기본 타입 상세 분석

**정수형 계열:**
```
비트 크기  | 부호 있는  | 부호 없는  |
----------|------------|------------|
8-bit     | i8         | u8         |
16-bit    | i16        | u16        |
32-bit    | i32        | u32        |
64-bit    | i64        | u64        |
128-bit   | i128       | u128       |
arch      | isize      | usize      |
```

**부동소수점형:**
- `f32`: IEEE 754-2008 단일 정밀도 (32-bit)
- `f64`: IEEE 754-2008 배정밀도 (64-bit), 기본값

### 2.3 Never 타입 (`!`)

Never 타입은 값을 가질 수 없는 타입입니다:

```rust
fn never_returns() -> ! {
    panic!("This function never returns!");
}

let x: ! = never_returns();  // x는 결코 사용될 수 없음
```

Never 타입은 다음과 같이 사용됩니다:
- `panic!()`, `loop {}`, `break`, `return`의 반환 타입
- 모든 타입의 서브타입

```rust
// loop는 !를 반환하므로 어떤 타입으로도 강변환 가능
let x: i32 = loop {
    break 42;
};
```

### 2.4 타입 추론(Type Inference)

Rust는 **Hindley-Milner** 스타일의 타입 추론을 사용합니다:

```rust
// 기본적인 추론
let x = 5;           // i32로 추론
let y = 3.14;        // f64로 추론

// 타입 어노테이션 필요한 경우
let guess: u32 = "42".parse().expect("Not a number!");

// 제네릭에서의 추론
fn pick<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

let x = pick(1, 2);     // T는 i32로 추론
```

### 2.5 수직 타입 변환 (Coercion)

Rust는 특정 상황에서 자동으로 타입을 변환합니다:

```rust
// 수직 변환 사이트
// 1. 함수 인자
fn foo(_: i32) {}
let x: i8 = 10;
foo(x);  // i8에서 i32로 수직 변환

// 2. 메서드 수신자
let x: &[i8] = &[1, 2, 3];
let y: &[i32] = x;  // &[i8]에서 &[i32]로 변환

// 3. 미리 정의된 수직 변환
// - T to T (식별)
// - T to U if T: CoerceUnsized<U>
// - &mut T to &T
// - *mut T to *T
// - &T to *T
// - &mut T to *mut T
// - &T to &U if T: Unsize<U>
// - Function pointer types
```

---

## 3. 소유권 시스템 분석

### 3.1 소유권 규칙의 형식적 정의

Rust의 소유권 시스템은 다음과 같이 형식화할 수 있습니다:

```
∀ 값 v, ∑ 소유자 o₁, o₂, ..., oₙ (단, n ≤ 1)
∀ 소유자 o, o.유효범위 → drop(o.value)
```

즉, 모든 값은 정확히 하나의 소유자를 가지며, 소유자가 스코프를 벗어나면 값이 자동으로 해제됩니다.

### 3.2 메모리 할당 모델

**스택 메모리 모델:**
```
스택 프레임 구조:
┌─────────────────────────────────┐
│      함수 매개변수                │
├─────────────────────────────────┤
│      반환 주소                   │
├─────────────────────────────────┤
│      로컬 변수 (LIFO 순서)        │
│      - 스택 포인터 (SP)로 관리    │
└─────────────────────────────────┘
```

**힙 메모리 모델:**
```
힙 할당 과정:
1. 할당자(allocator)에게 메모리 요청
2. 할당자가 적절한 힙 블록 찾음
3. 포인터(힙 주소) 반환
4. 포인터를 통해 간접 접근
```

### 3.3 Move 시맨틱스 분석

**값의 이동(Move) 과정:**

```rust
let s1 = String::from("hello");
// 메모리 상태:
// s1: [ptr: 0x1234, len: 5, cap: 5] (스택)
//      0x1234: ['h', 'e', 'l', 'l', 'o'] (힙)

let s2 = s1;
// 메모리 상태:
// s1: [무효화됨]
// s2: [ptr: 0x1234, len: 5, cap: 5] (스택)
//      0x1234: ['h', 'e', 'l', 'l', 'o'] (힙)
```

**Move의 중요성:**
1. **이중 해제 방지**: 한 개의 포인터만 유효함을 보장
2. **메모리 효율성**: 깊은 복사(deep copy) 방지
3. **명시성**: 소유권 이전이 명시적임

### 3.4 Copy 트레이트 구현 조건

```rust
pub trait Copy: Clone {
    // Copy는 마커 트레이트: 메서드 없음
}
```

**Copy 구현 가능 조건:**
1. 타입 자체가 `Copy`를 구현해야 함
2. 타입의 모든 필드가 `Copy`를 구현해야 함
3. `Drop` 트레이트를 구현하지 않아야 함

```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// 컴파일러가 자동으로 구현:
impl Copy for Point {}
impl Clone for Point {
    fn clone(&self) -> Self {
        *self  // 단순 비트 복사
    }
}
```

### 3.5 Drop 순서와 RAII

```rust
struct Droppable<'a> {
    name: &'a str,
}

impl<'a> Drop for Droppable<'a> {
    fn drop(&mut self) {
        println!("Dropping: {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };
    let _b = Droppable { name: "b" };
    let _c = Droppable { name: "c" };
}
// 출력:
// Dropping: c
// Dropping: b
// Dropping: a
// (선언의 역순으로 drop)
```

---

## 4. 대여(Borrowing) 시스템

### 4.1 대여 규칙의 형식적 정의

어떤 스코프에서도 다음 중 하나만 허용됩니다:

```
∀ 스코프 S, ∀ 값 v:
    (∀ n∈ℕ: n 개의 불변 참조 &v) ∨
    (∄: 단일 가변 참조 &mut v)
```

이는 **대여 규칙(Borrowing Rules)**이라고 불립니다.

### 4.2 대여 규칙의 실제 구현

Rust 컴파일러의 Borrow Checker는 다음을 검사합니다:

```rust
let mut x = 5;

// 불변 참조 여러 개 가능
let r1 = &x;  // 'a 시작
let r2 = &x;  // 'b 시작
// 두 불변 참조가 동시에 유효

let r3 = &mut x;  // 에러! r1, r2가 여전히 유효함

println!("{} and {}", r1, r2);  // r1, r2 사용 종료
let r3 = &mut x;  // 이제 가능
```

### 4.3 수명과 대여의 관계

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 수명 변수 'a는 x와 y의 수명의 교집합
// 'a = min( lifetime(x), lifetime(y) )
```

### 4.4 NLL (Non-Lexical Lifetimes)

Rust 2018 이후에서는 **비어휘적 수명(Non-Lexical Lifetimes)**이 적용됩니다:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);
// r1, r2가 여기서 더 이상 사용되지 않음을 컴파일러가 감지

let r3 = &mut s;  // 이제 가능 (NLL 덕분에)
```

NLL 이전에는 r1, r2의 수명이 블록 끝까지 연장되었으나, NLL은 참조가 마지막으로 사용되는 지점까지만 수명을 확장합니다.

---

## 5. 수명(Lifetime) 시스템

### 5.1 수명 파라미터의 의미

수명 파라미터는 참조가 유효한 범위를 나타냅니다:

```rust
// 'a는 수명 파라미터
fn first_word<'a>(s: &'a str) -> &'a str {
    // 반환 값의 수명은 입력의 수명과 동일해야 함
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];  // &s는 수명 'a를 가짐
        }
    }

    &s[..]  // &s는 수명 'a를 가짐
}
```

### 5.2 수명 생략(Lifetime Elision) 규칙

컴파일러가 수명을 추론하는 3가지 규칙:

**규칙 1**: 각 참조 파라미터는 자신만의 수명 파라미터를 가짐

```rust
// 명시적으로
fn foo<'a, 'b>(x: &'a i32, y: &'b i32)

// 생략 가능
fn foo(x: &i32, y: &i32)
```

**규칙 2**: 입력 파라미터가 하나이면, 그 수명이 모든 출력에 적용

```rust
// 명시적으로
fn foo<'a>(x: &'a i32) -> &'a i32

// 생략 가능
fn foo(x: &i32) -> &i32
```

**규칙 3**: 메서드의 `&self`나 `&mut self`가 있으면, self의 수명이 모든 출력에 적용

```rust
// 명시적으로
fn method<'a, 'b>(&'a self, x: &'b i32) -> &'a i32

// 생략 가능
fn method(&self, x: &i32) -> &i32
```

### 5.3 구조체의 수명 어노테이션

```rust
struct Context<'s> {
    speaker: &'s str,
    message: &'s str,
}

impl<'s> Context<'s> {
    fn new(speaker: &'s str, message: &'s str) -> Self {
        Context { speaker, message }
    }

    fn as_tuple(&self) -> (&'s str, &'s str) {
        (self.speaker, self.message)
    }
}
```

### 5.4 수명 하위 타이핑

```rust
fn example<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    x
}
// 'b: 'a는 "'b는 'a보다 길거나 같아야 함"을 의미
```

### 5.5 정적 수명('static)

```rust
// 프로그램의 전체 수명 동안 유효
static MAX_POINTS: u32 = 100_000;
static HELLO_WORLD: &str = "Hello, world!";

fn static_lifetime() -> &'static str {
    "I have a static lifetime"
}
```

---

## 6. 스마트 포인터

### 6.1 스마트 포인터 vs 일반 참조 비교

| 특성 | 일반 참조 `&T` | 스마트 포인터 |
|------|---------------|--------------|
| 소유권 | 없음 | 있음 |
| Deref 구현 | 자동 | 명시적 필요 |
| 데이터 포함 | 참조만 | 포인터 + 메타데이터 |
| 예시 | `&i32`, `&String` | `Box<i32>`, `Rc<String>` |

### 6.2 Box: 힙 할당 스마트 포인터

```rust
use std::mem::size_of;

// 1. 재귀적 타입 구현
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 2. 큰 데이터를 힙에 할당
let large = Box::new([0; 1_000_000]);

// 3. 트레이트 객체
trait Draw {
    fn draw(&self);
}

struct Button { width: u32, height: u32 }
impl Draw for Button {
    fn draw(&self) { println!("Button: {}x{}", self.width, self.height); }
}

let components: Vec<Box<dyn Draw>> = vec
![
    Box::new(Button { width: 50, height: 10 }),
    Box::new(Button { width: 100, height: 20 }),
];
```

### 6.3 Rc: 참조 카운팅

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1

    let b = List::Cons(10, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2

    {
        let c = List::Cons(15, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}
```

**Rc의 제약사항:**
- 단일 스레드만 지원 (`!Send`, `!Sync`)
- 불변 참조만 제공
- 순환 참조 시 메모리 누수 가능

### 6.4 RefCell과 내부 가변성

```rust
use std::cell::RefCell;

pub struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> Self {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }

    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
        // 불변 참조(&self)를 가지고도 내부를 변경 가능!
    }
}
```

**RefCell의 런타임 대여 규칙 검사:**
```rust
let data = RefCell::new(5);

let r1 = data.borrow();  // 불변 대여
let r2 = data.borrow();  // 또 다른 불변 대여 (허용됨)

let r3 = data.borrow_mut();  // 에러! r1, r2가 여전히 유효
```

### 6.5 Rc<RefCell> 조합 패턴

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // 모든 리스트에서 공유된 값 수정
    *value.borrow_mut() += 10;
}
```

### 6.6 Arc: 스레드 안전 참조 카운팅

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

**Rc vs Arc 비교:**
- `Rc`: 단일 스레드, 비원자적 연산
- `Arc`: 다중 스레드, 원자적 연산 (Atomic Reference Counted)

---

## 7. 트레이트 시스템

### 7.1 트레이트 정의와 특성

```rust
trait Summary {
    // 필수 메서드
    fn summarize(&self) -> String;

    // 기본 구현이 제공된 메서드
    fn summarize_verbose(&self) -> String {
        format!("(Read more from {}...)", self.summarize())
    }
}
```

### 7.2 트레이트 경계(Trait Bounds)의 모든 문법

```rust
// 1. impl 트레이트 문법
fn notify1(item: &impl Summary) {
    println!("{}", item.summarize());
}

// 2. 트레이트 경계 문법
fn notify2<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// 3. 복수 트레이트 경계
fn notify3<T: Summary + Display>(item: &T) {
    println!("{}", item.summarize());
}

// 4. where 절
fn notify4<T>(item: &T)
where
    T: Summary + Display,
{
    println!("{}", item.summarize());
}

// 5. 반환 타입에서의 트레이트 경계
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

### 7.3 연관 타입(Associated Types)

```rust
trait Iterator {
    type Item;  // 연관 타입

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;  // 구체 타입 지정

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

**연관 타입 vs 제네릭:**
```rust
// 연관 타입: 타입당 하나의 구현
trait Container {
    type Item;
    fn get(&self) -> Self::Item;
}

struct Container32(i32);
impl Container for Container32 {
    type Item = i32;
    fn get(&self) -> Self::Item { self.0 }
}

// 제네릭: 타입당 여러 구현 가능
trait ContainerGen<T> {
    fn get(&self) -> T;
}

impl ContainerGen<i32> for Container32 {
    fn get(&self) -> i32 { self.0 }
}

impl ContainerGen<u32> for Container32 {
    fn get(&self) -> u32 { self.0 as u32 }
}
```

### 7.4 트레이트 객체 (dyn Trait)

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button '{}' ({}x{})", self.label, self.width, self.height);
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

**트레이트 객체의 메모리 레이아웃:**
```
Box<dyn Draw>:
┌─────────────────────────┐
│  data pointer          │  데이터 포인터
│  vtable pointer        │  vtable 포인터
└─────────────────────────┘
```

### 7.5 트레이트 상속

```rust
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

### 7.6 마커 트레이트

```rust
// Send: 스레드 간 소유권 이전 가능
unsafe impl<T: Send> !Send for *const T {}

// Sync: 여러 스레드에서 &T 사용 가능
unsafe impl<T: Sync + ?Sized> Sync for &T {}

// Copy: 값 복사 시 move 대신 복사
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// Clone: 명시적 복사 가능
#[derive(Clone)]
struct PointMut {
    x: i32,
    y: i32,
}
```

---

## 8. 동시성 프로그래밍

### 8.1 무공포 동시성 (Fearless Concurrency)

Rust는 타입 시스템을 통해 다음을 보장합니다:
1. 데이터 레이스 방지
2. 댕글링 참조 방지
3. 미정의 동작 방지

### 8.2 Send와 Sync 트레이트

```rust
// Send: 소유권이 스레드 간에 이전 가능
unsafe trait Send {
    // 빈 트레이트 (마커)
}

// Sync: &T가 여러 스레드에서 안전하게 사용 가능
unsafe trait Sync {
    // 빈 트레이트 (마커)
}
```

**Send와 Sync의 관계:**
```
T: Sync ⇔ &T: Send
```

### 8.3 스레드 생성과 join

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread", i);
        thread::sleep(Duration::from_millis(1));
    }
});

for i in 1..5 {
    println!("hi number {} from the main thread", i);
    thread::sleep(Duration::from_millis(1));
}

handle.join().unwrap();  // 스레드 종료 대기
```

### 8.4 Move 클로저와 소유권 전달

```rust
use std::thread;

let v = vec
![1, 2, 3];

// move 키워드로 v의 소유권을 클로저로 이동
let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});

// v는 더 이상 유효하지 않음
// println!("{:?}", v);  // 에러!

handle.join().unwrap();
```

### 8.5 채널(Channel)을 통한 메시지 전달

```rust
use std::sync::mpsc;  // multiple producer, single consumer
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    // val은 여기서 더 이상 유효하지 않음
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```

**채널의 종류:**
- `mpsc::channel`: 비동기 (unbounded)
- `mpsc::sync_channel`: 동기 (bounded)

### 8.6 Mutex를 통한 공유 상태 동기화

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

**Mutex vs RwLock:**
| 특성 | Mutex<T> | RwLock<T> |
|------|-----------|-----------|
| 읽기 | 단일 스레드 | 다중 스레드 |
| 쓰기 | 단일 스레드 | 단일 스레드 |
| 오버헤드 | 낮음 | 높음 |
| 사용 사례 | 쓰기/읽기 빈번 | 읽기 빈번 |

### 8.7 조건 변수(Condvar)

```rust
use std::sync::{Arc, Mutex, Condvar};

let pair = Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = Arc::clone(&pair);

thread::spawn(move || {
    let (lock, cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_one();
});

let (lock, cvar) = &*pair;
let mut started = lock.lock().unwrap();
while !*started {
    started = cvar.wait(started).unwrap();
}
```

### 8.8 Atomics

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);

// 원자적 연산
counter.fetch_add(1, Ordering::SeqCst);

// 메모리 순서 (Memory Ordering):
// - Relaxed: 순서 보장 없음
// - Acquire: 이후 읽기 재배치 방지
// - Release: 이전 쓰기 재배치 방지
// - AcqRel: Acquire + Release
// - SeqCst: 순차적 일관성 (가장 강력)
```

---

## 9. 비동기 프로그래밍

### 9.1 Future 트레이트

```rust
trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

**Future의 작동 원리:**
1. `poll()` 호출
2. 준비되면 `Poll::Ready(value)` 반환
3. 준비되지 않으면 `Poll::Pending` 반환하고 waker 등록
4. waker가 준비되면 다시 `poll()` 호출

### 9.2 Async/Await 문법

```rust
async fn hello_world() {
    println!("hello");
    hello().await;
    println!("world");
}

// 위 코드는 대략 다음으로 변환됨:
fn hello_world() -> impl Future<Output = ()> {
    async move {
        println!("hello");
        hello().await;
        println!("world");
    }
}
```

### 9.3 비동기 함수의 반환 타입

```rust
async fn foo() -> i32 {
    5
}

// 위 코드는 실제로 다음과 같음:
fn foo() -> impl Future<Output = i32> {
    // Future를 반환하는 구체적 타입
}
```

### 9.4 실행기(Executor)와 런타임

```rust
use tokio::time::{sleep, Duration};

async fn task1() {
    sleep(Duration::from_secs(1)).await;
    println!("Task 1 completed");
}

async fn task2() {
    sleep(Duration::from_secs(2)).await;
    println!("Task 2 completed");
}

#[tokio::main]
async fn main() {
    tokio::spawn(task1());
    tokio::spawn(task2());

    sleep(Duration::from_secs(3)).await;
}
```

### 9.5 스트림(Sink/Stream)

```rust
use futures::stream::{self, StreamExt};

async fn sum_stream() {
    let mut stream = stream::iter(1..=10);

    let sum = stream.fold(0, |acc, x| async move {
        acc + x
    }).await;

    println!("Sum: {}", sum);
}
```

### 9.6 비동기 트레이트

```rust
use async_trait::async_trait;

#[async_trait]
trait Executor {
    async fn execute(&self, command: &str) -> Result<String, Error>;
}

struct MyExecutor;

#[async_trait]
impl Executor for MyExecutor {
    async fn execute(&self, command: &str) -> Result<String, Error> {
        Ok(format!("Executed: {}", command))
    }
}
```

---

## 10. 메모리 관리 및 레이아웃

### 10.1 데이터 메모리 정렬(Alignment)

```rust
struct A {
    a: u8,   // offset 0, size 1
    // padding 3 bytes
    b: u32,  // offset 4, size 4
    c: u8,   // offset 8, size 1
    // padding 7 bytes
}  // total: 16 bytes

struct B {
    a: u8,   // offset 0, size 1
    c: u8,   // offset 1, size 1
    // padding 2 bytes
    b: u32,  // offset 4, size 4
}  // total: 8 bytes (최적화됨)
```

### 10.2 Enum의 메모리 표현

```rust
enum Option<T> {
    Some(T),
    None,
}
// 크기: discriminant (1 byte) + max(sizeof(T), padding)

enum IpAddr {
    V4(u8, u8, u8, u8),  // 4 bytes
    V6(String),          // 24 bytes (ptr + len + cap)
}
// 크기: discriminant + padding + max(4, 24) = 25 + padding
```

### 10.3 장소 기민 데이터(Phantom Data)

```rust
use std::marker::PhantomData;

struct Slice<'a, T> {
    start: *const T,
    end: *const T,
    _phantom: PhantomData<&'a T>,
}
```

---

## 11. 컴파일러 아키텍처

### 11.1 컴파일 파이프라인

```
Source Code (.rs)
      ↓
   Tokenizer
      ↓
    AST
      ↓
    HIR (High-level IR)
      ↓
    MIR (Mid-level IR)
      ↓
LLVM IR
      ↓
Machine Code
```

### 11.2 MIR (Mid-level Intermediate Representation)

MIR은 제어 흐름 분석과 데이터 흐름 분석을 수행하는 중간 표현입니다.

### 11.3 모노모피제이션(Monomorphization)

```rust
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 컴파일 시 구체화:
fn max_i32(a: i32, b: i32) -> i32 { /* ... */ }
fn max_f64(a: f64, b: f64) -> f64 { /* ... */ }
```

### 11.4 인라인화(Inline)

```rust
#[inline]
fn always_inline() { /* ... */ }

#[inline(never)]
fn never_inline() { /* ... */ }

// 인라인 힌트:
// - inline: 항상 인라인 권장
// - inline(never): 인라인 금지
// - 없음: 컴파일러 결정에 위임
```

---

## 12. 패턴 매칭 시스템

### 12.1 패턴의 종류

```rust
// 리터럴
match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("anything"),
}

// 명명된 변수
match x {
    Some(y) => println!("{}", y),
    _ => println!("none"),
}

// 여러 패턴
match x {
    1 | 2 => println!("one or two"),
    _ => println!("anything"),
}

// 범위
match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}

// 구조체 분해
struct Point { x: i32, y: i32 }
match point {
    Point { x: 0, y } => println!("y is {}", y),
    Point { x, y: 0 } => println!("x is {}", x),
    Point { x, y } => println!("({}, {})", x, y),
}

// 열거형 분해
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(msg) => println!("{}", msg),
}
```

### 12.2 매치 가드(Match Guards)

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => println!("none"),
}
```

### 12.3 @ 바인딩

```rust
enum Message {
    Hello { id: i32 },
}

match msg {
    Message::Hello { id: id @ 3..=7 } => {
        println!("Found an id in range: {}", id)
    }
    Message::Hello { id: 10..=12 } => {
        println!("id는 여기서 사용 불가")
    }
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    }
}
```

---

## 13. 고급 기능

### 13.1 불안전한 Rust(Unsafe Rust)

```rust
unsafe fn dangerous() {
    // *mut 역참조
    // extern 함수 호출
    // unsafe 트레이트 구현
    // union 필드 접근
}

// 안전한 래퍼로 제공
unsafe trait SafeWrapper {
    // ...
}
```

### 13.2 고급 트레이트

```rust
// 연관 타입과 제네릭의 조합
trait Lender {
    type Lender;

    fn lend(&self) -> Self::Lender;
}

// 트레이트의 수명과 제네릭
trait Builder<'a> {
    type Output;

    fn build(&'a self) -> Self::Output;
}
```

### 13.3 매크로 시스템

```rust
// 선언적 매크로
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

say_hello!();

// 프로시저 매크로 (proc_macro)
use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // ...
}
```

### 13.4 FFI (Foreign Function Interface)

```rust
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn printf(format: *const c_char, ...);
}

fn main() {
    let s = CString::new("Hello, world!\n").unwrap();
    unsafe {
        printf(s.as_ptr());
    }
}
```

---

## 결론

Rust 프로그래밍 언어는 다음과 같은 특징을 가집니다:

1. **메모리 안전성**: 소유권, 대여, 수명 시스템을 통한 컴파일 타임 보장
2. **제로 비용 추상화**: 고수준 추상화도 저수준 코드만큼 빠름
3. **무공포 동시성**: Send/Sync 트레이트를 통한 데이터 레이스 방지
4. **현대적 패러다임**: 패턴 매칭, 트레이트, 제네릭 등

**적합한 사용 사례:**
- 시스템 프로그래밍 (OS, 임베디드)
- 웹 서버 및 CLI 도구
- 블록체인 및 크립토
- 고성능 애플리케이션

---

## 참고 자료

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [Rust Guidelines](https://rust-lang.github.io/rust-guidelines/)
