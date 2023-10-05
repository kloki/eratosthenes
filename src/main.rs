use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// return primes up to n
    n: usize,
}
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut sieve = vec![true; n + 1];
    let mut p = 2;

    while p * p <= n {
        if sieve[p] {
            for i in ((p * p)..n + 1).step_by(p) {
                sieve[i] = false;
            }
        }
        p += 1;
    }
    sieve
        .iter()
        .enumerate()
        .filter(|(i, x)| **x && i > &1)
        .map(|(i, _)| i)
        .collect()
}

fn main() {
    let result = sieve_of_eratosthenes(Args::parse().n);
    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[cfg(test)]
mod tests {
    use super::sieve_of_eratosthenes;

    #[test]
    fn test_1() {
        assert_eq!(sieve_of_eratosthenes(1), vec![])
    }
    #[test]
    fn test_10() {
        assert_eq!(sieve_of_eratosthenes(10), vec![2, 3, 5, 7])
    }
    #[test]
    fn test_100() {
        assert_eq!(
            sieve_of_eratosthenes(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        )
    }
    #[test]
    fn test_10_000_000() {
        assert_eq!(sieve_of_eratosthenes(10_000_000).len(), 664579)
    }
}
