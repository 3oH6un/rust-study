mod scalar_types;
mod compound_types;

fn main() {
    variables_declaration();
    constants_declaration();
    variable_shadowing();
    type_annotations();

    // scalar_types
    scalar_types::integer();
    scalar_types::float();
    scalar_types::basic_operation();
    scalar_types::boolean();
    scalar_types::character();

    // compound_types
    compound_types::tuple();
    compound_types::array();
    compound_types::index_out_of_bounds();
}

/// 변수 선언
/// 변수는 let 키워드를 사용하여 선언하며, mut 키워드를 사용하지 않으면 불변 변수로 선언된다.
fn variables_declaration() {
    let mut x = 5;
    println!("x의 값은 : {}", x);
    x = 6;
    println!("x의 값은 : {}", x);
}

/// 상수 선언
/// 상수는 const 키워드를 사용하여 선언하며, 반드시 타입을 명시해야 한다.
fn constants_declaration() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS의 값은 : {}", THREE_HOURS_IN_SECONDS);
}

/// 변수 섀도잉
/// let 키워드를 사용하여 변수를 여러 번 선언할 수 있다.
/// 이때, 변수의 값은 가장 최근에 선언된 변수의 값이 된다.
/// 스코프 내에서 선언된 변수는 해당 스코프 내에서만 유효하다.
fn variable_shadowing() {
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("y의 안쪽 스코프 값은 : {}", y);
    }
    println!("y의 값은 : {}", y);
    // 변수 섀도잉을 사용하여 변수의 타입을 변경할 수 있다.
    let spaces = "   ";
    let spaces = spaces.len();
}

/// 데이터 타입
/// 러스트는 정적 타입 언어로 변수의 타입을 명시해야 한다.
fn type_annotations() {
    // let guess = "42".parse().expect("숫자가 아닙니다.");
    let guess: u32 = "42".parse().expect("숫자가 아닙니다.");
}
