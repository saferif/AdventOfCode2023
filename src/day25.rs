use crate::error::AoCError;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub(crate) fn part1(input: String) -> Result<String, AoCError> {
    let mut graph = BTreeMap::<&str, BTreeMap<&str, usize>>::new();
    input
        .trim()
        .lines()
        .map(|line| {
            let (from, to) = line
                .split_once(':')
                .ok_or(AoCError::from("invalid format"))?;
            to.trim().split(' ').for_each(|to| {
                graph.entry(from).or_default().insert(to, 1);
                graph.entry(to).or_default().insert(from, 1);
            });
            Ok(())
        })
        .collect::<Result<Vec<_>, AoCError>>()?;
    let nodes = graph.keys().copied().collect::<Vec<_>>();

    let len = nodes
        .iter()
        .enumerate()
        .find_map(|(i, &s)| {
            nodes[i + 1..].iter().copied().find_map(|t| {
                let mut graph = graph.clone();
                let (flow, len) = edmonds_karp(&mut graph, s, t);
                (flow == 3).then_some(len)
            })
        })
        .ok_or(AoCError::from("no answer"))?;

    Ok((len * (nodes.len() - len)).to_string())
}

pub(crate) fn part2(_: String) -> Result<String, AoCError> {
    Ok("🎅❄️🥳🎉☃️".to_string())
}

fn bfs<'a>(
    graph: &BTreeMap<&'a str, BTreeMap<&'a str, usize>>,
    start: &'a str,
) -> BTreeMap<&'a str, &'a str> {
    let mut came_from = BTreeMap::from([(start, "")]);
    let mut queue = VecDeque::from([start]);
    while let Some(current) = queue.pop_front() {
        graph
            .get(current)
            .into_iter()
            .flatten()
            .for_each(|(&child, &capacity)| {
                if !came_from.contains_key(child) && capacity > 0 {
                    queue.push_back(child);
                    came_from.insert(child, current);
                }
            });
    }
    came_from
}

fn edmonds_karp<'a>(
    graph: &mut BTreeMap<&'a str, BTreeMap<&'a str, usize>>,
    source: &'a str,
    sink: &str,
) -> (usize, usize) {
    let mut max_flow = 0;
    let len = loop {
        let came_from = bfs(graph, source);
        if !came_from.contains_key(sink) {
            break came_from.len();
        }

        let mut path_flow = usize::MAX;
        let mut current = sink;
        while current != source {
            let parent = came_from.get(current).expect("must have came from source");
            let capacity = graph
                .get(parent)
                .and_then(|edges| edges.get(current))
                .expect("must have used an existing edge");
            current = parent;
            path_flow = path_flow.min(*capacity)
        }

        max_flow += path_flow;

        current = sink;
        while current != source {
            let parent = came_from.get(current).expect("must have came from source");
            graph
                .get_mut(parent)
                .and_then(|edges| edges.get_mut(current))
                .into_iter()
                .for_each(|capacity| *capacity -= path_flow);
            graph
                .get_mut(current)
                .and_then(|edges| edges.get_mut(parent))
                .into_iter()
                .for_each(|capacity| *capacity += path_flow);
            current = parent;
        }
    };
    (max_flow, len)
}
