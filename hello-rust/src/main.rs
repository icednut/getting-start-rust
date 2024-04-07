use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new(); // mut는 mutable의 약자로 mut가 붙지 않은 vector는 아무런 값도 넣을 수 없다.

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument")); // expect 값이 기대한대로 나오면 정상적인 값을 반환 아니면 panic
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ..."); // 해당 매크로는 표준 오류 추력 스트림에 오류 메시지를 기록
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); // !는 이것이 함수 호출이 아니라 매크로 호출이라는 것을 의미
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}