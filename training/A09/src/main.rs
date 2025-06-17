use std::{io::Write, vec};

fn run<W: Write>(w: &mut W, stdin: String) {
    let mut lines = stdin.lines();
    let hwn = lines
        .next()
        .expect("No input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Failed to parse number"))
        .collect::<Vec<usize>>();
    let (height, width, n) = (hwn[0], hwn[1], hwn[2]);
    let mut z: Vec<Vec<usize>> = vec![vec![0; width]; height];

    for _ in 0..n {
        let data = lines
            .next()
            .expect("No input")
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Failed to parse number"))
            .collect::<Vec<usize>>();
        let (a, b, c, d) = (data[0], data[1], data[2], data[3]);
        for j in a - 1..c {
            z[j][b - 1..d].iter_mut().for_each(|x| *x += 1);
        }
    }

    for h in 0..height {
        writeln!(w, "{}", {
            z[h].iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .expect("Failed to write output");
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
        let input = r#"5 5 2
1 1 3 3
2 2 4 4
"#
        .into();
        let mut output = Vec::new();
        run(&mut output, input);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            r#"1 1 1 0 0
1 2 2 1 0
1 2 2 1 0
0 1 1 1 0
0 0 0 0 0
"#
        );
    }
}
