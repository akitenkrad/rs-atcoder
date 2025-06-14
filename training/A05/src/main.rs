use std::io::Write;

fn run<W: Write>(w: &mut W, stdin: String) {
    let mut lines = stdin.lines();
    let data: Vec<usize> = lines
        .next()
        .expect("No input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Failed to parse number"))
        .collect();

    let (n, k) = (data[0], data[1]);

    let mut answer = 0;
    for r in 1..=n {
        for b in 1..=n {
            if r + b > k {
                break; // No need to continue if the sum exceeds k
            } else if r + b + n < k {
                continue; // If the maximum possible sum with current r and b is less than k, skip
            }

            let x = k - r - b;
            if 1 <= x && x <= n {
                answer += 1;
            }
        }
        if r > k {
            break; // No need to continue if r exceeds k
        } else if r + n + n < k {
            continue; // If the maximum possible sum with current r is less than k, skip
        }
    }

    write!(w, "{}", answer).expect("Failed to write output");
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).expect("Failed to read from stdin");
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    run(&mut stdout, stdin);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "3 6\n".to_string();
        let mut output = Vec::new();
        run(&mut output, input);
        assert_eq!(String::from_utf8(output).unwrap(), "7");
    }

    #[test]
    fn test_2() {
        let input = "3000 4000\n".to_string();
        let mut output = Vec::new();
        run(&mut output, input);
        assert_eq!(String::from_utf8(output).unwrap(), "6498498");
    }
}
