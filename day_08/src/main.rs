use std::{cmp::{self, Reverse}, collections::HashSet, ops::IndexMut, str::FromStr};
use anyhow::{self, Context};
use xmas::{display_result, point3d::{ParsePoint3DError, Point3D}};

fn main() -> anyhow::Result<()> {
    part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");

    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;
    let junctions = parse_junctions(&input)?;
    let mut connections = get_possible_connections(&junctions);

    sort_connections(&junctions, &mut connections);

    let mut circuits = Vec::<HashSet<usize>>::new();
    for conn in connections.iter().take(1000) {
        merge_connections(&mut circuits, conn.0, conn.1);
    }
    circuits.sort_by_key(|c| Reverse(c.len()));

    let result = circuits
        .iter()
        .take(3)
        .map(|c| {
            // println!("{}", c.len());
            c.len()
        })
        .reduce(|a, b| a * b)
        .unwrap();

    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");

    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;
    let junctions = parse_junctions(&input)?;
    let mut connections = get_possible_connections(&junctions);

    sort_connections(&junctions, &mut connections);

    let mut circuits = Vec::<HashSet<usize>>::new();

    let mut i = 0;
    while circuits.get(0).is_none_or(|c| c.len() < junctions.len()) {
        let conn = connections[i];
        merge_connections(&mut circuits, conn.0, conn.1);

        i += 1;
    }

    let last_conn = connections[i - 1];
    let result = junctions[last_conn.0].0 * junctions[last_conn.1].0;

    display_result(&result);

    Ok(())
}

fn parse_junctions(input: &str) -> Result<Vec<Point3D>, ParsePoint3DError> {
    input
        .lines()
        .map(|s| Point3D::from_str(s))
        .collect::<Result<Vec<_>, _>>()
}

fn get_possible_connections(junctions: &[Point3D]) -> Vec<(usize, usize)> {
    let mut connections = Vec::new();
    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            connections.push((i, j));
        }
    }
    connections
}

fn sort_connections(junctions: &[Point3D], connections: &mut [(usize, usize)]) {
    connections.sort_by_cached_key(|c| {
        let diff = junctions[c.0] - junctions[c.1];
        diff.sqr_magnitude()
    });
}

fn merge_connections(circuits: &mut Vec<HashSet<usize>>, a: usize, b: usize) {
    let circuit_a = circuits.iter().position(|c| c.contains(&a));
    let circuit_b = circuits.iter().position(|c| c.contains(&b));

    match (circuit_a, circuit_b) {
        (Some(i), Some(j)) => {
            if i == j {
                return;
            }

            let lower_i = cmp::min(i, j);
            let upper_i = cmp::max(i, j);

            let removed = circuits.remove(upper_i);
            circuits.index_mut(lower_i).extend(removed.iter());
        },
        (Some(i), None) => {
            circuits.index_mut(i).insert(b);
        },
        (None, Some(j)) => {
            circuits.index_mut(j).insert(a);
        },
        (None, None) => {
            let mut circuit = HashSet::new();
            circuit.insert(a);
            circuit.insert(b);
            circuits.push(circuit);
        }
    };
}
