use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // Non Functional Implementation
    let limit = limit as i32;
    let mut set: HashSet<i32> = HashSet::new();
    for i in factors {
        if i == &0 {
            break;
        }
        for j in 1.. {
            let res = (i * j) as i32;
            if res < limit {
                set.insert(res);
            } else {
                break;
            }
        }
    }
    let sum: i32 = set.into_iter().sum();
    sum as u32
}
#[cfg(test)]
#[test]
fn some_thing() {
    assert_eq!(23, sum_of_multiples(10, &[3, 5]))
}
