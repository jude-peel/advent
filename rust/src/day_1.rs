use std::{fs, io};
pub struct LocationList {
    pub list_1: Vec<u32>,
    pub list_2: Vec<u32>,
}

impl LocationList {
    pub fn build() -> io::Result<Self> {
        let mut list_1: Vec<u32> = Vec::with_capacity(1000);
        let mut list_2: Vec<u32> = Vec::with_capacity(1000);

        fs::read_to_string("../data/day_1")?
            .lines()
            .map(|x| x.split_whitespace().collect::<Vec<_>>())
            .for_each(|s| {
                list_1.push(s[0].parse().unwrap());
                list_2.push(s[1].parse().unwrap());
            });

        list_1.sort();
        list_2.sort();

        Ok(Self { list_1, list_2 })
    }
    #[inline]
    pub fn get_total_distance(&self) -> u32 {
        self.list_1
            .iter()
            .enumerate()
            .map(|(i, dist)| dist.abs_diff(self.list_2[i]))
            .sum()
    }
    pub fn get_similarity(&self) -> u32 {
        let mut list_2_occurances = vec![0u32; *self.list_1.iter().max().unwrap() as usize + 1];

        self.list_2
            .iter()
            .fold(&mut list_2_occurances, |acc, &idx| {
                (*acc)[idx as usize] = (*acc)[idx as usize].saturating_add(1);
                acc
            });

        self.list_1
            .iter()
            .map(|x| *x * list_2_occurances[*x as usize])
            .sum()
    }
}
