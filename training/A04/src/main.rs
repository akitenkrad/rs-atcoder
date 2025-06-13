use std::io::Write;

fn run<W: Write>(w: &mut W, stdin: String) {
    let mut lines = stdin.lines();
    let n = lines
        .next()
        .expect("No input")
        .parse::<usize>()
        .expect("Failed to parse number");

    // print n as binary number
    let binary = format!("{:b}", n);
    write!(w, "{:0>10}", binary).expect("Failed to write output");
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
        let input = "13\n".to_string();
        let mut output = Vec::new();
        run(&mut output, input);
        assert_eq!(String::from_utf8(output).unwrap(), "0000001101");
    }

    #[test]
    fn test_2() {
        let input = "37\n".to_string();
        let mut output = Vec::new();
        run(&mut output, input);
        assert_eq!(String::from_utf8(output).unwrap(), "0000100101");
    }
}
