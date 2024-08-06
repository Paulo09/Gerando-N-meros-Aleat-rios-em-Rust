use std::collections::HashSet;

fn intersecao_hashset(v1: &[i32], v2: &[i32]) -> Vec<i32> {
    let set1: HashSet<_> = v1.iter().cloned().collect();
    let set2: HashSet<_> = v2.iter().cloned().collect();

    set1.intersection(&set2).cloned().collect()
}
