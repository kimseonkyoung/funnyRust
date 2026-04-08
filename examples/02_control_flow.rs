fn main() {
    // 1. 기본 if / else
    let x = 5;

    if x > 3 {
        println!("x는 3보다 크다");
    } else if x == 3 {
        println!("x는 3이다");
    } else {
        println!("x는 3보다 작다");
    }

    // 2. if는 표현식 - 값을 변수에 바로 대입 가능
    let result = if x > 3 { "크다" } else { "작다" };
    println!("result = {}", result);

    // 3. 기본 match
    let y = 3;

    match y {
        1 => println!("하나"),
        2 => println!("둘"),
        3 => println!("셋"),
        _ => println!("그 외"), // 나머지 모든 경우 (필수)
    }

    // 4. match 범위 매칭
    let score = 75;

    match score {
        90..=100 => println!("A"),
        80..=89  => println!("B"),
        70..=79  => println!("C"),
        60..=69  => println!("D"),
        _        => println!("F"),
    }

    // 5. match는 표현식 - 값 반환 가능
    let grade = match score {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        _        => "F",
    };
    println!("grade = {}", grade);

    // 6. match 다중 패턴 - | 로 여러 값 한번에 처리
    let day = 6;

    let day_type = match day {
        1 | 7 => "주말",
        2..=6 => "평일",
        _     => "잘못된 값",
    };
    println!("day_type = {}", day_type);
}
