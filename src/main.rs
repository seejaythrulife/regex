use regex::graph::Graph;
use std::time::Instant;

/// the integer type used to count strings (for easy upgrading in case we want to count languages large enough to overflow).
type Count = u64;

fn main() {
    let default_options = StringCountOptions {
        max_string_length: 0,
        strings_to_print: None,
        seconds_between_updates: 1,
    };

    // (lo)+l+
    let lol = {
        // (lo)+
        let lo = {
            // l
            let l = Graph::from('l');

            // o
            let o = Graph::from('o');

            l.concat(o).one_or_more()
        };

        // l+
        let l = Graph::from('l').one_or_more();

        lo.concat(l)
    };

    print_string_count(
        &lol,
        StringCountOptions {
            max_string_length: 571,
            ..default_options
        },
    );

    // hah+a*
    let haha = {
        // h
        let h1 = Graph::from('h');

        // a
        let a1 = Graph::from('a');

        // h+
        let h2 = Graph::from('h').one_or_more();

        // a*
        let a2 = Graph::from('a').zero_or_more();

        h1.concat_many([a1, h2, a2])
    };

    print_string_count(
        &haha,
        StringCountOptions {
            max_string_length: 272,
            ..default_options
        },
    );

    // ((ab?|c?d)+|e+)+
    let abcde = {
        // (ab?|c?d)+
        let abcd = {
            // ab?
            let ab = {
                // a
                let a = Graph::from('a');

                // b?
                let b = Graph::from('b').optional();

                a.concat(b)
            };

            // c?d
            let cd = {
                // c?
                let c = Graph::from('c').optional();

                // d
                let d = Graph::from('d');

                c.concat(d)
            };

            ab.union(cd).one_or_more()
        };

        // e+
        let e = Graph::from('e').one_or_more();

        abcd.union(e).one_or_more()
    };

    print_string_count(
        &abcde,
        StringCountOptions {
            max_string_length: 6,
            ..default_options
        },
    );
}

#[derive(Clone)]
struct StringCountOptions {
    /// The maximum length of which to generate strings.
    max_string_length: usize,

    /// The number of strings to print while counting (`Some(0)` prints all strings, `None` prints no strings).
    strings_to_print: Option<usize>,

    /// The number of seconds between updates (0 prints no updates).
    seconds_between_updates: u64,
}

fn print_string_count(graph: &Graph, options: StringCountOptions) {
    let StringCountOptions {
        max_string_length,
        strings_to_print,
        seconds_between_updates,
    } = options;

    println!("========== {}", graph.label());

    let count = graph
        // produce the strings.
        .produce_strings(max_string_length)
        // get the index of each string.
        .enumerate()
        // print strings/updates (as per parameters).
        .inspect({
            let start_time = Instant::now();
            let mut last_update = 0;

            move |(i, string)| {
                // print update.
                if seconds_between_updates > 0 {
                    let next_update = start_time.elapsed().as_secs() / seconds_between_updates;

                    if next_update > last_update {
                        println!("busy for {} seconds", next_update * seconds_between_updates);

                        last_update = next_update;
                    }
                }

                // print string.
                if match strings_to_print {
                    Some(0) => true,
                    Some(strings_to_print) => *i < strings_to_print,
                    None => false,
                } {
                    println!("{i} : \"{string}\"");
                }
            }
        })
        // get the count.
        .fold(Some(0 as Count), |count, _| {
            count.map(|count| count.checked_add(1)).flatten()
        });

    match count {
        Some(count) => println!("string count: {}", count),
        None => println!("overflowed! string count: more than {}", Count::MAX),
    };
    println!();
}
