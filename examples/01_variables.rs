fn main() {
    // 1. 기본 변수 - Rust 변수는 기본적으로 불변(immutable)
    let x = 5;
    println!("x = {}", x);

    // x = 6; // 오류! 불변 변수는 변경 불가

    // 2. 가변 변수 - mut 키워드로 변경 가능하게 선언
    let mut y = 10;
    println!("y = {}", y);
    y = 20;
    println!("y 변경 후 = {}", y);

    // 3. 타입 명시
    let a: i32 = 100;      // 32비트 정수
    let b: f64 = 3.14;     // 64비트 부동소수점
    let c: bool = true;    // 불리언
    let d: char = 'A';     // 문자 (작은따옴표)
    let e: &str = "hello"; // 문자열 슬라이스
    println!("a={}, b={}, c={}, d={}, e={}", a, b, c, d, e);

    // 4. Shadowing - 같은 이름으로 재선언 가능
    let z = 5;
    let z = z + 1;     // z를 새로운 변수로 덮어씀
    let z = z * 2;
    println!("shadowing z = {}", z); // 12

    // shadowing은 타입도 바꿀 수 있음
    let spaces = "   ";        // &str 타입
    let spaces = spaces.len(); // usize 타입으로 변경
    println!("spaces = {}", spaces); // 3

    // 5. 상수 - const는 타입 명시 필수, 절대 변경 불가
    const MAX_SCORE: u32 = 100;
    println!("MAX_SCORE = {}", MAX_SCORE);
}
