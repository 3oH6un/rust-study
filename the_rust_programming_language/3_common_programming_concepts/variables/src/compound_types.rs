/// 복합 타입 - 튜플
/// 튜플은 서로 다른 타입의 여러 값을 하나의 타입으로 그룹화할 수 있다.
/// 구조 분해를 통해 튜플의 값을 추출할 수 있다.
pub fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("y의 값: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}

/// 복합 타입 - 배열
/// 배열은 동일한 타입의 값을 여러 개 그룹화할 수 있다.
/// 배열의 길이는 고정되어 있으며, 한 번 생성되면 변경할 수 없다.
pub fn array() {
    let a = [1, 2, 3, 4, 5];
    print!("a = [");
    print_array(&a);

    let first = a[0];
    println!("a[0] = {}", first);

    let second = a[1];
    println!("a[1] = {}", second);

    let a = [3; 5]; // [3, 3, 3, 3, 3]
    print_array(&a);
}

fn print_array(&a: &[i32; 5]) {
    print!("a = [");
    for i in 0..a.len() {
        if i == a.len() - 1 {
            println!("{}]", a[i]);
        } else {
            print!("{}, ", a[i]);
        }
    }
}

use std::io;
/// 올바르지 않은 인덱스로 배열에 접근하는 경우
pub fn index_out_of_bounds() {
    let a = [1, 2, 3, 4, 5];

    println!("배열 인덱스를 입력해주세요.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("입력값을 읽을 수 없습니다.");

    let index: usize = index
        .trim()
        .parse()
        .expect("입력값을 숫자로 변환할 수 없습니다.");

    let element = a[index];

    println!("{index} 번째 인덱스의 값: {element}");
}