use std::cmp::{max, min};

pub fn execute(code: &str) -> String {
    if code.is_empty() {
        return "*".to_string();
    }

    const DIRECTIONS: [[i64; 2]; 4] = [[0, 1], [1, -1], [0, -1], [1, 1]];

    let mut coordinates: Vec<[i64; 2]> = vec![[0, 0]];

    let mut cur: [i64; 2] = [0, 0];

    let mut chars = code.chars();
    let mut edge = [0; 4]; // left, right, top, bottom

    {
        let mut cmd = chars.next().unwrap();
        let mut times = 0i32;
        let mut direct_idx = 0i32;

        let mut do_cmd = |c: char, mut t: i32| {
            t = max(t, 1);
            match c {
                'F' => {
                    let d_idx = (((direct_idx % 4) + 4) % 4) as usize;

                    for _ in 0..t {
                        cur[DIRECTIONS[d_idx][0] as usize] += DIRECTIONS[d_idx][1];
                        coordinates.push(cur.clone());
                    }

                    edge[0] = min(edge[0], cur[0]);
                    edge[1] = max(edge[1], cur[0]);
                    edge[2] = min(edge[2], cur[1]);
                    edge[3] = max(edge[3], cur[1]);
                }
                'R' => direct_idx += t,
                'L' => direct_idx -= t,
                _ => {}
            }
        };

        chars.for_each(|ch| match ch {
            'F' | 'L' | 'R' => {
                do_cmd(cmd, times);
                cmd = ch;
                times = 0;
            }
            _ => {
                times = times * 10 + ch.to_digit(10).unwrap() as i32;
            }
        });

        do_cmd(cmd, times);
    }

    let width = ((edge[1] - edge[0]) + 1) as usize;
    let height = ((edge[3] - edge[2]) + 1) as usize;

    let mut char_coords: Vec<Vec<char>> = Vec::with_capacity(height);
    for _ in 0..height {
        char_coords.push(vec![' '; width]);
    }

    coordinates
        .into_iter()
        .map(|coord| [(coord[0] - edge[0]) as usize, (coord[1] - edge[2]) as usize])
        .for_each(|coord| {
            char_coords[height - 1 - coord[1]][coord[0]] = '*';
        });

    char_coords
        .into_iter()
        .map(|chars| chars.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\r\n")
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[cfg(test)]
    macro_rules! expect_equal {
        ($actual:expr, $expected:expr $(,)*) => {{
            let actual = $actual;
            let expected = $expected;
            assert_eq!(
                actual, expected,
                "\ngot:\n{}\n\nexpected:\n{}\n",
                actual, expected
            );
        }};
    }

    #[test]
    fn examples_in_description() {
        expect_equal!(execute(""), "*");
        expect_equal!(execute("FFFFF"), "******");
        expect_equal!(
            execute("FFFFFLFFFFFLFFFFFLFFFFFL"),
            "******\r\n*    *\r\n*    *\r\n*    *\r\n*    *\r\n******",
        );
        expect_equal!(
            execute("LFFFFFRFFFRFFFRFFFFFFF"),
            "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
        );
        expect_equal!(
            execute("LF5RF3RF3RF7"),
            "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
        );
    }

}
