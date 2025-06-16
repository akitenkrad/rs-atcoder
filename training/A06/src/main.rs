use std::io::Write;

fn run<W: Write>(w: &mut W, stdin: String) {
    let mut lines = stdin.lines();
    let data: Vec<usize> = lines
        .next()
        .expect("No input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Failed to parse number"))
        .collect();

    let (_, q) = (data[0], data[1]);
    let mut a = Vec::new();
    for i in lines
        .next()
        .expect("No input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Failed to parse number"))
    {
        let a_prev = if a.is_empty() { 0 } else { *a.last().unwrap() };
        a.push(a_prev + i);
    }

    for _ in 0..q {
        let q_i = lines
            .next()
            .expect("No input")
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Failed to parse number"))
            .collect::<Vec<usize>>();
        let (l, r) = (q_i[0], q_i[1]);
        let sum = if l == 1 {
            a[r - 1]
        } else {
            a[r - 1] - a[l - 2]
        };
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
        let input = r#"10 5
8 6 9 1 2 1 10 100 1000 10000
2 3
1 4
3 9
6 8
1 10
"#
        .into();
        let mut output = Vec::new();
        run(&mut output, input);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            r#"15
24
1123
111
11137
"#
        );
    }
}
