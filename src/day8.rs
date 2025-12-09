use crate::solution::Day;
use rayon::prelude::*;

/// x,y,z coordinates
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
struct Coord3D {
    x: i64,
    y: i64,
    z: i64,
}

impl std::fmt::Display for Coord3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl std::str::FromStr for Coord3D {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(Self::new(
            parts.next().unwrap().parse()?,
            parts.next().unwrap().parse()?,
            parts.next().unwrap().parse()?,
        ))
    }
}

impl Coord3D {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).pow(2) as f64
            + (self.y - other.y).pow(2) as f64
            + (self.z - other.z).pow(2) as f64)
            .sqrt()
    }

    fn distance_squared(&self, other: &Self) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn parse_input(input: &str) -> Vec<Coord3D> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

struct DisjointSet {
    /// Parent IDX of element N
    parent: Vec<usize>,
    /// Size of set that element N belongs to. Only root numbers are meaningful.
    size: Vec<usize>,
}

impl DisjointSet {
    /// Preallocate N elements
    fn new(size: usize) -> Self {
        Self {
            parent: vec![0; size],
            size: vec![0; size],
        }
    }

    /// Add a new element into a new set containing only the new element
    fn make_set(&mut self, x: usize) {
        self.parent[x] = x;
        self.size[x] = 1;
    }

    /// Find the root element (representative) of element x
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            // flatten
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    /// Replaces the set containing x and the set containing y with their union
    /// Returns true if the union was successful (x,y were not already in the same set), false otherwise
    fn union(&mut self, x: usize, y: usize) -> bool {
        // find roots of both
        let mut x = self.find(x);
        let mut y = self.find(y);

        // nothing to do, they already belong to the same set
        if x == y {
            return false;
        }

        // swap so x has >= descendants than y
        if self.size[x] < self.size[y] {
            (x, y) = (y, x);
        }

        // Make x the new root (y now points to x)
        self.parent[y] = x;
        self.size[x] += self.size[y];

        true
    }

    /// Find indexes of all roots, i.e. items where their parent is themselves
    fn roots(&self) -> impl Iterator<Item = usize> {
        self.parent
            .iter()
            .enumerate()
            .filter_map(|(idx, &x)| if x == idx { Some(idx) } else { None })
    }

    fn get_size(&self, x: usize) -> usize {
        self.size[x]
    }

    fn sets_count(&self) -> usize {
        self.parent
            .iter()
            .enumerate()
            .filter(|(idx, x)| idx == *x)
            .count()
    }

    fn circuit_sizes(&self) -> Vec<usize> {
        let mut sizes: Vec<usize> = self
            .parent
            .iter()
            .enumerate()
            .filter(|(idx, x)| *idx == **x && self.size[*idx] > 1)
            .map(|(idx, _)| self.size[idx])
            .collect();
        sizes.sort_by(|a, b| b.cmp(a));
        sizes
    }
}

/// Returns (DisjointSet with all elements initialized, pairs sorted by distance)
fn prepare_circuits(boxes: &[Coord3D]) -> (DisjointSet, Vec<(usize, usize, i64)>) {
    let mut set = DisjointSet::new(boxes.len());
    (0..boxes.len()).for_each(|idx| set.make_set(idx));
    let mut pairs: Vec<_> = (0..boxes.len())
        .flat_map(|i| {
            ((i + 1)..boxes.len()).map(move |j| (i, j, boxes[i].distance_squared(&boxes[j])))
        })
        .collect();
    // sorting takes a while, parallelizing it speeds it up
    pairs.par_sort_unstable_by_key(|&(_, _, d)| d);
    (set, pairs)
}

fn solve(boxes: &[Coord3D], iterations: usize) -> u64 {
    let (mut set, pairs) = prepare_circuits(boxes);

    // then for each iteration (or however many boxes we have)
    for &(x_idx, y_idx, _) in pairs.iter().take(iterations) {
        // connect x and y to same circuit
        set.union(x_idx, y_idx);
    }

    // after we're done, find 3 largest circuits;
    // for each root, gets its size
    set.circuit_sizes()
        .iter()
        .take(3)
        .map(|&x| x as u64)
        .product()
}

fn solve_all(boxes: &[Coord3D]) -> i64 {
    let (mut set, pairs) = prepare_circuits(boxes);

    // stack of indexes that we connected
    let mut last_connected = (0, 0);

    // do it for all pairs
    for iter_idx in 0..pairs.len() {
        let (x_idx, y_idx, _) = pairs[iter_idx];
        // connect x and y to same circuit
        if set.union(x_idx, y_idx) {
            last_connected = (x_idx, y_idx);
        }
    }

    // get last 2 boxes we connected and multiply their X coords
    boxes[last_connected.0].x * boxes[last_connected.1].x
}

pub struct Solution;

impl Day for Solution {
    fn part1(input: &str) -> crate::solution::Solution {
        let coords = parse_input(input);
        solve(&coords, 1000).into()
    }

    fn part2(input: &str) -> crate::solution::Solution {
        let coords = parse_input(input);
        solve_all(&coords).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_solve() {
        let coords = parse_input(TEST_INPUT);
        let result = solve(&coords, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_solve_all() {
        let coords = parse_input(TEST_INPUT);
        let result = solve_all(&coords);
        assert_eq!(result, 25272);
    }
}
