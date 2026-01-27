fn main() {
    let mut s = String::from("헬로");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;

    // 오류. immutable 참조자가 존재하는 동안 mutable 참조자를 만들 수 없음
    // println!("{} and {} and {}", r1, r2, r3);

    // r1과 r2가 더 이상 사용되지 않으므로, 이제 mutable 참조자를 만들 수 있음
    println!("r3 is {}", r3);
}
