fn main() {
    println!("--- Day 11: Dumbo Octopus ---");

    // let input = include_str!("../sample.txt");
    let input = include_str!("../input.txt");

    let mut map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();

    // dbg!(&map);

    let mut flashes = 0;

    for step in 0.. {
        for row in map.iter_mut() {
            for energy in row.iter_mut() {
                *energy += 1;
            }
        }

        let mut flashed = [[false; 10]; 10];
        let mut done = false;

        while !done {
            done = true;

            for y in 0..10 {
                for x in 0..10 {
                    if map[y][x] > 9 && !flashed[y][x] {
                        flashed[y][x] = true;
                        done = false;

                        for ny in (y.saturating_sub(1))..=(y.saturating_add(1)) {
                            for nx in (x.saturating_sub(1))..=(x.saturating_add(1)) {
                                if (ny, nx) == (y, x) {
                                    continue;
                                }

                                if let Some(nrow) = map.get_mut(ny) {
                                    if let Some(n) = nrow.get_mut(nx) {
                                        *n += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for row in map.iter_mut() {
            for energy in row.iter_mut() {
                if *energy > 9 {
                    *energy = 0;
                }
            }
        }

        // dbg!(flashed);
        // dbg!(&map);

        flashes += flashed
            .iter()
            .map(|row| row.iter().filter(|&&x| x).count())
            .sum::<usize>();

        if step + 1 == 100 {
            println!("Flashes after 100 steps: {}", flashes);
        }

        if flashed.iter().map(|row| row.iter().all(|&x| x)).all(|row| row) {
            println!("First synchronized flash at step #{}", step + 1);
            break;
        }
    }
}
