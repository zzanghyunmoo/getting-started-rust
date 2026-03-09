//! Example 6: Floating-Point Types
//!
//! Demonstrates:
//! - f32 (단일 정밀도)
//! - f64 (배정밀도, 기본값)
//! - 데미지 계산 예시

fn main() {
    println!("=== 부동소수점 타입 예시 ===");
    println!();

    // f64로 자동 추론
    let damage = 15.5;
    let heal_amount: f32 = 25.0;

    println!("데미지: {} (f64)", damage);
    println!("회복량: {} (f32)", heal_amount);

    println!();

    // 계산 예시
    let current_hp: f64 = 80.0;
    let max_hp: f64 = 100.0;
    let final_hp = current_hp - damage + heal_amount as f64;

    println!("=== HP 계산 ===");
    println!("현재 HP: {}", current_hp);
    println!("받은 데미지: -{}", damage);
    println!("회복량: +{}", heal_amount as f64);
    println!("최종 HP: {}", final_hp);

    println!();

    // 퍼센트 계산
    let hp_percent = (current_hp / max_hp) * 100.0;
    println!("HP 퍼센트: {:.1}%", hp_percent);

    println!();

    // 크리티컬 히트 계산 (1.5배 데미지)
    let base_damage = 20.0;
    let critical_multiplier = 1.5;
    let critical_damage = base_damage * critical_multiplier;

    println!("=== 크리티컬 시스템 ===");
    println!("기본 데미지: {}", base_damage);
    println!("크리티컬 배수: {}x", critical_multiplier);
    println!("크리티컬 데미지: {}", critical_damage);
}
