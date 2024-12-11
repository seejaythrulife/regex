use crate::graph::Graph;

fn do_test(graph: &Graph, cases: &[(&str, bool)]) {
    let (passed, fail_message) = cases.iter().copied().fold(
        (true, String::new()),
        |(passed, message), (string, expected)| {
            if graph.matches(string) != expected {
                (
                    false,
                    message + &format!("{string} (should match? {expected})\n"),
                )
            } else {
                (passed, message)
            }
        },
    );

    assert!(
        passed,
        "***** graph:\n{:?}\n\n***** failed cases:\n{:#?}",
        &graph, fail_message,
    );
}

#[test]
fn test_zero_or_more() {
    let graph = Graph::from('a').zero_or_more();

    do_test(
        &graph,
        &[
            ("", true),
            ("a", true),
            ("aa", true),
            ("aaa", true),
            ("aab", false),
            ("aba", false),
            ("baa", false),
        ],
    );
}

#[test]
fn test_concat() {
    let graph = {
        let l = Graph::from('l');
        let o = Graph::from('o');

        let lo = l.concat(o).zero_or_more();
        let l = Graph::from('l').zero_or_more();

        lo.concat(l)
    };

    do_test(
        &graph,
        &[
            ("", true),
            ("lo", true),
            ("lol", true),
            ("lolll", true),
            ("lolol", true),
            ("looool", false),
            ("olll", false),
            ("lolololll", true),
        ],
    );
}

#[test]
fn test_one_or_more_0() {
    let graph = {
        let h = Graph::from('h');
        let i = Graph::from('i').one_or_more();

        h.concat(i)
    };

    do_test(
        &graph,
        &[("hi", true), ("hihi", false), ("hii", true), ("h", false)],
    );
}

#[test]
fn test_one_or_more_1() {
    let graph = {
        let h = Graph::from('h');
        let i = Graph::from('i');

        h.concat(i).one_or_more()
    };

    do_test(
        &graph,
        &[("hi", true), ("hihi", true), ("hii", false), ("", false)],
    );
}

#[test]
fn test_union_0() {
    let graph = {
        let h = Graph::from('h');
        let i = Graph::from('i');

        let hi = h.concat(i);

        let l = Graph::from('l');
        let o = Graph::from('o');

        let lo = l.concat(o);

        hi.union(lo)
    };

    do_test(&graph, &[("hi", true), ("lo", true), ("hilo", false)]);
}

#[test]
fn test_union_1() {
    let graph = {
        let i = Graph::from('i');
        let a = Graph::from('a');

        let h = Graph::from('h');
        let ia = i.union(a);

        h.concat(ia)
    };

    do_test(
        &graph,
        &[("hi", true), ("ha", true), ("hia", false), ("ho", false)],
    );
}

#[test]
fn test_optional_0() {
    let graph = {
        let h = Graph::from('h');
        let i = Graph::from('i').optional();

        h.concat(i)
    };

    do_test(
        &graph,
        &[("h", true), ("hi", true), ("i", false), ("ho", false)],
    );
}

#[test]
fn test_optional_1() {
    let graph = {
        let h = Graph::from('h');
        let i = Graph::from('i');

        h.concat(i).optional()
    };

    do_test(
        &graph,
        &[("hi", true), ("h", false), ("i", false), ("ho", false)],
    );
}
