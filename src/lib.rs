/// Give (n choose m) permutations in the range 0..n
pub fn n_choose_m(n: usize, m: usize) -> Vec<Vec<usize>> {
    let m_minus_one = match m.checked_sub(1) {
        Some(mmo) => mmo,
        None => return vec![vec![]],
    };
    
    let mut out = vec![];
    
    for i in m_minus_one..n {
        for mut sub in n_choose_m(i, m_minus_one) {
            sub.push(i);
            out.push(sub);
        }
    }
    out
}

// TODO: make an iterator!
