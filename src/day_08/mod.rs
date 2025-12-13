use std::fs;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self { parent: (0..size).collect(), rank: vec![0; size] }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            match self.rank[ra].cmp(&self.rank[rb]) {
                std::cmp::Ordering::Less => self.parent[ra] = rb,
                std::cmp::Ordering::Greater => self.parent[rb] = ra,
                std::cmp::Ordering::Equal => {
                    self.parent[rb] = ra;
                    self.rank[ra] += 1;
                },
            }
        }
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = vec![0; self.parent.len()];
        for i in 0..self.parent.len() {
            sizes[self.find(i)] += 1;
        }
        sizes.into_iter().filter(|&s| s > 0).collect()
    }
}

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_08/{}", input_file))
        .expect("Failed to read input file");

    let points: Vec<[i64; 3]> = input
        .lines()
        .filter_map(|line| {
            let c: Vec<i64> = line.split(',').filter_map(|n| n.parse().ok()).collect();
            Some([*c.first()?, *c.get(1)?, *c.get(2)?])
        })
        .collect();

    let result = match part {
        1 => part1(&points, 1000),
        2 => part2(&input),
        _ => {
            println!("Part {} not implemented for day 8", part);
            return;
        },
    };

    println!("Day 8 Part {}: {}", part, result);
}

fn part1(points: &[[i64; 3]], n_iterations: usize) -> usize {
    let mut pairs: Vec<(i64, usize, usize)> = (0..points.len())
        .flat_map(|i| {
            (i + 1..points.len()).map(move |j| {
                let d: i64 = (0..3).map(|k| (points[i][k] - points[j][k]).pow(2)).sum();
                (d, i, j)
            })
        })
        .collect();

    pairs.sort_by_key(|p| p.0);

    let mut uf = UnionFind::new(points.len());
    for &(_, a, b) in pairs.iter().take(n_iterations) {
        uf.union(a, b);
    }
    let mut sizes = uf.component_sizes();
    sizes.sort_by(|a, b| b.cmp(a));
    dbg!(&sizes);
    let answer: usize = sizes.iter().take(3).product();
    answer
}

fn part2(_input: &str) -> usize {
    // TODO: Implement part 2
    println!("Part 2 not yet implemented");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let points = vec![
            [162, 817, 812],
            [57, 618, 57],
            [906, 360, 560],
            [592, 479, 940],
            [352, 342, 300],
            [466, 668, 158],
            [542, 29, 236],
            [431, 825, 988],
            [739, 650, 466],
            [52, 470, 668],
            [216, 146, 977],
            [819, 987, 18],
            [117, 168, 530],
            [805, 96, 715],
            [346, 949, 466],
            [970, 615, 88],
            [941, 993, 340],
            [862, 61, 35],
            [984, 92, 344],
            [425, 690, 689],
        ];
        assert_eq!(part1(&points, 10), 40);
    }
}
