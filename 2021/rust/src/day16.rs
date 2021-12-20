#[derive(Default)]
pub struct Day {}

#[derive(Debug, PartialEq)]
enum Sub {
    Literal(usize),
    Op(Vec<Packet>),
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    typ: usize,
    data: Sub,
}

impl crate::Day for Day {
    type Input = Vec<bool>;

    fn gen(&self, data: &str) -> Self::Input {
        let data = data.trim_end();
        let mut res = vec![];
        (0..data.len()).for_each(|i| {
            let mut byte = u8::from_str_radix(&data[i..i + 1], 16).unwrap();
            for _ in 0..4 {
                res.push((byte & 0b1000) != 0);
                byte <<= 1;
            }
        });
        res
    }

    fn part1(&self, input: &Self::Input) -> String {
        let (res, _) = parse_pkt(&mut input.iter().copied());

        format!("{}", ver_sum(&res))
    }

    fn part2(&self, input: &Self::Input) -> String {
        let (seq, _) = parse_pkt(&mut input.iter().copied());
        let res = calc(&seq);
        format!("{}", res)
    }
}

fn calc(pkt: &Packet) -> usize {
    match &pkt.data {
        Sub::Op(inner) => match &pkt.typ {
            0 => inner.iter().map(|x| calc(x)).sum(),
            1 => inner.iter().map(|x| calc(x)).product(),
            2 => inner.iter().map(|x| calc(x)).min().unwrap(),
            3 => inner.iter().map(|x| calc(x)).max().unwrap(),
            5 => (calc(&inner[0]) > calc(&inner[1])).into(),
            6 => (calc(&inner[0]) < calc(&inner[1])).into(),
            7 => (calc(&inner[0]) == calc(&inner[1])).into(),
            _ => unimplemented!(),
        },
        Sub::Literal(l) => *l,
    }
}

fn bit_dec<T, I>(bits: &mut I, len: usize) -> T
where
    T: From<usize>,
    I: Iterator<Item = bool>,
{
    let mut res: usize = 0;
    for _ in 0..len {
        res = res * 2 + if bits.next().unwrap() { 1 } else { 0 };
    }
    T::from(res)
}

fn parse_pkt<I: Iterator<Item = bool>>(data: &mut I) -> (Packet, usize) {
    let version = bit_dec(data, 3);
    let typ = bit_dec(data, 3);
    let (data, read) = match &typ {
        4 => parse_literal(data),
        _ => parse_op(data),
    };
    (Packet { version, typ, data }, read + 6)
}

fn parse_literal<I: Iterator<Item = bool>>(data: &mut I) -> (Sub, usize) {
    let mut total = 0;
    let mut read = 0;
    loop {
        read += 5;
        let is_not_last = data.next().unwrap();
        let inc: usize = bit_dec(data, 4);
        total = total * 16 + inc;
        if !is_not_last {
            return (Sub::Literal(total), read);
        }
    }
}

fn parse_op<I: Iterator<Item = bool>>(data: &mut I) -> (Sub, usize) {
    let mut inner = vec![];
    let mut read = 0;
    if !data.next().unwrap() {
        let len: usize = bit_dec(data, 15);
        while read < len {
            let (pkt, pkt_len) = parse_pkt(data);
            read += pkt_len;
            inner.push(pkt);
        }
        read += 16;
    } else {
        let num_pkt: usize = bit_dec(data, 11);
        read += 12;
        for _ in 0..num_pkt {
            let (pkt, pkt_len) = parse_pkt(data);
            read += pkt_len;
            inner.push(pkt);
        }
    }
    (Sub::Op(inner), read)
}

/// add versions from packet recursively.
fn ver_sum(packet: &Packet) -> usize {
    packet.version
        + match &packet.data {
            Sub::Op(s) => s.iter().map(ver_sum).sum(),
            _ => 0,
        }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Day as _;

    #[test]
    fn lit() {
        let data = Day {}.gen("D2FE28");
        assert_eq!(
            Packet {
                version: 6,
                typ: 4,
                data: Sub::Literal(2021),
            },
            parse_pkt(&mut data.iter().copied()).0
        )
    }

    #[test]
    fn op() {
        let data = Day {}.gen("38006F45291200");
        let parsed = parse_pkt(&mut data.iter().copied()).0;
        assert_eq!("Packet { version: 1, typ: 6, data: Op([Packet { version: 6, typ: 4, data: Literal(10) }, Packet { version: 2, typ: 4, data: Literal(20) }]) }", format!("{:?}", parsed));
    }
}
