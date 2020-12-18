use std::collections::BTreeMap;

const NAME: &'static str = "day16";

day! {
    type INPUT = (BTreeMap<String, (u32, u32, u32, u32)>, Vec<u32>, Vec<Vec<u32>>);

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let mut it = data.lines();
        let mut fields =  BTreeMap::new();
        loop {
            let l = it.next().unwrap();
            let sep = match l.find(':') {
                Some(p) => p,
                None => {break;}
            };
            let (key, tail) = l.split_at(sep);
            let nums: Vec<u32> = tail[2..]
                .split(|c| c == '-' || c == ' ')
                .filter_map(|x| x.parse().ok())
                .collect();
            fields.insert(String::from(key), (nums[0], nums[1], nums[2], nums[3]));
        }
        it.next();

        let mine: Vec<u32> = it.next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        it.next();
        it.next();
        let tickets = it.map(
            |x| x.split(',').map(|x| x.parse().unwrap()).collect()
        ).collect();

        Ok((fields, mine, tickets))
    }

    fn part1(input: &Self::INPUT) -> Result<String> {
        let (fields, _mine, tickets) = input;
        let (error_rate, _) = check(&fields, &tickets);
        Ok(format!("{}", error_rate))
    }

    fn part2(input: &Self::INPUT) -> Result<String> {
        let (fields, mine, tickets) = input;
        let (_, valid) = check(&fields, &tickets);

        let mut field_map: Vec<Vec<&String>> = (0..mine.len())
            .map(|_| fields.keys().collect())
            .collect();
        for t in &valid {
            for (i, x) in t.iter().enumerate() {
                field_map[i].retain(|field| {
                    let (a_min, a_max, b_min, b_max) = fields[*field];
                    x >= &a_min && x <= &a_max || x >= &b_min && x <= &b_max
                });
            }
        }

        // dumb mapping reduction
        let field_count = mine.len();
        for _ in 0..field_count {
            for i in 0..field_count {
                let (head, tail) = field_map.split_at_mut(i);
                let (elem, tail) = tail.split_at_mut(1);
                if elem[0].len() == 1 {
                    let elem = elem[0][0];
                    for sub in itertools::chain(head, tail) {
                        sub.retain(|x| *x != elem);
                    }
                }
            }
        }
        Ok(format!("{}", field_map.iter().enumerate().filter_map(|(i, x)| {
            if x[0].starts_with("departure"){
                Some(mine[i] as u64)
            } else {
                None
            }
        }).product::<u64>()))
    }
}

fn check<'a>(
    fields: &BTreeMap<String, (u32, u32, u32, u32)>,
    tickets: &'a Vec<Vec<u32>>,
) -> (u32, Vec<&'a Vec<u32>>) {
    let mut valid = vec![];
    let mut error_rate = 0;
    'tickets: for t in tickets {
        'nums: for x in t {
            for (a_min, a_max, b_min, b_max) in fields.values() {
                if (x >= a_min && x <= a_max) || (x >= b_min && x <= b_max) {
                    continue 'nums;
                }
            }
            error_rate += x;
            continue 'tickets;
        }
        valid.push(t);
    }
    (error_rate, valid)
}
