use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone)]
struct Graph {
    entries: HashMap<String, HashSet<String>>,
}

impl From<String> for Graph {
    fn from(value: String) -> Self {
        let entries = value
            .lines()
            .map(|value| {
                let parts: Vec<&str> = value.split(':').collect();
                let device = parts[0].trim().to_string();
                let mapping = parts[1].split_whitespace().map(|s| s.to_string()).collect();
                (device, mapping)
            })
            .collect();
        Graph { entries }
    }
}

impl Graph {
    fn degress(&self) -> HashMap<String, u64> {
        let mut degress: HashMap<String, u64> =
            self.all_nodes().into_iter().map(|device| (device.clone(), 0)).collect();
        for mapping in self.entries.values() {
            for map in mapping {
                *degress.get_mut(map).unwrap() = degress[map] + 1;
            }
        }
        degress
    }

    fn all_nodes(&self) -> HashSet<String> {
        self.entries
            .iter()
            .flat_map(|(node, children)| {
                std::iter::once(node.clone()).chain(children.iter().cloned())
            })
            .collect()
    }

    fn topological_sort(&self) -> Vec<String> {
        let mut queue: BTreeSet<String> = BTreeSet::new();
        let mut degress = self.degress();

        for (device, degree) in &degress {
            if *degree == 0 {
                queue.insert(device.clone());
            }
        }

        let mut ordered_nodes: Vec<String> = Vec::new();
        while !queue.is_empty() {
            let node = &queue.pop_first().unwrap();
            ordered_nodes.push(node.clone());
            if let Some(children) = self.entries.get(node) {
                for child in children {
                    let degree = degress.get_mut(child).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.insert(child.clone());
                    }
                }
            }
        }

        ordered_nodes
    }
}

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_11/{}", input_file))
        .expect("Failed to read input file");

    let graph = Graph::from(input);

    let result = match part {
        1 => part1(&graph),
        2 => part2(&graph),
        _ => {
            println!("Part {} not implemented for day 11", part);
            return;
        },
    };

    println!("Day 11 Part {}: {}", part, result);
}

fn count_paths(graph: &Graph, from: &str, to: &str) -> u64 {
    let ordered = graph.topological_sort();
    let mut paths: HashMap<String, u64> =
        HashMap::from_iter(graph.all_nodes().iter().map(|node| (node.clone(), 0)));

    *paths.get_mut(to).unwrap() = 1;
    for node in ordered.iter().rev() {
        let increment: u64 = if let Some(children) = graph.entries.get(node) {
            children.iter().filter_map(|child| paths.get(child)).sum()
        } else {
            0
        };
        if let Some(p) = paths.get_mut(node) {
            *p += increment;
        }
    }

    paths.get(from).copied().unwrap_or(0)
}

fn part1(graph: &Graph) -> u64 {
    count_paths(graph, "you", "out")
}

fn part2(graph: &Graph) -> u64 {
    // Paths visiting dac before fft
    let dac_then_fft = count_paths(graph, "svr", "dac")
        * count_paths(graph, "dac", "fft")
        * count_paths(graph, "fft", "out");

    // Paths visiting fft before dac
    let fft_then_dac = count_paths(graph, "svr", "fft")
        * count_paths(graph, "fft", "dac")
        * count_paths(graph, "dac", "out");

    dac_then_fft + fft_then_dac
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = r##"aaa: you hhh
    you: bbb ccc
    bbb: ddd eee
    ccc: ddd eee fff
    ddd: ggg
    eee: out
    fff: out
    ggg: out
    hhh: ccc fff iii
    iii: out"##;

    #[test]
    fn test_example() {
        let graph = Graph::from(TEST_INPUT.to_string());
        assert_eq!(part1(&graph), 5);
    }

    const TEST_INPUT_PART2: &str = r##"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"##;

    #[test]
    fn test_example_part2() {
        let graph = Graph::from(TEST_INPUT_PART2.to_string());
        assert_eq!(part2(&graph), 2);
    }
}
