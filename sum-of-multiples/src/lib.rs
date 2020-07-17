use std::collections::HashSet;
// Non Functional Implementation
// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//     let limit = limit as i32;
//     let mut set: HashSet<i32> = HashSet::new();
//     for i in factors {
//         for j in 1.. {
//             let res = (i * j) as i32;
//             if res < limit && res > 0 {
//                 // This is the Important Part
//                 set.insert(res); // Having a set removes duplicate values i.e 15 in case of 3 and 5 as Factors
//             } else {
//                 break;
//             }
//         }
//     }
//     let sum: i32 = set.into_iter().sum();
//     sum as u32
// }

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let sum: u32 = factors
        .iter()
        .filter(|x| **x != 0)
        .flat_map(|x| (1..limit).filter(|y| y % x == 0).collect::<Vec<u32>>())
        .collect::<HashSet<u32>>()
        .iter()
        .sum();
    sum
}
