use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

#[derive(Debug)]
pub enum Cell {
    Vert,
    Horiz,
    Start,
    Cross,
    Turn(/*inv*/ bool),
    Void,
}

type Map = Vec<Vec<Cell>>;

#[derive(Clone, Debug)]
pub struct Cart {
    x: i32,
    y: i32,
    dir: (i32, i32),
    next_turn: u8,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (Map, Vec<Cart>);

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut carts = vec![];
        let map = data
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '-' => Cell::Horiz,
                        '|' => Cell::Vert,
                        '+' => Cell::Cross,
                        ' ' => Cell::Void,
                        '/' => Cell::Turn(true),
                        '\\' => Cell::Turn(false),
                        c => {
                            carts.push(Cart {
                                x: x as i32,
                                y: y as i32,
                                dir: match c {
                                    'v' => (0, 1),
                                    '^' => (0, -1),
                                    '>' => (1, 0),
                                    '<' => (-1, 0),
                                    _ => unimplemented!(),
                                },
                                next_turn: 0,
                            });
                            Cell::Start
                        }
                    })
                    .collect()
            })
            .collect();

        Ok((map, carts))
    }

    fn part1(&mut self, (map, carts): &Self::Input) -> Result<String> {
        let mut carts = carts.clone();
        let collision = loop {
            carts.sort_by(cart_comp);
            let res = run(map, &mut carts);
            match res {
                Ok(()) => (),
                Err(pos) => break pos,
            }
        };
        Ok(format!("{},{}", collision.0, collision.1))
    }

    fn part2(&mut self, (map, carts): &Self::Input) -> Result<String> {
        let mut carts = carts.clone();
        while carts.len() > 1 {
            carts.sort_by(cart_comp);
            run(map, &mut carts).unwrap_or_default();
        }
        Ok(format!("{},{}", carts[0].x, carts[0].y))
    }
}

fn cart_comp(a: &Cart, b: &Cart) -> std::cmp::Ordering {
    a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x))
}

fn run(map: &Map, carts: &mut Vec<Cart>) -> std::result::Result<(), (i32, i32)> {
    let mut result = Ok(());
    let mut removed = vec![];

    for i in 0..carts.len() {
        let mut cart = &mut carts[i];
        cart.x += cart.dir.0;
        cart.y += cart.dir.1;
        let (x, y) = (cart.x, cart.y);

        match map[y as usize][x as usize] {
            Cell::Cross => {
                cart.dir = match cart.next_turn {
                    0b00 => (cart.dir.1, -cart.dir.0),
                    0b01 => cart.dir,
                    0b10 => (-cart.dir.1, cart.dir.0),
                    _ => unreachable!(),
                };
                cart.next_turn = (cart.next_turn + 1) % 3;
            }
            Cell::Turn(false) => cart.dir = (cart.dir.1, cart.dir.0),
            Cell::Turn(true) => cart.dir = (-cart.dir.1, -cart.dir.0),
            Cell::Vert | Cell::Horiz | Cell::Start | Cell::Void => (),
        }

        // check collision
        if let Some((j, _)) = carts
            .iter()
            .enumerate()
            .find(|(j, c)| c.x == x && c.y == y && i != *j)
        {
            result = Err((x, y));
            removed.push(j);
            removed.push(i);
        }
    }
    *carts = carts
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if removed.contains(&i) {
                None
            } else {
                Some(c.clone())
            }
        })
        .collect();
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "7,3";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = r#"/>-<\  
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;
        let expected = "6,4";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
