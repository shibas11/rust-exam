fn main() {
    let numbers = vec![3, 4, 1, 6, 8, 10];

    let result = smallest_i32(&numbers);
    println!("가장 작은 수는 {}", result);

    let chars = vec!['홍', '길', '동'];

    let result = smallest_char(&chars);
    println!("가장 작은 글자는 {}", result);
}

fn smallest_i32(list: &[i32]) -> &i32 {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn smallest_char(list: &[char]) -> &char {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}
