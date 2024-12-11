use crate::graph::Graph;

fn do_test(graph: &Graph, max_len: usize, expected: &[&str]) {
    type Strings = std::collections::HashSet<String>;

    let actual = Strings::from_iter(graph.produce_strings(max_len));
    let expected = Strings::from_iter(expected.iter().cloned().map(str::to_owned));

    assert_eq!(actual, expected);
}

#[test]
fn test_0() {
    let graph = {
        let a = Graph::from('a');

        let b = Graph::from('b');
        let c = Graph::from('c');

        let bc = b.union(c).optional();

        let d = Graph::from('d').optional();

        a.concat(bc).concat(d)
    };

    do_test(&graph, 10, &["a", "ad", "ab", "abd", "ac", "acd"]);
}

#[test]
fn test_1() {
    let graph = {
        let a = Graph::from('a').zero_or_more();
        let b = Graph::from('b').zero_or_more();

        a.concat(b)
    };

    do_test(
        &graph,
        7,
        &[
            "", "b", "bb", "bbb", "bbbb", "bbbbb", "bbbbbb", "bbbbbbb", "a", "ab", "abb", "abbb",
            "abbbb", "abbbbb", "abbbbbb", "aa", "aab", "aabb", "aabbb", "aabbbb", "aabbbbb", "aaa",
            "aaab", "aaabb", "aaabbb", "aaabbbb", "aaaa", "aaaab", "aaaabb", "aaaabbb", "aaaaa",
            "aaaaab", "aaaaabb", "aaaaaa", "aaaaaab", "aaaaaaa",
        ],
    );
}
