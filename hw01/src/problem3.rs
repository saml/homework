/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut is_prime: Vec<bool> = vec![];

    for _ in 0..n {
        is_prime.push(true);
    }
    for i in 2..(n as usize) {
        let x: bool = is_prime[i];
        if x {
            for j in i..(n as usize) {
                let number = i * j;
                if number >= is_prime.len() {
                    break;
                }
                is_prime[number] = false;
            }
        }
    }

    let mut result: Vec<u32> = vec![];
    for i in 2..n {
        if is_prime[i as usize] {
            result.push(i);
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sieve_basic() {
        assert_eq!(vec![2, 3, 5, 7, 11], sieve(12));
    }

}
