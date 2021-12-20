#[derive(Default)]
pub struct Day {}

impl crate::Day for Day {
    type Input = (Vec<bool>, Vec<Vec<bool>>);

    fn gen(&self, data: &str) -> Self::Input {
        let mut lines = data.trim_end().lines();
        let alg: Vec<bool> = lines.next().unwrap().bytes().map(|b| b == b'#').collect();
        assert_eq!(512, alg.len());
        lines.next();
        let img = lines
            .map(|l| l.bytes().map(|b| b == b'#').collect())
            .collect();

        (alg, img)
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut data = pad(&input.1);
        data = expand(&data);
        for _ in 0..2 {
            data = expand(&data);
            data = enhance(&data, &input.0);
            infinify(&mut data);
        }
        let res = data.iter().flatten().filter(|c| **c).count();
        format!("{}", res)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut data = pad(&input.1);
        data = expand(&data);
        for _ in 0..50 {
            data = expand(&data);
            data = enhance(&data, &input.0);
            infinify(&mut data);
        }
        let res = data.iter().flatten().filter(|c| **c).count();
        format!("{}", res)
    }
}

fn enhance(img: &[Vec<bool>], lookup: &[bool]) -> Vec<Vec<bool>> {
    let width = img[0].len();
    let height = img.len();
    let mut res = vec![vec![false; width]; height];
    for j in 1..(height - 1) {
        for i in 1..(width - 1) {
            let idx = ((img[j - 1][i - 1] as usize) << 8)
                + ((img[j - 1][i] as usize) << 7)
                + ((img[j - 1][i + 1] as usize) << 6)
                + ((img[j][i - 1] as usize) << 5)
                + ((img[j][i] as usize) << 4)
                + ((img[j][i + 1] as usize) << 3)
                + ((img[j + 1][i - 1] as usize) << 2)
                + ((img[j + 1][i] as usize) << 1)
                + (img[j + 1][i + 1] as usize);
            res[j][i] = lookup[idx];
        }
    }
    res
}

fn infinify(img: &mut [Vec<bool>]) {
    let width = img[0].len();
    let height = img.len();
    (1..(height - 1)).for_each(|j| {
        img[j][0] = img[j][1];
        img[j][width - 1] = img[j][width - 2];
    });
    img[0] = img[1].clone();
    img[height - 1] = img[height - 2].clone();
}

fn pad(img: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut res = vec![];
    let width = img[0].len();
    res.push(vec![false; width + 2]);
    res.extend(img.iter().map(|l| {
        let mut newl = vec![false];
        newl.extend_from_slice(l);
        newl.push(false);
        newl
    }));
    res.push(vec![false; width + 2]);
    res
}

fn expand(img: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut res = vec![];
    let width = img[0].len();

    let mut newl = vec![img[0][0]];
    newl.extend(&img[0]);
    newl.push(img[0][width - 1]);
    res.push(newl);
    res.extend(img.iter().map(|l| {
        let mut newl = vec![l[0]];
        newl.extend(l);
        newl.push(l[width - 1]);
        newl
    }));

    let mut newl = vec![img[img.len() - 1][0]];
    newl.extend(&img[img.len() - 1]);
    newl.push(img[img.len() - 1][width - 1]);
    res.push(newl);
    res
}
