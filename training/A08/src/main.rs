use std::{io::Write, vec};

fn run<W: Write>(w: &mut W, stdin: String) {
    let mut lines = stdin.lines();
    let hw = lines
        .next()
        .expect("No input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Failed to parse number"))
        .collect::<Vec<usize>>();
    let (height, width) = (hw[0], hw[1]);
    let mut x: Vec<Vec<usize>> = vec![vec![0; width]; height];
    for i in 0..height {
        let data = lines
            .next()
            .expect("No input")
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Failed to parse number"))
            .collect::<Vec<usize>>();
        for (j, &value) in data.iter().enumerate() {
            x[i][j] = value;
        }
    }

    let q = lines
        .next()
        .expect("No input")
        .parse::<usize>()
        .expect("Failed to parse number");
    for _ in 0..q {
        let data = lines
            .next()
            .expect("No input")
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Failed to parse number"))
            .collect::<Vec<usize>>();
        let (a, b, c, d) = (data[0], data[1], data[2], data[3]);
        let mut sum = 0;
        for i in a - 1..c {
            sum += x[i][b - 1..d].iter().sum::<usize>();
        }
        writeln!(w, "{}", sum).expect("Failed to write output");
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
    fn test_1() {
        let input = r#"5 5
2 0 0 5 1
1 0 3 0 0
0 8 5 0 2
4 1 0 0 6
0 9 2 7 0
2
2 2 4 5
1 1 5 5
"#
        .into();
        let mut output = Vec::new();
        run(&mut output, input);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            r#"25
56
"#
        );
    }
}
