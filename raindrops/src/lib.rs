pub fn raindrops(n: u32) -> String {
    let mut rd = String::new();
    let is_pling = |n| n % 3 == 0;
    let is_plang = |n| n % 5 == 0;
    let is_plong = |n| n % 7 == 0;
    if is_pling(n) {
        rd.push_str("Pling");
    }
    if is_plang(n) {
        rd.push_str("Plang");
    }
    if is_plong(n) {
        rd.push_str("Plong");
    }
    if rd.is_empty() {
        rd.push_str(&n.to_string());
    }

    rd
}
