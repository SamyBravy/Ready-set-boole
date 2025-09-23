pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>>
{
    let mut powset = Vec::new();

    for i in 0..(1 << set.len())
	{
        let subset: Vec<i32> = set.iter()
            .enumerate()
            .filter_map(|(j, &val)| if (i >> j) & 1 == 1 { Some(val) } else { None })
            .collect();
        powset.push(subset);
    }

    powset
}
