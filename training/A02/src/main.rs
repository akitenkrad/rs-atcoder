use std::io::Write;

fn run<W: Write>(w: &mut W, stdin: String) {
    let mut lines = stdin.lines();
    let line_1 = lines
        .next()
        .expect("No input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Failed to parse number"))
        .collect::<Vec<_>>();
    let x = line_1[1];
    let a: Vec<usize> = lines
        .next()
        .expect("No input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Failed to parse number"))
        .collect();
    let out = a.iter().filter(|&&a_i| a_i == x).collect::<Vec<&usize>>();
    if out.is_empty() {
        writeln!(w, "No").expect("Failed to write output");
    } else {
        writeln!(w, "Yes").expect("Failed to write output");
    }
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
    fn test_example() {
        let input = "5 3\n1 2 3 4 5\n".to_string();
        let mut output = Vec::new();
        run(&mut output, input);
        assert_eq!(String::from_utf8(output).unwrap(), "Yes\n");
    }

    #[test]
    fn test_no_match() {
        let input = "5 6\n1 2 3 4 5\n".to_string();
        let mut output = Vec::new();
        run(&mut output, input);
        assert_eq!(String::from_utf8(output).unwrap(), "No\n");
    }
}
