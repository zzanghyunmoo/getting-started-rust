# 프로그래밍 기초 (Rust) 강의 계획서

> **과목 정보**
> - 과목명: 프로그래밍 기초 (CS101)
> - 학점: 3학점
> - 대상: 컴퓨터 공학과 1학년
> - 언어: Rust 2024 Edition
> - 수업: 주 2회 (90분 × 2)
> - 실습: 주 1회 (120분)
> - **프로젝트: 텍스트 RPG 게임 "던전 크롤러"**

---

## 📋 목차

1. [강의 개요](#1-강의-개요)
2. [프로젝트 개요](#2-프로젝트-개요)
3. [주차별 강의 계획](#3-주차별-강의-계획)
4. [실습 및 과제](#4-실습-및-과제)
5. [평가 방식](#5-평가-방식)
6. [교재 및 참고자료](#6-교재-및-참고자료)

---

## 1. 강의 개요

### 1.1 과목 소개

본 과목은 컴퓨터 공학을 전공하는 학생들을 대상으로 프로그래밍의 기초 개념과 문제 해결 능력을 배양하는 것을 목표로 합니다. **Rust** 언어를 통해 **텍스트 RPG 게임 "던전 크롤러"** 를 점진적으로 개발하며 다음과 같은 내용을 학습합니다:

- 프로그래밍 언어의 기초 문법과 개념
- 알고리즘 설계와 구현
- 자료구조의 이해와 활용
- 메모리 관리와 시스템 프로그래밍 기초
- 좋은 코드 작성법과 소프트웨어 공학 입문

### 1.2 프로젝트 기반 학습

한 학기 동안 하나의 완성된 게임을 만들어가며 학습합니다. 각 주차의 과제는 게임의 새로운 기능을 추가하는 방식으로 진행됩니다.

**최종 결과물 예시:**
```
=====================================
           DUNGEON CRAWLER
=====================================

[용사의 이름을 입력하세요: ] 철수

=====================================
          DUNGEON CRAWLER
=====================================

용사: 철수 (Lv.1 직업: 모험가)
HP: [████████░░] 80/100  MP: [███░░░░░░] 30/100
공격력: 15  방어력: 5  골드: 50G

[인벤토리]
 1. 녹슬의 칼 (공격력+15)
 2. 가죽 갑옷 (방어력+5)
 3. 하얀 포션 x3 (회복+30)

[메뉴]
1. 던전 입장
2. 상점
3. 휴식
4. 인벤토리
5. 상태 보기
6. 게임 저장

> 1

던전으로 들어갑니다...

어둡침한 통로에서 고블린을 만났습니다!
[고블린] 체력: 30  공격력: 8

[행동 선택]
1. 공격하기
2. 스킬 사용
3. 아이템 사용
4. 도망치기

> 1

철수가 녹슬의 칼로 고블른을 공격했습니다!
크리티컬 히트! 25의 피해를 입혔습니다!

고블린이 철수에게 8의 피해를 입혔습니다!

[용사: 철수 | HP: 72/100 | MP: 35/100]

---

고블른을 물리쳤습니다!
[경험치 +25 | 골드 +12]
[레벨업! 경험치 +50 | HP/MP 전체 회복]

---

던전을 계속하시겠습니까? (y/n)
```

---

## 2. 프로젝트 개요

### 2.1 던전 크롤러 게임

**게임 개요:**
- 플레이어는 용사를 육성하며 던전을 탐험
- 몬스터와 전투하며 경험치와 아이템을 획득
- 상점에서 장비를 구매하고 인벤토리를 관리
- 레벨업을 통해 캐릭터 성장

**기능 요약:**
| 기능 | 설명 |
|------|------|
| 캐릭터 생성 | 이름, 직업 선택, 초기 스탯 |
| 전투 시스템 | 턴 기반 전투, 스킬, 아이템 사용 |
| 인벤토리 | 아이템 장착, 소모품 사용 |
| 상점 | 장비/소모품 구매 |
| 던전 탐험 | 랜덤 몬스터 조우, 보물 상자 |
| 레벨 시스템 | 경험치, 스탯 상승 |
| 저장/로드 | 게임 진행 저장 |

### 2.2 주차별 기능 추가 계획

| 주차 | 추가 기능 | 학습 개념 |
|------|-----------|-----------|
| 1주 | 게임 제목 화면 | Hello World, 출력 |
| 2주 | 플레이어 이름, 직업, 스탯 | 변수, 타입 |
| 3주 | 전투 데미지 계산 | 연산자, 표현식 |
| 4주 | HP에 따른 상태 메시지, 승리/패배 | 조건문, match |
| 5주 | 게임 루프, 여러 전투 | 반복문 |
| 7주 | 공격(), 치유(), 이동() 함수 | 함수 기초 |
| 8주 | 보물 상자 연속 열기 | 재귀 |
| 9주 | Player, Monster, Item 구조체 | 구조체 |
| 10주 | ItemType, GameState, Direction | 열거형, Option |
| 11주 | 인벤토리(Vec), 아이템 설명(String) | Vec, String |
| 12주 | 몬스터 도감(HashMap), 에러 처리 | HashMap, Result |
| 13주 | 인벤토리 정렬 | 정렬 알고리즘 |
| 14주 | 도감에서 검색 | 탐색 알고리즘 |

### 2.3 최종 완성 기능

**기말 프로젝트 필수 기능:**
- [x] 캐릭터 생성 및 커스터마이징
- [ ] 전투 시스템 (공격, 스킬, 아이템)
- [ ] 인벤토리 시스템
- [ ] 상점 시스템
- [ ] 던전 랜덤 생성
- [ ] 레벨업 시스템
- [ ] 게임 저장/로드

---

## 3. 주차별 강의 계획

---

### 📅 제1주: 프로그래밍과 컴퓨팅 개론

**주제:** 첫 번째 프로그램 - 게임 제목 화면

**학습 목표:**
- 프로그래밍과 컴퓨터 과학의 기본 개념 이해
- Rust 개발 환경 설정
- `println!` 매크로로 텍스트 출력

**이론 내용:**
- 컴퓨터 과학이란?
- 프로그래밍 언어의 역사와 분류
- Rust 언어 소개
- 컴파일러와 인터프리터

**실습 내용:**
1. Rust 설치 및 개발 환경 설정 (rustup, cargo, VS Code)
2. 첫 번째 프로그램: Hello, World!
3. cargo 프로젝트 생성 및 빌드
4. `println!`으로 게임 제목 출력

**실습 코드:**
```rust
fn main() {
    println!("====================================");
    println!("         DUNGEON CRAWLER");
    println!("====================================");
    println!();
    println!("         v1.0 - Coming Soon");
    println!("====================================");
}
```

**과제 #1: 게임 시작 화면**
```rust
// 요구사항:
// 1. 게임 제목 "DUNGEON CRAWLER" 출력
// 2. 장식선 (=)로 테두리 출력
// 3. 개발자 정보 출력 (본인 이름, 학번)
// 4. "Press Enter to start..." 메시지

// 실행 예시:
// ====================================
//          DUNGEON CRAWLER
// ====================================
//
// Developed by: 홍길동 (202412345)
//                컴퓨터 공학과
//
// Press Enter to start...
```

---

### 📅 제2주: 변수와 기본 타입

**주제:** 플레이어 정보 저장하기

**학습 목표:**
- 변수와 상수의 개념 이해
- Rust의 기본 타입 이해
- 타입 추론 이해

**이론 내용:**
- 변수 선언과 초기화
- 불변성(Immutability)과 `mut`
- 기본 타입: 정수, 부동소수점, 불리언, 문자
- 섀도잉(Shadowing)
- 상수(const)

**실습 내용:**
1. 플레이어 이름 저장
2. 직업 선택 (전사, 마법사, 도적)
3. 능력치 저장

**실습 코드:**
```rust
fn main() {
    // 플레이어 이름
    let player_name = "철수";

    // 직업
    let player_class = "전사";

    // 능력치
    let player_hp = 100;
    let player_mp = 50;
    let player_attack = 15;
    let player_defense = 5;

    println!("용사: {}", player_name);
    println!("직업: {}", player_class);
    println!("HP: {}", player_hp);
    println!("MP: {}", player_mp);
}
```

**과제 #2: 캐릭터 생성 화면**
```rust
// 요구사항:
// 1. 플레이어 이름 입력 받기 (아직 안 배움 -> 변수에 직접 저장)
// 2. 직업 선택 변수
// 3. 초기 능력치 변수
//    - HP: 100 (전사), 60 (마법사), 80 (도적)
//    - MP: 50 (전사), 100 (마법사), 60 (도적)
//    - 공격력: 15, 10, 12
//    - 방어력: 5, 3, 8
// 4. 캐릭터 정보 출력

// 실행 예시:
// 용사의 이름을 입력하세요: 철수
// 직업을 선택하세요 (1.전사 2.마법사 3.도적): 1
//
// ====================================
// 캐릭터가 생성되었습니다!
// ====================================
// 이름: 철수
// 직업: 전사
// HP: 100 / MP: 50
// 공격력: 15 / 방어력: 5
// ====================================
```

---

### 📅 제3주: 연산자와 표현식

**주제:** 전투 시스템 구현 (기초)

**학습 목표:**
- 다양한 연산자의 사용법
- 전투 데미지 계산 구현
- 표현식과 문장의 차이

**이론 내용:**
- 산술 연산자: `+`, `-`, `*`, `/`, `%`
- 비교 연산자: `==`, `!=`, `<`, `>`, `<=`, `>=`
- 논리 연산자: `&&`, `||`, `!`
- 복합 대입 연산자: `+=`, `-=`, `*=`

**실습 내용:**
1. 기본 공격 데미지 계산
2. 크리티컬 히트 시스템
3. 체력 회복량 계산

**실습 코드:**
```rust
fn main() {
    // 공격자와 방어자 스탯
    let player_attack = 15;
    let monster_defense = 3;

    // 기본 데미지 계산
    let damage = player_attack - monster_defense;

    println!("데미지: {}", damage);

    // 크리티컬 (랜덤 20% 확률 - 아직 안 배움 -> 고정값)
    let is_critical = true;
    let final_damage = if is_critical {
        damage * 2
    } else {
        damage
    };

    println!("최종 데미지: {}", final_damage);
}
```

**과제 #3: 전투 데미지 계산 시스템**
```rust
// 요구사항:
// 1. 플레이어 공격력: 15, 몬스터 방어력: 5
// 2. 기본 데미지 = 공격력 - 방어력 (최소 1 보장)
// 3. 크리티컬 히트 여부 (is_critical = true로 고정)
// 4. 크리티컬 시 데미지 x2
// 5. 경험치 계산 (데미지 x 5)
// 6. 골드 획득 (랜덤 5~15 -> 고정값 10)

// 실행 예시:
// ====================================
// 전투 결과
// ====================================
// 기본 데미지: 10
// 크리티컬 히트!
// 최종 데미지: 20
// 획득 경험치: 100
// 획득 골드: 10
// ====================================
```

---

### 📅 제4주: 제어 흐름 - 조건문

**주제:** 상태에 따른 메시지 출력

**학습 목표:**
- if/else/else if 구조 이해
- match 표현식 이해
- HP에 따른 상태 메시지

**이론 내용:**
- if 표현식
- match 표현식과 패턴 매칭
- 매치 가드(match guards)

**실습 내용:**
1. HP에 따른 상태 메시지
2. 승리/패배 조건
3. 레벨업 조건

**실습 코드:**
```rust
fn main() {
    let player_hp = 80;
    let player_max_hp = 100;

    // HP 비율 계산
    let hp_percent = (player_hp as f64 / player_max_hp as f64) * 100.0;

    // 상태 메시지
    if hp_percent == 100.0 {
        println!("상태: 건강함!");
    } else if hp_percent >= 70.0 {
        println!("상태: 양호함");
    } else if hp_percent >= 30.0 {
        println!("상태: 부상");
    } else {
        println!("상태: 위험함!");
    }

    // match로 레벨업 메시지
    let level = 1;
    let exp = 150;

    match exp {
        exp if exp >= 100 => println!("레벨업! Lv.{} -> Lv.{}", level, level + 1),
        _ => println!("현재 경험치: {}/100", exp),
    }
}
```

**과제 #4: 전투 결과 시스템**
```rust
// 요구사항:
// 1. 전투 전후 HP 출력
// 2. HP에 따른 상태 메시지 (건강, 양호, 부상, 위험, 사망)
// 3. 승리/패배 조건 (HP > 0 = 승리, HP <= 0 = 패배)
// 4. 레벨업 시스템 (exp >= 100)

// 실행 예시 (승리):
// ====================================
// 전투 시작!
// ====================================
// [용사] HP: 100 -> 72
// [고블린] HP: 30 -> 0
//
// 승리했습니다!
// 상태: 양호
// 경험치 +80
//
// ====================================
//
// 실행 예시 (패배):
// ====================================
// 전투 시작!
// ====================================
// [용사] HP: 30 -> 0
// [고블린] HP: 50 -> 20
//
// 패배했습니다...
// 상태: 사망
// 마을로 돌아갑니다.
// ====================================
```

---

### 📅 제5주: 제어 흐름 - 반복문

**주제:** 게임 루프와 연속 전투

**학습 목표:**
- loop, while, for 반복문 이해
- 게임 루프 구현
- 여러 몬스터와 연속 전투

**이론 내용:**
- loop 무한 반복
- while 조건부 반복
- for 범위 반복
- break와 continue

**실습 내용:**
1. 게임 루프 구조
2. 메인 메뉴
3. 3번의 연속 전투

**실습 코드:**
```rust
fn main() {
    let mut hp = 100;

    // 게임 루프
    loop {
        println!("현재 HP: {}", hp);

        if hp <= 0 {
            println!("게임 오버!");
            break;
        }

        // 전투 처리 (간단)
        hp -= 10;
        println!("전투 후 HP: {}", hp);
    }
}
```

**과제 #5: 게임 루프와 메인 메뉴**
```rust
// 요구사항:
// 1. 게임 시작 화면 후 메인 메뉴 출력
// 2. 메뉴: 1.던전 입장 2.상점 3.휴식 4.종료
// 3. "던전 입장" 선택 시 3번의 연속 전투
// 4. HP가 0 이하가 되면 마을로 돌아감
// 5. 각 전투 후 HP, 경험치, 골드 변화 출력

// 실행 예시:
// ====================================
//          DUNGEON CRAWLER
// ====================================
// 용사: 철수 | HP: 100/100 | 골드: 50G
//
// [메뉴]
// 1. 던전 입장
// 2. 상점
// 3. 휴식
// 4. 종료
// 선택: 1
//
// ---
// 1번째 전투: 고블른 (승리!) HP: 100 -> 85
// 2번째 전투: 오크 (승리!) HP: 85 -> 60
// 3번째 전투: 늑대 (승리!) HP: 60 -> 30
//
// 던전에서 살아남아 돌아왔습니다!
// 현재 HP: 30/100
// ====================================
```

---

### 📅 제6주: 중간고사

**범위:** 제1주 ~ 제5주

**형식:**
- 이론: 30분 (객관식 20문제)
- 코딩: 60분 (2문제)

**예상 문제:**
1. 변수, 타입, 연산자 기초
2. if/match를 활용한 조건문 작성
3. for/while을 활용한 반복문 작성
4. 간단한 게임 로직 구현

---

### 📅 제7주: 함수 기초

**주제:** 전투 시스템 모듈화

**학습 목표:**
- 함수 정의와 호출
- 매개변수와 반환값
- 전투 관련 함수 분리

**이론 내용:**
- 함수 정의 문법
- 매개변수와 인수
- 반환값과 반환 타입
- 조기 반환(early return)

**실습 내용:**
1. `attack()` 함수
2. `take_damage()` 함수
3. `heal()` 함수
4. `check_status()` 함수

**실습 코드:**
```rust
fn calculate_damage(attack: i32, defense: i32) -> i32 {
    let damage = attack - defense;
    if damage < 1 {
        1
    } else {
        damage
    }
}

fn attack(attack_power: i32, target_defense: i32) -> i32 {
    calculate_damage(attack_power, target_defense)
}

fn take_damage(current_hp: i32, damage: i32) -> i32 {
    current_hp - damage
}

fn is_alive(hp: i32) -> bool {
    hp > 0
}

fn main() {
    let player_hp = 100;
    let player_attack = 15;
    let monster_defense = 5;

    let damage = attack(player_attack, monster_defense);
    let new_hp = take_damage(player_hp, damage);

    println!("데미지: {}", damage);
    println!("남은 HP: {}", new_hp);
    println!("생존: {}", is_alive(new_hp));
}
```

**과제 #6: 전투 시스템 함수화**
```rust
// 요구사항:
// 1. calculate_damage(attack, defense) -> i32
// 2. attack(player_attack, monster_defense, is_critical) -> i32
// 3. take_damage(current_hp, damage) -> i32
// 4. is_alive(hp) -> bool
// 5. check_status(hp) -> &str (상태 메시지 반환)
// 6. print_combat_result(player_hp, monster_hp, damage)
// 7. gain_exp(current_exp, amount) -> i32 (레벨업 체크 포함)

// 실행 예시:
// calculate_damage(15, 5) = 10
// attack(15, 5, true) = 20 (크리티컬)
// take_damage(100, 20) = 80
// is_alive(80) = true
// check_status(80) = "양호"
// print_combat_result()로 전투 결과 출력
// gain_exp(150, 80) = 230 (레벨업!)
```

---

### 📅 제8주: 함수 심화와 재귀

**주제:** 보물 상자와 랜덤 보상

**학습 목표:**
- 재귀 함수 이해
- 연속 보물 상자 열기
- 중첩된 보물 시스템

**이론 내용:**
- 재귀의 개념
- 기저 조건(base case)
- 재귀 조건(recursive case)

**실습 내용:**
1. 단일 보물 상자
2. 중첩된 보물 상자
3. 확률 보상 시스템

**실습 코드:**
```rust
// 보물 상자
struct TreasureBox {
    gold: u32,
    contains_box: bool,
}

fn open_box(box_: TreasureBox) -> u32 {
    let mut total_gold = box_.gold;

    if box_.contains_box {
        println!("상자 안에 또 다른 상자가 있습니다!");
        let inner_box = TreasureBox {
            gold: 50,
            contains_box: false,
        };
        total_gold += open_box(inner_box);
    }

    total_gold
}

fn main() {
    let box1 = TreasureBox {
        gold: 100,
        contains_box: true,
    };

    let box2 = TreasureBox {
        gold: 100,
        contains_box: false,
    };

    println!("상자 1: {}G", open_box(box1));  // 150G
    println!("상자 2: {}G", open_box(box2));  // 100G
}
```

**과제 #7: 연속 보물 상자 시스템**
```rust
// 요구사항:
// 1. TreasureBox 구조체 (gold, contains_box, item)
// 2. open_box(box) -> u32 재귀 함수 (총 골드 계산)
// 3. 보물 상자 랜덤 생성 함수
//    - 확률로 아이템 포함 (무기, 방어구, 포션)
//    - 10% 확률로 또 다른 상자 포함 (최대 3중첩)
// 4. 전리 획득 골드 = 상자 골드 + 전리 가치

// 실행 예시:
// 보물 상자를 발견했습니다!
// 상자를 엽니다...
//
// [상자 내용물]
// - 50G
// - 녹슬의 칼 (공격력+15)
// - 안에 또 다른 상자가 있습니다!
//
// 안쪽 상자를 엽니다...
// - 30G
// - 하얀 포션 (회복+50)
//
// 총 획득: 80G, 녹슬의 칼, 하얀 포션
```

---

### 📅 제9주: 복합 타입 - 구조체

**주제:** Player, Monster, Item 구조체

**학습 목표:**
- 구조체 정의와 사용
- 메서드 구현
- 게임 객체 모델링

**이론 내용:**
- 구조체 정의
- 필드와 인스턴스화
- 메서드 문법 (`&self`, `&mut self`)
- 연관 함수

**실습 내용:**
1. Player 구조체와 메서드
2. Monster 구조체
3. Item 구조체

**실습 코드:**
```rust
struct Player {
    name: String,
    class: String,
    hp: i32,
    max_hp: i32,
    mp: i32,
    max_mp: i32,
    attack: i32,
    defense: i32,
    level: u32,
    exp: u32,
    gold: u32,
}

impl Player {
    fn new(name: String, class: String) -> Self {
        let (hp, mp, attack, defense) = match class.as_str() {
            "전사" => (100, 50, 15, 5),
            "마법사" => (60, 100, 10, 3),
            "도적" => (80, 60, 12, 8),
            _ => (80, 50, 10, 5),
        };

        Player {
            name,
            class,
            max_hp: hp,
            hp,
            max_mp: mp,
            mp,
            attack,
            defense,
            level: 1,
            exp: 0,
            gold: 100,
        }
    }

    fn attack(&self, defense: i32) -> i32 {
        let damage = self.attack - defense;
        if damage < 1 { 1 } else { damage }
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
    }

    fn heal(&mut self, amount: i32) {
        self.hp += amount;
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn gain_exp(&mut self, amount: u32) -> bool {
        self.exp += amount;
        if self.exp >= 100 {
            self.level_up();
            return true;
        }
        false
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.exp -= 100;
        self.max_hp += 10;
        self.hp = self.max_hp;
        self.max_mp += 10;
        self.mp = self.max_mp;
        self.attack += 3;
        self.defense += 2;
        println!("{} 레벨업! Lv.{}", self.name, self.level);
    }

    fn print_status(&self) {
        println!("====================================");
        println!("용사: {} (Lv.{})", self.name, self.level);
        println!("직업: {}", self.class);
        println!("HP: {}/{} | MP: {}/{}", self.hp, self.max_hp, self.mp, self.max_mp);
        println!("공격력: {} | 방어력: {}", self.attack, self.defense);
        println!("경험치: {}/100 | 골드: {}G", self.exp, self.gold);
        println!("====================================");
    }
}

struct Monster {
    name: String,
    hp: i32,
    max_hp: i32,
    attack: i32,
    defense: i32,
    exp_reward: u32,
    gold_reward: u32,
}

impl Monster {
    fn goblin() -> Self {
        Monster {
            name: String::from("고블린"),
            hp: 30,
            max_hp: 30,
            attack: 8,
            defense: 3,
            exp_reward: 25,
            gold_reward: 10,
        }
    }

    fn orc() -> Self {
        Monster {
            name: String::from("오크"),
            hp: 50,
            max_hp: 50,
            attack: 12,
            defense: 5,
            exp_reward: 40,
            gold_reward: 15,
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

fn main() {
    let mut player = Player::new(
        String::from("철수"),
        String::from("전사")
    );

    player.print_status();

    let mut monster = Monster::goblin();

    let damage = player.attack(monster.defense);
    monster.take_damage(damage);

    println!("{}이 {}에게 {}의 피해를 입혔습니다!",
        player.name, monster.name, damage);
}
```

**과제 #8: Player, Monster 구조체 적용**
```rust
// 요구사항:
// 1. 위 Player 구조체를 그대로 사용
// 2. Monster 구조체에 다음 몬스터 추가
//    - 슬라임 (HP: 20, 공격: 5, 방어: 1)
//    - 늑대 (HP: 80, 공격: 15, 방어: 8)
//    - 드래곤 (HP: 200, 공격: 30, 방어: 15)
// 3. Item 구조체 추가
//    - struct Item { name, item_type, attack_bonus, defense_bonus, heal_amount }
// 4. 전투 시스템에 구조체 적용
//    - player.attack(monster.defense)
//    - monster.attack(player.defense)
//    - player.take_damage(damage)

// 실행 예시:
// ====================================
// 용사: 철수 (Lv.1)
// HP: 100/100 | MP: 50/50
// 공격력: 15 | 방어력: 5
// ====================================
//
// 고블린을 만났습니다!
// [고블린] HP: 30 | 공격력: 8 | 방어력: 3
//
// 철수가 고블린에게 12의 피해를 입혔습니다!
// 고블린이 철수에게 3의 피해를 입혔습니다!
```

---

### 📅 제10주: 복합 타입 - 열거형과 Option

**주제:** ItemType, Direction, 선택적 값

**학습 목표:**
- 열거형 정의와 사용
- Option<T> 타입 이해
- 패턴 매칭 심화

**이론 내용:**
- 열거형 정의
- 데이터가 있는 열거형
- Option<T>과 널 안전성
- match와 if let

**실습 내용:**
1. ItemType 열거형
2. Direction 열거형
3. Option을 활용한 검색

**실습 코드:**
```rust
enum ItemType {
    Weapon,
    Armor,
    Potion,
}

struct Item {
    name: String,
    item_type: ItemType,
    attack_bonus: i32,
    defense_bonus: i32,
    heal_amount: i32,
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn get_item_description(item: &Item) -> &str {
    match item.item_type {
        ItemType::Weapon => &format!("무기 (공격력+{})", item.attack_bonus),
        ItemType::Armor => &format!("방어구 (방어력+{})", item.defense_bonus),
        ItemType::Potion => &format!("포션 (회복+{})", item.heal_amount),
    }
}

fn find_item(inventory: &[Item], name: &str) -> Option<usize> {
    for (i, item) in inventory.iter().enumerate() {
        if item.name == name {
            return Some(i);
        }
    }
    None
}

fn main() {
    let sword = Item {
        name: String::from("녹슬의 칼"),
        item_type: ItemType::Weapon,
        attack_bonus: 15,
        defense_bonus: 0,
        heal_amount: 0,
    };

    println!("{}", get_item_description(&sword));

    let inventory = vec
![sword];

    match find_item(&inventory, "녹슬의 칼") {
        Some(index) => println!("인덱스: {}", index),
        None => println!("찾을 수 없습니다"),
    }
}
```

**과제 #9: 열enumeration과 Option 적용**
```rust
// 요구사항:
// 1. ItemType 열�형 (Weapon, Armor, Potion, Special)
// 2. Item 구조체에 item_type 추가
// 3. GameState 열갤형
//    - MainMenu, Exploring, Combat, Shop, Resting, GameOver
// 4. Option을 활용한 검색
//    - find_item(inventory, name) -> Option<usize>
//    - find_monster( monsters, name) -> Option<usize>
// 5. get_item_bonus(item) -> i32 (아이템 효과 반환)

// 실행 예시:
// 인벤토리에서 아이템을 찾습니다...
// 이름: 녹슬의 칼
// [무기] 공격력: +15
//
// 몬스터 도감에서 검색합니다...
// 이름: 드래곤
// [드래곤] HP: 200 | 공격력: 30 | 방어력: 15
// 경험치: 500 | 골드: 100
```

---

### 📅 제11주: 컬렉션 - Vec과 String

**주제:** 인벤토리 시스템

**학습 목표:**
- Vec<T> 사용법
- String 조작
- 슬라이스 기초

**이론 내용:**
- Vec 생성과 조작
- 반복자 기초
- String vs &str

**실습 내용:**
1. 인벤토리 구현
2. 아이템 추가/제거
3. 아이템 설명 출력

**실습 코드:**
```rust
struct Item {
    name: String,
    description: String,
}

fn main() {
    // 인벤토리 생성
    let mut inventory: Vec<Item> = Vec::new();

    // 아이템 추가
    inventory.push(Item {
        name: String::from("녹슬의 칼"),
        description: String::from("고대의 영움이 사용했던 검입니다. 공격력 +15"),
    });

    inventory.push(Item {
        name: String::from("가죽 갑옷"),
        description: String::from("튼튼한 가죽으로 만든 갑옷입니다. 방어력 +5"),
    });

    // 인벤토리 출력
    println!("=== 인벤토리 ({} / 20) ===", inventory.len());

    for (index, item) in inventory.iter().enumerate() {
        println!("{}. {}", index + 1, item.name);
        println!("   {}", item.description);
    }
}
```

**과제 #10: 인벤토리 시스템 완성**
```rust
// 요구사항:
// 1. Vec<Item>으로 인벤토리 구현 (최대 20개)
// 2. 아이템 추가/제거 기능
//    fn add_item(inventory, item)
//    fn remove_item(inventory, index) -> Item
//    fn use_item(inventory, index, player) -> (bool, String)
// 3. 아이템 사용
//    - 무기: 장착 시 공격력 증가
//    - 방어구: 장착 시 방어력 증가
//    - 포션: HP/MP 회복
// 4. 장착 시스템
//    - weapon: Option<Item>
//    - armor: Option<Item>

// 실행 예시:
// === 인벤토리 (2 / 20) ===
// 1. 녹슬의 칼 [장착중]
// 2. 가죽 갑옷
// 3. 하얀 포션 x5
//
// 사용할 아이템 번호: 3
// 하얀 포션을 사용했습니다!
// HP: 80 -> 100
// 남은 포션: 4
```

---

### 📅 제12주: 해시맵과 에러 처리

**주제:** 몬스터 도감과 상점

**학습 목표:**
- HashMap<K, V> 사용법
- Result<T, E> 활용
- 에러 전파

**이론 내용:**
- HashMap 생성과 조작
- Result 타입
- ? 연산자

**실습 내용:**
1. 몬스터 도감 구현
2. 상점 아이템 목록
3. 구매 시스템

**실습 코드:**
```rust
use std::collections::HashMap;

struct MonsterInfo {
    hp: i32,
    attack: i32,
    defense: i32,
    exp_reward: u32,
    gold_reward: u32,
    description: String,
}

fn create_monster_bestiary() -> HashMap<&'static str, MonsterInfo> {
    let mut bestiary = HashMap::new();

    bestiary.insert("고블린", MonsterInfo {
        hp: 30,
        attack: 8,
        defense: 3,
        exp_reward: 25,
        gold_reward: 10,
        description: String::from("작고약한 몬스터입니다."),
    });

    bestiary.insert("오크", MonsterInfo {
        hp: 50,
        attack: 12,
        defense: 5,
        exp_reward: 40,
        gold_reward: 15,
        description: String::from("공격적인 성향의 오크입니다."),
    });

    bestiary.insert("슬라임", MonsterInfo {
        hp: 20,
        attack: 5,
        defense: 1,
        exp_reward: 15,
        gold_reward: 5,
        description: String::from("아주 약하지만 수가 많습니다."),
    });

    bestiary.insert("늑대", MonsterInfo {
        hp: 80,
        attack: 15,
        defense: 8,
        exp_reward: 60,
        gold_reward: 25,
        description: String::from("민첩하고 강력한 늑대입니다."),
    });

    bestiary.insert("드래곤", MonsterInfo {
        hp: 200,
        attack: 30,
        defense: 15,
        exp_reward: 500,
        gold_reward: 100,
        description: String::from("전설 속의 강력한 용입니다."),
    });

    bestiary
}

fn search_monster(bestiary: &HashMap<&str, MonsterInfo>, name: &str) -> Option<&MonsterInfo> {
    bestiary.get(name)
}

fn main() {
    let bestiary = create_monster_bestiary();

    // 전체 도감 출력
    println!("=== 몬스터 도감 ===");
    for (name, info) in &bestiary {
        println!("{}: {}", name, info.description);
    }

    // 검색
    match search_monster(&bestiary, "드래곤") {
        Some(info) => {
            println!("=== {} ===", "드래곤");
            println!("HP: {}", info.hp);
            println!("공격력: {}", info.attack);
            println!("방어력: {}", info.defense);
            println!("보상: 경험치 {}, 골드 {}G", info.exp_reward, info.gold_reward);
        }
        None => {
            println!("몬스터를 찾을 수 없습니다.");
        }
    }
}
```

**과제 #11: 도감과 상점 시스템**
```rust
// 요구사항:
// 1. create_monster_bestiary() -> HashMap<&str, MonsterInfo>
//    - 5종 몬스터 정보
// 2. create_shop_items() -> HashMap<&str, (u32, &str)>
//    - 아이템별 가격과 설명
// 3. search_monster(bestiary, name) -> Option<&MonsterInfo>
// 4. buy_item(gold, price) -> Result<bool, String>
//    - 골드 부족 시 Err 반환
// 5. 상점 메뉴 구현

// 실행 예시:
// === 상점 ===
// 1. 목검 (500G) - 전사의 기본 무기
// 2. 롱소드 (1000G) - 강력한 대검
// 3. 플레이트 아머 (800G) - 가벼운 갑옷
// 4. 하얀 포션 (50G) - HP 30 회복
// 5. 마나 포션 (50G) - MP 30 회복
//
// 구매할 아이템 번호: 1
// 롱소드를 구매했습니다!
// -1000G 지불
// 보유 중인 장검이 저장되었습니다.
// 남은 골드: 0G
```

---

### 📅 제13주: 알고리즘 기초 (1) - 정렬

**주제:** 인벤토리 정렬

**학습 목표:**
- 기본 정렬 알고리즘 이해
- 버블 정렬 구현
- 인벤토리 정렬 적용

**이론 내용:**
- 정렬의 필요성
- 버블 정렬 (Bubble Sort)
- 시간 복잡도 기초

**실습 내용:**
1. 아이템 이름순 정렬
2. 아이템 가격순 정렬
3. 아이템 공격력순 정렬

**실습 코드:**
```rust
#[derive(Debug, Clone)]
struct Item {
    name: String,
    price: u32,
    power: i32,
}

fn sort_by_name(items: &mut Vec<Item>) {
    let n = items.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if items[j].name > items[j + 1].name {
                items.swap(j, j + 1);
            }
        }
    }
}

fn sort_by_price(items: &mut Vec<Item>) {
    let n = items.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if items[j].price > items[j + 1].price {
                items.swap(j, j + 1);
            }
        }
    }
}

fn sort_by_power(items: &mut Vec<Item>) {
    let n = items.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if items[j].power < items[j + 1].power {
                items.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut items = vec
![
        Item {
            name: String::from("포션"),
            price: 50,
            power: 0,
        },
        Item {
            name: String::from("검"),
            price: 500,
            power: 15,
        },
        Item {
            name: String::from("갑옷"),
            price: 300,
            power: 0,
        },
    ];

    println!("이름순:");
    sort_by_name(&mut items);
    for item in &items {
        println!("  {}", item.name);
    }
}
```

**과제 #12: 인벤토리 정렬 시스템**
```rust
// 요구사항:
// 1. sort_inventory(inventory, sort_by) 함수
//    - sort_by: "name", "price", "attack", "defense"
// 2. 각 정렬 기준에 따라 다른 순서로 정렬
// 3. 정렬 후 인벤토리 출력
// 4. 메뉴에서 정렬 옵션 선택 가능

// 실행 예시:
// === 인벤토리 정렬 ===
// 1. 이름순
// 2. 가격순
// 3. 공격력순
// 4. 방어력순
// 선택: 2
//
// === 인벤토리 (가격순) ===
// 1. 롱소드 (1000G) 공격력+25
// 2. 목검 (500G) 공격력+15
// 3. 플레이트 아머 (800G) 방어력+8
// 4. 하얀 포션 (50G) 회복+30
// ...
```

---

### 📅 제14주: 알고리즘 기초 (2) - 탐색

**주제:** 도감 검색 시스템

**학습 목표:**
- 선형 탐색 구현
- 이진 탐색 구현
- 도감 검색 적용

**이론 내용:**
- 선형 탐색 (Linear Search)
- 이진 탐색 (Binary Search)
- 탐색 알고리즘 비교

**실습 내용:**
1. 이름으로 몬스터 검색
2. 경험치 범위로 검색
3. 이진 탐색으로 최적화

**실습 코드:**
```rust
use std::collections::HashMap;

struct MonsterInfo {
    name: String,
    hp: i32,
    attack: i32,
    exp_reward: u32,
}

fn linear_search_monsters(
    monsters: &[(&str, MonsterInfo)],
    name: &str
) -> Option<&MonsterInfo> {
    for (_, info) in monsters {
        if info.name == name {
            return Some(info);
        }
    }
    None
}

fn binary_search_monsters_by_exp(
    monsters: &mut [(String, MonsterInfo)],
    target_exp: u32
) -> Option<&MonsterInfo> {
    // 먼저 exp 기준 정렬 필요
    monsters.sort_by_key(|(_, info)| info.exp_reward);

    let mut left = 0;
    let mut right = monsters.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if monsters[mid].1.exp_reward >= target_exp {
            left = mid;
        } else {
            right = mid;
        }

        if right == left + 1 && monsters[left].1.exp_reward < target_exp {
            return None;
        }

        if monsters[left].1.exp_reward == target_exp {
            return Some(&monsters[left].1);
        }
    }

    // 정확히 일치하는 것 찾기
    monsters.get(left)
        .filter(|(_, info)| info.exp_reward == target_exp)
        .map(|(_, info)| info)
}

fn main() {
    // ... (도감 생성 코드)

    match linear_search_monsters(&monsters_vec, "드래곤") {
        Some(info) => println!("{}을 찾았습니다!", info.name),
        None => println!("몬스터를 찾을 수 없습니다."),
    }
}
```

**과제 #13: 도감 검색 시스템**
```rust
// 요구사항:
// 1. search_by_name(bestiary, name) -> Option<&MonsterInfo>
// 2. search_by_exp_range(bestiary, min_exp, max_exp) -> Vec<&MonsterInfo>
// 3. search_by_gold_range(bestiary, min_gold, max_gold) -> Vec<&MonsterInfo>
// 4. binary_search_by_exp(bestiary, target_exp) -> Option<&MonsterInfo>
//    - HashMap을 Vec로 변환 후 정렬하여 이진 탐색
// 5. 도감 메뉴 구현

// 실행 예시:
// === 몬스터 도감 검색 ===
// 1. 이름으로 검색
// 2. 경험치 범위로 검색
// 3. 골드 범위로 검색
// 4. 뒤로가기
//
// 선택: 2
// 최소 경험치: 20
// 최대 경험치: 50
//
// 검색 결과 (3마리):
// - 고블린 (경험치: 25)
// - 오크 (경험치: 40)
// - 슬라임 (경험치: 15) // 제외됨
```

---

### 📅 제15주: 기말 프로젝트 완성 및 발표

**프로젝트 완성: 던전 크롤러**

**필수 기능:**
- [x] 캐릭터 생성 시스템
- [x] 전투 시스템 (공격, 크리티컬, 스킬)
- [ ] 인벤토리 시스템 (추가, 제거, 사용)
- [ ] 상점 시스템 (구매, 판매)
- [ ] 던전 랜덤 생성
- [ ] 레벨업 시스템
- [ ] 게임 저장/로드

**추가 기능 (선택):**
- [ ] 스킬 시스템 (강타공, 치유, 파이어볼 등)
- [ ] 여러 던전 (숲, 폐허, 동굴)
- [ ] 보스 몬스터
- [ ] 엔딩 시스템
- [ ] 업적 (도적, 성직사, 마법사)

**평가 기준:**
- 완성도 (40%)
- 코드 품질 (30%)
- 창의성 (15%)
- 발표 및 데모 (15%)

---

### 📅 제16주: 기말고사

**범위:** 제7주 ~ 제14주

**형식:**
- 이론: 30분 (객관식 20문제)
- 코딩: 90분 (3문제 - 함수, 구조체, 컬렉션)

---

## 4. 실습 및 과제

### 4.1 실습 운영 방침

**실습 시간:** 매주 수요일 14:00-16:00 (120분)

**실습 구성:**
- 이론 복습 (20분)
- 주차 과제 구현 (80분)
- TA 피드백 및 Q&A (20분)

**실습 환경:**
- OS: Linux (WSL2 또는 Ubuntu)
- Editor: VS Code + rust-analyzer
- 버전 관리: Git + GitHub

### 4.2 과제 정책

**과제 제출:** 매주 일요일 23:59까지 (GitHub에 commit)

**과제 형식:**
```markdown
# 던전 크롤러 - Week N 과제

## 실행 방법
\`\`\`bash
cargo run --release
\`\`\`

## 구현 내용
- 주차 학습 내용
- 추가된 기능
- 코드 구조

## 실행 예시
\`\`\`
게임 실행 화면...
\`\`\`

## 참고 자료
- Rust Book 해당 장
- 기타 참고자료
```

**지연 제출:**
- 1일 지연: 10% 감점
- 7일 이후: 0점

### 4.3 과제 연결성

모든 과제는 하나의 게임으로 연결됩니다:

```
Week 1: 제목 화면
    ↓
Week 2: 캐릭터 생성
    ↓
Week 3: 전투 데미지 계산
    ↓
Week 4: 승리/패배 조건
    ↓
Week 5: 게임 루프
    ↓
Week 7: 함수화
    ↓
Week 8: 보물 상자
    ↓
Week 9: 구조체 (Player, Monster, Item)
    ↓
Week 10: 열거형 (ItemType, GameState)
    ↓
Week 11: 인벤토리 (Vec)
    ↓
Week 12: 도감 & 상점 (HashMap)
    ↓
Week 13: 인벤토리 정렬
    ↓
Week 14: 도감 검색
    ↓
Week 15: 최종 완성
```

### 4.4 기말 프로젝트

**개발 기간:** 제1주 ~ 제15주 (전 학기)

**저장소:** GitHub 비공개 저장소

**커밋 규칙:**
- 각 주차 과제 완료 시 commit
- commit 메시지: `feat: Week N - 기능 설명`
- 코드 리뷰: 조교 또는 동료 학생

---

## 5. 평가 방식

### 5.1 성적 비율

| 항목 | 비율 | 비고 |
|------|------|------|
| 출석 | 10% | 결석 3회 이상 시 F |
| 주차 과제 | 30% | 매주 과제 (14주 × 2점) |
| 중간고사 | 20% | 제6주 |
| 기말 프로젝트 | 20% | 제15주 발표 |
| 기말고사 | 20% | 제16주 |

### 5.2 성적 등급

| 점수 | 등급 | 평점 |
|------|------|------|
| 95-100 | A+ | 4.5 |
| 90-94 | A | 4.3 |
| 85-89 | B+ | 4.0 |
| 80-84 | B | 3.7 |
| 75-79 | C+ | 3.3 |
| 70-74 | C | 3.0 |
| 65-69 | D+ | 2.7 |
| 60-64 | D | 2.3 |
| 0-59 | F | 0.0 |

### 5.3 과제 평가 기준

주차 과제는 다음 기준으로 평가합니다:

| 평가 항목 | 비율 | 설명 |
|-----------|------|------|
| 요구사항 충족 | 40% | 모든 필수 기능 구현 |
| 실행 가능성 | 20% | 에러 없이 실행 |
| 코드 품질 | 20% | 가독성, 가독성, 네이밍 |
| 코드 스타일 | 10% | Rust 관용 스타일 준수 |
| 창의성 | 10% | 추가 기능, UX 개선 |

---

## 6. 교재 및 참고자료

### 6.1 주 교재

- **The Rust Programming Language** (Official Rust Book)
  - 무료 온라인: https://doc.rust-lang.org/book/
  - 한국어 번역: https://doc.rust-lang.kr/book/

### 6.2 부 교재

- **Rust by Example**
  - https://doc.rust-lang.org/rust-by-example/

- **Rust for Rustaceans**
  - Jon Gjengset, O'Reilly Media

### 6.3 온라인 자료

- **Rust Standard Library**
  - https://doc.rust-lang.org/std/

- **Rust Language Reference**
  - https://doc.rust-lang.org/reference/

- **Rust Playground** (웹에서 Rust 실행)
  - https://play.rust-lang.org/

### 6.4 추천 영상

- **Rust Programming Language Tutorial** (freeCodeCamp)
- **Crust of Rust** (Jon Gjengset)
- **Writing an OS in Rust** (Phil Opp)

---

## 부록: 추가 학습 자료

### A. 프로젝트 파일 구조

```
dungeon_crawler/
├── Cargo.toml
├── src/
│   ├── main.rs          (메인 게임 루프)
│   ├── player.rs        (Player 구조체 및 메서드)
│   ├── monster.rs       (Monster 구조체 및 메서드)
│   ├── item.rs          (Item 구조체 및 ItemType)
│   ├── combat.rs        (전투 시스템)
│   ├── inventory.rs     (인벤토리 시스템)
│   ├── shop.rs          (상점 시스템)
│   ├── dungeon.rs       (던전 생성)
│   ├── save.rs          (저장/로드)
│   └── utils.rs         (유� 함수)
├── tests/
│   └── integration_test.rs
├── README.md
└── .gitignore
```

### B. Git 사용법 기초

```bash
# 저장소 초기화
git init

# .gitignore 생성 (cargo new가 자동 생성)
# .gitignore에 target/ 추가

# 첫 커밋
git add .
git commit -m "feat: Week 1 - 게임 제목 화면"

# 원격 저장소 연결
git remote add origin https://github.com/username/dungeon-crawler.git

# 푸시
git push -u origin main
```

### C. 유용한 Cargo 명령어

```bash
# 새 프로젝트 생성
cargo new dungeon_crawler

# 빌드
cargo build

# 실행
cargo run

# 체크 (컴파일만 확인)
cargo check

# 테스트
cargo test

# 문서 생성
cargo doc --open

# 포맷팅
cargo fmt

# 린트
cargo clippy
```

### D. Rust 게임 개발 라이브러리

추천 라이브러리 (선택적 사용):
- **ggez**: 2D 게임 엔진
- **bevy**: ECS 기반 게임 엔진
- **tui**: 터미널 UI 라이브러리
- **crossterm**: 터미널 UI 라이브러리
- **rand**: 난수 생성

---

*본 강의 계획서는 학사의 학습 진도와 이해도에 따라 변경될 수 있습니다.*
