mod input;

fn main() {
    let splited: Vec<Vec<Range>> = input::VARIABLE
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|range| {
                    let mut split = range.split('-');
                    let start = split.next().unwrap().parse().unwrap();
                    let end = split.next().unwrap().parse().unwrap();
                    Range::new(start, end)
                })
                .collect()
        })
        .collect();
    let mut result = 0;
    for ranges in splited {
        if is_partly_contained(&ranges[0], &ranges[1]) {
            result += 1;
        }
    }
    println!("{:?}", result);
}

struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}

fn is_fully_contained(a: &Range, b: &Range) -> bool {
    if a.start <= b.start && a.end >= b.end {
        return true;
    }
    if b.start <= a.start && b.end >= a.end {
        return true;
    }
    false
}

fn is_partly_contained(a: &Range, b: &Range) -> bool {
    if a.start <= b.start && a.end >= b.start {
        return true;
    }
    if b.start <= a.start && b.end >= a.start {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_contained() {
        let a = Range::new(2, 8);
        let b = Range::new(3, 7);
        let result = is_fully_contained(&a, &b);
        assert_eq!(result, true);
    }
    #[test]
    fn full_contained_second() {
        let b = Range::new(4, 6);
        let a = Range::new(6, 6);
        let result = is_fully_contained(&a, &b);
        assert_eq!(result, true);
    }
    #[test]
    fn partialy_cotains() {
        let a = Range::new(5, 7);
        let b = Range::new(7, 9);
        let result = is_partly_contained(&a, &b);
        assert_eq!(result, true);
    }
    #[test]
    fn partialy_cotains_second() {
        let a = Range::new(2, 8);
        let b = Range::new(3, 7);
        let result = is_partly_contained(&a, &b);
        assert_eq!(result, true);
    }
    #[test]
    fn dosent_contain() {
        let a = Range::new(2, 4);
        let b = Range::new(6, 8);
        let result = is_partly_contained(&a, &b);
        assert_eq!(result, false);
    }
}
