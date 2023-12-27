use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Mode {
    Add,
    Remove,
}

struct Lens<'a> {
    label: &'a str,
    mode: Mode,
    focal_length: u32,
    box_id: u32,
}

#[derive(Debug, PartialEq)]
struct ParseLensError;
impl<'a> From<&'a str> for Lens<'a> {
    fn from(s: &'a str) -> Lens<'a> {
        if let Some(index) = s.chars().position(|c| c == '-') {
            return Lens {
                label: &s[..index],
                mode: Mode::Remove,
                focal_length: 0,
                box_id: hash_value(&s[..index]),
            };
        }
        if let Some(index) = s.chars().position(|c| c == '=') {
            Lens {
                label: &s[..index],
                mode: Mode::Add,
                focal_length: s[index + 1..].parse::<u32>().unwrap(),
                box_id: hash_value(&s[..index]),
            }
        } else {
            panic!("whoops")
        }
    }
}

fn hash_value(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, char| ((acc + (char as u32)) * 17) % 256)
}

pub fn part_1(input: &str) -> u32 {
    input.split(',').map(hash_value).sum()
}

pub fn part_2(input: &str) -> u32 {
    let mut hash_map: HashMap<u32, Vec<Lens>> = HashMap::new();

    for lens in input.split(',').map(Lens::from) {
        let lenses = hash_map.entry(lens.box_id).or_default();

        if let Some(index) = lenses.iter().position(|l| l.label == lens.label) {
            match lens.mode {
                Mode::Remove => {
                    lenses.remove(index);
                }
                Mode::Add => lenses[index] = lens,
            }
        } else if lens.mode == Mode::Add {
            lenses.push(lens);
        }
    }

    let mut sum = 0;
    for (_, lenses) in hash_map {
        sum += lenses
            .iter()
            .enumerate()
            .fold(0, |acc, (slot_number, lens)| {
                acc + (lens.box_id + 1) * (slot_number as u32 + 1) * lens.focal_length
            })
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(part_1(input), 1320);
    }

    #[test]
    fn hash_value_test() {
        assert_eq!(hash_value("HASH"), 52);
        assert_eq!(hash_value("rn=1"), 30);
        assert_eq!(hash_value("rn"), 0);
    }

    #[test]
    fn part_2_test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(part_2(input), 145);
    }
}
