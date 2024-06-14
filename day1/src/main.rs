mod part1 {
    use std::fs::File;
    use std::io::Read;
    pub fn solution() {
        let mut file = File::open("input").unwrap();
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut sum = 0;
        for line in contents.split_whitespace() {
            match line.parse::<i32>() {
                Ok(n) => {
                    sum += n;
                }
                Err(_) => {
                    println!("Can not parse the input value");
                }
            }
        }

        println!("Sum of value: {}", sum);
    }
}

mod part2 {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::Read;
    pub fn solution() {
        let mut file: File = File::open("input").unwrap();
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap();

        // using this function as loop
        let mut sum = 0;
        let mut set: HashSet<i32> = HashSet::new();
        set.insert(sum);
        loop {
            for line in contents.split_whitespace() {
                match line.parse::<i32>() {
                    Ok(n) => {
                        sum += n;
                        if set.contains(&sum) {
                            println!("First duplicate value: {}", sum);
                            return;
                        }
                        set.insert(sum);
                    }
                    Err(_) => {
                        println!("Can not parse the input value");
                    }
                }
            }
        }
    }
}

fn main() {
    part1::solution();
    part2::solution();
}
