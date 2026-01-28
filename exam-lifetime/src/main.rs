// The Borrow Checker
//   참조의 수명보다, 원래 값의 수명이 같거나 길어야 함

fn main() {
    let s1 = String::from("가나다");
    let s2 = "하나둘셋";

    let res = longest(s1.as_str(), s2);
    println!("더 긴 문자열은 {}", res);
}

fn longest(s1: &str, s2: &str) -> &str {  // 오류: borrow한 값인 s1 또는 s2를 리턴해야 하는데, 컴파일러는 모름 (실행시점에서만 알 수 있음)
    if s1.len() > s2.len() {              //       따라서 명시적으로 borrow lifetime을 명시해 줘야 함
        s1
    } else {
        s2
    }
}
