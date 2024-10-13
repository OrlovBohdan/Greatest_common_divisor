fn gcd(mut a: u32, mut b: u32) -> u32 {
    // Використовуємо алгоритм Евкліда для знаходження GCD
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    // Тести для перевірки функції
    assert_eq!(gcd(15, 6), 3);
    assert_eq!(gcd(60, 24), 12);

    println!("Усі тести пройдені успішно!");
}
