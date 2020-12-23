use crate::prelude::*;

pub fn day10() -> Result<()> {
    let data = INPUT;
    let mut data: Vec<usize> = data.lines().filter_map(|x| Some(x.parse().ok()?)).collect();
    data.sort_unstable();
    part_1(&data);
    part_2(&data);
    Ok(())
}

fn part_1(data: &[usize]) -> usize {
    // start with (1,1) in acc, as the wall-socket is a 1, and our device is a 3
    let (ones, threes) =
        data.windows(2)
            .fold((1, 1), |(ones, threes), val| match val[1] - val[0] {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => unreachable!(),
            });
    println!("2020 10.1 -> {}", ones * threes);
    ones * threes
}

fn part_2(data: &[usize]) -> usize {
    let result = data.windows(2).collect::<Vec<_>>();
    let split = result.split(|n| n[1] - n[0] == 3).collect::<Vec<_>>();
    let product = 2 * split
        .iter()
        .map(|n| match n.len() {
            4 => 7,
            3 => 4,
            2 => 2,
            _ => 1,
        })
        .product::<usize>();
    println!("2020 10.2 -> {}", product);
    product
}

const INPUT: &str = "17
110
146
144
70
57
124
121
134
12
135
120
19
92
6
103
46
56
93
65
14
31
63
41
131
60
73
83
71
37
85
79
13
7
109
24
94
2
30
3
27
77
91
106
123
128
35
26
112
55
97
21
100
88
113
117
25
82
129
66
11
116
64
78
38
99
130
84
98
72
50
36
54
8
34
20
127
1
137
143
76
69
111
136
53
43
140
145
49
122
18
42
";

#[allow(dead_code)]
const SAMPLE: &str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

#[allow(dead_code)]
const SAMPLE2: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
