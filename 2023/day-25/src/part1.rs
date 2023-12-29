#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use petgraph::prelude::*;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

/// day 25 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part1(input: &str) -> String {
    let (_, initial_map) = parse_input(input).expect("AOC should have valid input");

    let all_node_strings = initial_map
        .iter()
        .flat_map(|(key, nodes)| {
            let mut nodes = nodes.clone();
            nodes.push(key);
            nodes
        })
        .unique()
        .collect::<Vec<_>>();

    let mut graph = UnGraph::<&str, u32>::default();
    let node_map = all_node_strings
        .iter()
        .map(|&id| (id, graph.add_node(id)))
        .collect::<HashMap<_, _>>();

    for (&src_id, dest_nodes) in &initial_map {
        for &dest_id in dest_nodes {
            graph.add_edge(node_map[src_id], node_map[dest_id], 1);
        }
    }
    let total_nodes = all_node_strings.len();
    let min_cut_res: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_mincut, partition) = min_cut_res.unwrap().unwrap();
    let partition_len = partition.len();
    let rest_len = total_nodes - partition_len;
    (partition_len * rest_len).to_string()
}

fn parse_input(input: &str) -> IResult<&str, HashMap<&str, Vec<&str>>> {
    let (input, nodes_as_array) = separated_list1(
        complete::line_ending,
        separated_pair(
            complete::alpha1,
            tuple((tag(":"), complete::space0)),
            separated_list1(complete::space1, complete::alpha1),
        ),
    )(input)?;
    Ok((input, nodes_as_array.into_iter().collect()))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "54".to_string());
    }
}
