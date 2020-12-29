day! {
    day18;
    type INPUT = Vec<String>;

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        Ok(file.lines().map(|s| s.unwrap()).collect())
    }

    fn part1(input: &Self::INPUT) -> Result<std::string::String> {
        let res: u64 = input.iter().map(|l| calc(0, &Tok::Add, &mut tokenize(l).iter())).sum();
        Ok(format!("{}", res))
    }

    fn part2(input: &Self::INPUT) -> Result<std::string::String> {
        let res: u64 = input.iter().map(|l| calc2(0, &Tok::Add, &mut tokenize(l).iter())).sum();
        Ok(format!("{}", res))
    }
}

fn calc<'a>(operand: u64, operator: &Tok, expr: &mut impl Iterator<Item = &'a Tok>) -> u64 {
    let b = match expr.next().unwrap() {
        Tok::Lit(x) => *x,
        Tok::Start => calc(0, &Tok::Add, expr),
        _ => unreachable!(),
    };
    let res = if let Tok::Add = operator {
        operand + b
    } else {
        operand * b
    };
    let op = expr.next();
    if let Some(Tok::End) | None = op {
        return res;
    }
    calc(res, op.unwrap(), expr)
}

fn calc2<'a>(operand: u64, operator: &Tok, expr: &mut impl Iterator<Item = &'a Tok>) -> u64 {
    let b = match expr.next().unwrap() {
        Tok::Lit(x) => *x,
        Tok::Start => calc2(0, &Tok::Add, expr),
        _ => unreachable!(),
    };
    let op = expr.next();
    if let Some(Tok::End) | None = op {
        if let Tok::Add = operator {
            return operand + b;
        } else {
            return operand * b;
        };
    }

    if let Tok::Add = operator {
        return calc2(operand + b, op.unwrap(), expr);
    } else {
        return operand * calc2(b, op.unwrap(), expr);
    };
}

enum Tok {
    Start,
    End,
    Lit(u64),
    Add,
    Mult,
}

fn tokenize(expr: &str) -> Vec<Tok> {
    let mut res = vec![];
    let mut it = expr.bytes();
    while let Some(c) = it.next() {
        match c {
            b'(' => res.push(Tok::Start),
            b')' => res.push(Tok::End),
            b'+' => res.push(Tok::Add),
            b'*' => res.push(Tok::Mult),
            b' ' => continue,
            x => {
                // Blindly assume valid expressions, like a bad dev.
                let lit = (x - b'0') as u64;
                res.push(Tok::Lit(lit));
            }
        };
    }
    res
}
