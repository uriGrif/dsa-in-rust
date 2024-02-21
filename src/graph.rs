#![allow(dead_code)]

use std::collections::VecDeque;

use crate::heap::MinHeap;

struct DijkstraTableItem {
    dist: u64, // all weights must be non-negative (of course, this is not a good way of checking that)
    seen: bool,
    previous: Option<usize>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct DijkstraHeapItem {
    index: usize,
    dist: u64, // shortest distance from source
}

impl Ord for DijkstraHeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.dist < other.dist {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    }
}

impl PartialOrd for DijkstraHeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    to: usize,
    weight: i32,
}

impl Edge {
    fn new(to: usize, weight: i32) -> Edge {
        Edge {
            to,
            weight,
        }
    }
}

pub struct Graph {
    adj_list: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { adj_list: Vec::<Vec<Edge>>::new() }
    }

    pub fn vertex_amount(&self) -> usize {
        self.adj_list.len()
    }

    pub fn add_vertex(&mut self) {
        self.adj_list.push(Vec::<Edge>::new())
    }

    pub fn delete_vertex(&mut self, index: usize) {
        self.adj_list.remove(index);
        for v in self.adj_list.iter_mut() {
            v.retain(|e| e.to != index);
        }
    }

    pub fn add_edge(&mut self, source: usize, to: usize, weight: i32) {
        self.adj_list[source].push(Edge::new(to, weight))
    }

    pub fn bfs_path(&self, source: usize, dest: usize, path: &mut Vec<usize>) {
        if source >= self.vertex_amount() || dest >= self.vertex_amount() {
            return;
        }

        let mut seen: Vec<bool> = vec![false; self.vertex_amount()];
        let mut prev: Vec<Option<usize>> = vec![None; self.vertex_amount()];
        let mut queue: VecDeque<usize> = VecDeque::<usize>::new();

        queue.push_back(source);
        seen[source] = true;

        while queue.len() != 0 {
            let v: usize = queue.pop_front().unwrap();
            if v == dest {
                break;
            }
            for e in self.adj_list[v].iter() {
                if !seen[e.to] {
                    seen[e.to] = true;
                    prev[e.to] = Some(v);
                    queue.push_back(e.to);
                }
            }
        }

        if prev[dest].is_none() {
            return;
        }

        let mut p: Option<usize> = prev[dest];
        path.push(dest);
        while p.is_some() {
            path.push(p.unwrap());
            p = prev[p.unwrap()];
        }
        path.reverse();
    }

    pub fn dfs_path(&self, source: usize, dest: usize, path: &mut Vec<usize>) {
        if source >= self.vertex_amount() || dest >= self.vertex_amount() {
            return;
        }

        let mut seen: Vec<bool> = vec![false; self.vertex_amount()];

        self.walk(source, dest, path, &mut seen);
    }

    fn walk(&self, curr: usize, dest: usize, path: &mut Vec<usize>, seen: &mut Vec<bool>) -> bool {
        if curr == dest {
            path.push(curr);
            return true;
        }

        if seen[curr] {
            return false;
        }

        seen[curr] = true;
        path.push(curr);

        for e in self.adj_list[curr].iter() {
            if self.walk(e.to, dest, path, seen) {
                return true;
            }
        }

        path.pop();
        return false;
    }

    pub fn dijkstra_shortest_path(
        &self,
        source: usize,
        dest: usize,
        path: &mut Vec<usize>,
        cost: &mut u64
    ) {
        // https://www.youtube.com/watch?v=EFg3u_E6eHU
        // https://doc.rust-lang.org/std/collections/binary_heap/index.html

        let mut table: Vec<DijkstraTableItem> = vec![];
        let mut queue: MinHeap<DijkstraHeapItem> = MinHeap::<DijkstraHeapItem>::new();

        for i in 0..self.vertex_amount() {
            table.push(DijkstraTableItem {
                dist: if i == source {
                    0
                } else {
                    u64::MAX
                },
                seen: false,
                previous: None,
            });
        }

        queue.insert(DijkstraHeapItem { index: source, dist: 0 });

        while queue.len() > 0 {
            let current: usize = queue.pop().unwrap().index;

            table[current].seen = true;

            if current == dest {
                break;
            }

            for e in self.adj_list[current].iter() {
                if table[current].dist + (e.weight as u64) < table[e.to].dist {
                    table[e.to].dist = table[current].dist + (e.weight as u64);
                    table[e.to].previous = Some(current);
                }

                if !table[e.to].seen {
                    queue.insert(DijkstraHeapItem { index: e.to, dist: table[e.to].dist });
                }
            }
        }

        if !table[dest].seen {
            *cost = u64::MAX;
            return;
        }

        *cost = table[dest].dist;
        let mut p: Option<usize> = table[dest].previous;
        path.push(dest);
        while p.is_some() {
            path.push(p.unwrap());
            p = table[p.unwrap()].previous;
        }
        path.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_path_search() {
        let mut graph: Graph = Graph::new();

        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();

        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(2, 4, 1);
        graph.add_edge(3, 0, 1);
        graph.add_edge(4, 3, 1);
        graph.add_edge(4, 5, 1);
        graph.add_edge(5, 1, 1);
        graph.add_edge(5, 3, 1);

        let mut path1: Vec<usize> = vec![];
        let mut path2: Vec<usize> = vec![];
        let mut path3: Vec<usize> = vec![];

        graph.bfs_path(0, 5, &mut path1);
        graph.bfs_path(2, 1, &mut path2);
        graph.bfs_path(1, 5, &mut path3);

        assert_eq!(vec![0, 2, 4, 5], path1);
        assert_eq!(vec![2, 4, 5, 1], path2);
        assert_eq!(Vec::<usize>::new(), path3);
    }

    #[test]
    fn dfs_path_search() {
        let mut graph: Graph = Graph::new();

        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();

        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(2, 4, 1);
        graph.add_edge(3, 0, 1);
        graph.add_edge(4, 3, 1);
        graph.add_edge(4, 5, 1);
        graph.add_edge(5, 1, 1);
        graph.add_edge(5, 3, 1);

        let mut path1: Vec<usize> = vec![];
        let mut path2: Vec<usize> = vec![];
        let mut path3: Vec<usize> = vec![];

        graph.dfs_path(0, 5, &mut path1);
        graph.dfs_path(2, 1, &mut path2);
        graph.dfs_path(1, 5, &mut path3);

        assert_eq!(vec![0, 2, 4, 5], path1);
        assert_eq!(vec![2, 4, 3, 0, 1], path2);
        assert_eq!(Vec::<usize>::new(), path3);
    }

    #[test]
    fn shortest_path() {
        let mut graph: Graph = Graph::new();

        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();

        graph.add_edge(0, 1, 10);
        graph.add_edge(0, 2, 1);
        graph.add_edge(2, 4, 1);
        graph.add_edge(2, 1, 10);
        graph.add_edge(3, 0, 1);
        graph.add_edge(3, 1, 1);
        graph.add_edge(4, 3, 1);
        graph.add_edge(4, 5, 1);
        graph.add_edge(1, 5, 10);
        graph.add_edge(1, 0, 10);
        graph.add_edge(5, 3, 1);

        let mut path1: Vec<usize> = vec![];
        let mut path2: Vec<usize> = vec![];
        let mut path3: Vec<usize> = vec![];
        let mut cost1: u64 = u64::MAX;
        let mut cost2: u64 = u64::MAX;
        let mut cost3: u64 = u64::MAX;

        graph.dijkstra_shortest_path(0, 5, &mut path1, &mut cost1);
        graph.dijkstra_shortest_path(2, 1, &mut path2, &mut cost2);
        graph.dijkstra_shortest_path(1, 6, &mut path3, &mut cost3);

        assert_eq!(vec![0, 2, 4, 5], path1);
        assert_eq!(vec![2, 4, 3, 1], path2);
        assert_eq!(Vec::<usize>::new(), path3);
        assert_eq!(3, cost1);
        assert_eq!(3, cost2);
        assert_eq!(u64::MAX, cost3);
    }
}
