/// 스칼라 타입 - 정수형
/// 8bit, 16bit, 32bit, 64bit, 128bit, arch 정수형이 있으며 기본 값은 i32 정수형이다.
/// 부호있는 정수형은 i로 시작하고 부호없는 정수형은 u로 시작한다.
pub fn integer() {
    // 8bit 정수
    let x: i8 = 127;
    let x: u8 = 255;
    // 16bit 정수
    let x: i16 = 32767;
    let x: u16 = 65535;
    // 32bit 정수
    let x: i32 = 2147483647;    // 정수형 기본 값
    let x: u32 = 4294967295;
    // 64bit 정수
    let x: i64 = 9223372036854775807;
    let x: u64 = 18446744073709551615;
    // 128bit 정수
    let x: i128 = 170141183460469231731687303715884105727;
    let x: u128 = 340282366920938463463374607431768211455;
    // arch 정수
    let x: isize = 9223372036854775807;
    let x: usize = 18446744073709551615;
    // 정수형 리터럴
    let x: u32 = 98_222;        // 10진수
    let x: u8 = 0xff;           // 16진수
    let x: u8 = 0o77;           // 8진수
    let x: u8 = 0b1111_0000;    // 2진수
    let x: u8 = b'A';           // 바이트
}

/// 스칼라 타입 - 부동소수점
/// f32, f64의 두가지 부동소수점 타입이 있으며, 기본적으로 f64가 사용된다.
pub fn float() {
    // 32bit 부동소수점
    let x: f32 = 3.0;
    println!("x(32bit 부동소수점): {}", x);
    // 64bit 부동소수점
    let x: f64 = 3.0;           // 부동소수점 기본 값
    println!("x(64bit 부동소수점): {}", x);
    // 부동소수점 리터럴
    let x: f32 = 2.0;           // 소수점
    let x: f32 = 2.0e2;         // 지수
    let x: f32 = 0.2;           // 소수점
    let x: f32 = 0.2e-2;        // 지수
}

/// 수치 연산
/// 덧셈, 뺄셈, 곱셈, 나눗셈, 나머지 연산
pub fn basic_operation() {
    // 덧셈
    let sum = 5 + 10;
    println!("덧셈(5 + 10): {}", sum);

    // 뺄셈
    let difference = 95.5 - 4.3;
    println!("뺄셈(95.5 - 4.3): {}", difference);

    // 곱셈
    let product = 4 * 30;
    println!("곱셈(4 * 30): {}", product);

    // 나눗셈
    let quotient = 56.7 / 32.2;
    println!("나눗셈(56.7 / 32.2): {}", quotient);
    let truncated = -5 / 3;
    println!("나눗셈(-5 / 3): {}", truncated);

    // 나머지 연산
    let remainder = 43 % 5;
    println!("나머지 연산(43 % 5): {}", remainder);
}

/// 스칼라 타입 - 불리언
/// 불리언 타입은 true, false 두 가지 값만 가질 수 있다.
/// 따라서 0, 1과 같은 정수형 값은 불리언 타입으로 알아서 변환해주지 않는다.
/// 1바이트 크기를 가진다.
pub fn boolean() {
    let t: bool = true;
    println!("참: {}", t);
    let f: bool = false;
    println!("거짓: {}", f);
}

/// 스칼라 타입 - 문자
/// 문자 타입은 유니코드 스칼라 값이며, 크기는 4바이트이다.
/// 문자열의 경우 큰따옴표("")를 사용하며, 문자의 경우 작은따옴표('')를 사용한다.
pub fn character() {
    // 문자 타입
    let c = 'z';
    println!("문자(z): {}", c);
    let z: char = 'ℤ';
    println!("문자(ℤ): {}", z);
    let heart_eyed_cat = '😻';
    println!("문자(😻): {}", heart_eyed_cat);
}