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

### 주요 기능

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

### 설치

```bash
# 저장소 복제
git clone https://github.com/your-username/dungeon-crawler.git
cd dungeon-crawler

# 실행
cargo run --release
```

### 게임 실행

```bash
cargo run
```

---

## 📖 사용법

### 게임 시작

```
=====================================
           DUNGEON CRAWLER
=====================================

Press Enter to start...
```

### 캐릭터 생성

```
용사의 이름을 입력하세요: 철수

직업을 선택하세요:
1. 전사 (HP: 100 | MP: 50 | 공격력: 15 | 방어력: 5)
2. 마법사 (HP: 60 | MP: 100 | 공격력: 10 | 방어력: 3)
3. 도적 (HP: 80 | MP: 60 | 공격력: 12 | 방어력: 8)

선택: 1
```

### 메인 메뉴

```
=====================================
 용사: 철수 (Lv.1 전사)
 HP: [████████░░] 80/100  MP: [███░░░░░░] 30/100
 공격력: 15  방어력: 5  골드: 50G
=====================================

[메뉴]
1. 던전 입장
2. 상점
3. 휴식
4. 인벤토리
5. 상태 보기
6. 저장/로드
7. 종료

선택: _
```

---

## 🏗️ 프로젝트 구조

```
dungeon_crawler/
├── Cargo.toml           # 프로젝트 설정
├── src/
│   ├── main.rs          # 메인 게임 루프
│   ├── player.rs        # Player 구조체
│   ├── monster.rs       # Monster 구조체
│   ├── item.rs          # Item, ItemType 열거형
│   ├── combat.rs        # 전투 시스템
│   ├── inventory.rs     # 인벤토리 관리
│   ├── shop.rs          # 상점 시스템
│   ├── dungeon.rs       # 던전 생성
│   ├── save.rs          # 저장/로드
│   └── utils.rs         # 유� 함수
├── tests/               # 테스트 코드
├── README.md            # 이 파일
└── .gitignore
```

---

## 📚 학습 로드맵

이 프로젝트는 프로그래밍 기초 강의의 실습 과제로 설계되었습니다. 각 주차별로 새로운 기능이 추가됩니다:

| 주차 | 추가 기능 | 학습 개념 |
|------|-----------|----------|
| 1 | 게임 제목 화면 | Hello World, `println!` |
| 2 | 캐릭터 생성 | 변수, 타입, `const` |
| 3 | 전투 데미지 계산 | 연산자, 표현식 |
| 4 | 승리/패배 조건 | `if`, `match` |
| 5 | 게임 루프 | `loop`, `while`, `for` |
| 7 | 전투 함수화 | 함수, 매개변수, 반환값 |
| 8 | 보물 상자 | 재귀 함수 |
| 9 | 구조체 도입 | `struct`, `impl`, 메서드 |
| 10 | 열거형과 Option | `enum`, `Option<T>`, 패턴 매칭 |
| 11 | 인벤토리 | `Vec<T>`, `String` |
| 12 | 도감 & 상점 | `HashMap<K, V>`, `Result<T, E>` |
| 13 | 정렬 시스템 | 버블 정렬 등 |
| 14 | 검색 기능 | 선형 탐색, 이진 탐색 |
| 15 | 최종 완성 | 전체 통합 |

---

## 🎯 게임 플레이 가이드

### 전사 (Warrior)

- **장점**: 높은 HP와 방어력, 안정적인 전투
- **단점**: 낮은 MP, 마법 스킬 부족
- **추천**: 초보자에게 적합

### 마법사 (Mage)

- **장점**: 높은 MP, 강력한 마법 스킬
- **단점**: 낮은 HP와 방어력, 물리적 약함
- **추천**: 전략적 플레이어에게 적합

### 도적 (Rogue)

- **장점**: 균형 밸런스, 높은 회피율
- **단점**: 특화된 스킬 부족
- **추천**: 고수 플레이어에게 적합

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
const MAX_INVENTORY_SIZE: usize = 20;
```

### 테스트 실행

```bash
cargo test
```

### 린트 확인

```bash
cargo clippy
```

### 포맷팅

```bash
cargo fmt
```

---

## 🎨 커스터마이징

게임 설정을 수정하려면 `src/utils.rs`에 있는 상수를 변경하세요:

```rust
// 게임 밸런스
const STARTING_GOLD: u32 = 100;
const LEVEL_UP_EXP: u32 = 100;

// 인벤토리
const MAX_INVENTORY_SIZE: usize = 20;

// 던전
const MAX_DUNGEON_DEPTH: u32 = 10;
```

---

## 📝 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자유롭게 수정하고 배포하실 수 있습니다.

---

## 🙏 기여자

- 개발: [학생들 이름]
- 조교: [조교 이름]
- 강의: [교수 이름]

---

## 📮 연락처

이 프로젝트에 관한 문의나 버그 리포트는 GitHub Issues를 통해 제출해주세요.

---

## 🗺️ 스크린샷

### 게임 시작 화면
![시작 화면](screenshots/title.png)

### 전투 화면
![전투](screenshots/combat.png)

### 인벤토리
![인벤토리](screenshots/inventory.png)

---

## 📚 참고 자료

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

---

## 🔄 버전 관리

- **v0.1.0** (Week 1): 기본 화면 출력
- **v0.2.0** (Week 2): 캐릭터 생성
- **v0.3.0** (Week 3): 전투 시스템 기초
- **v0.4.0** (Week 4): 승리/패배 조건
- **v0.5.0** (Week 5): 게임 루프
- **v0.6.0** (Week 7): 함수 분리
- **v0.7.0** (Week 8): 보물 상자
- **v0.8.0** (Week 9): 구조체 도입
- **v0.9.0** (Week 10): 열거형 도입
- **v1.0.0** (Week 11): 인벤토리 완성
- **v1.1.0** (Week 12): 도감과 상점
- **v1.2.0** (Week 13): 정렬 기능
- **v1.3.0** (Week 14): 검색 기능
- **v2.0.0** (Week 15): 기말 프로젝트 완성

---

*dungeon_crawler © 2024. All rights reserved.*
