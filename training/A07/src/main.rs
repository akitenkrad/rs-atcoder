use std::{io::Write, vec};

fn run<W: Write>(w: &mut W, stdin: String) {
    let mut lines = stdin.lines();
    let d: usize = lines
        .next()
        .expect("No input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Failed to parse number"))
        .collect::<Vec<usize>>()[0];
    let n: usize = lines
        .next()
        .expect("No input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Failed to parse number"))
        .collect::<Vec<usize>>()[0];

    let mut ans = vec![0; d];
    for _ in 0..n {
        let data = lines
            .next()
            .expect("No input")
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Failed to parse number"))
            .collect::<Vec<usize>>();
        let (l, r) = (data[0], data[1]);
        ans[l - 1..=r - 1].iter_mut().for_each(|x| *x += 1);
    }

    for i in 0..d {
        writeln!(w, "{}", ans[i]).expect("Failed to write output");
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
        let input = r#"8
5
2 3
3 6
5 7
3 7
1 5
"#
        .into();
        let mut output = Vec::new();
        run(&mut output, input);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            r#"1
2
4
3
4
3
2
0
"#
        );
    }
}
