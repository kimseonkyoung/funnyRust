fn main() {
    // 1. loop - 무한 루프, break으로 탈출
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }
    println!("count = {}", count); // 3

    // 2. loop는 표현식 - break에 값을 실어서 반환
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("result = {}", result); // 20

    // 3. while - 조건이 true인 동안 반복
    let mut n = 0;
    while n < 5 {
        println!("while n = {}", n);
        n += 1;
    }

    // 4. for - 범위 순회 (0..5 는 0,1,2,3,4)
    for i in 0..5 {
        println!("for i = {}", i);
    }

    // 5. for - 범위 끝 포함 (0..=5 는 0,1,2,3,4,5)
    for i in 0..=5 {
        println!("for i = {}", i);
    }

    // 6. for - 배열 순회 (인덱스 없이 원소 직접 접근)
    let arr = [10, 20, 30, 40, 50];
    for val in arr {
        println!("val = {}", val);
    }

    // 7. for - 인덱스가 필요할 때는 enumerate() 사용
    for (i, val) in arr.iter().enumerate() {
        println!("arr[{}] = {}", i, val);
    }
}
