/// 辞書順で出力するトポロジカルソート
pub type Graph = [Vec<usize>];
pub fn topological_sort(graph: &Graph, in_degree: &mut [usize]) -> Vec<usize> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut sorted_vertices = vec![];
    let mut heap = BinaryHeap::new();

    for (i, indeg) in in_degree.iter().enumerate() {
        if *indeg == 0 {
            heap.push(Reverse(i));
        }
    }
    while let Some(Reverse(v)) = heap.pop() {
        sorted_vertices.push(v);
        for &i in graph[v].iter() {
            in_degree[i] -= 1;
            if in_degree[i] == 0 {
                heap.push(Reverse(i))
            }
        }
    }

    sorted_vertices
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn topological_sort_test() {
        let mut graph = vec![vec![]; 9];
        graph[1].push(6);
        graph[2].push(5);
        graph[3].push(1);
        graph[3].push(2);
        graph[4].push(1);
        graph[4].push(6);
        graph[5].push(1);
        graph[6].push(7);
        graph[7].push(8);

        let mut in_degree = vec![0; 9];
        for i in graph.iter() {
            for j in i.iter() {
                in_degree[*j] += 1;
            }
        }

        assert_eq!(
            topological_sort(&graph, &mut in_degree),
            [0, 3, 2, 4, 5, 1, 6, 7, 8]
        );
    }
}
