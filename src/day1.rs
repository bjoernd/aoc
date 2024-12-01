use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day1
{
    list1: Vec<i32>,
    list2: Vec<i32>,
}

impl FromInput for Day1 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut l1 = vec![];
        let mut l2 = vec![];
        for l in _lines {
            let split :Vec<&str> = l.split("   ").collect();
            l1.push(i32::from_str_radix(split[0], 10).unwrap());
            l2.push(i32::from_str_radix(split[1], 10).unwrap());
        }
        Day1 { list1: l1, list2: l2 }
    }
}

impl DaySolution for Day1 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;
        
        let mut l1 = self.list1.clone();
        let mut l2 = self.list2.clone();

        l1.sort();
        l2.sort();

        for idx in 0..l1.len() {
            sum += (l1[idx] - l2[idx]).abs() as usize;
        }
        
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        
        for item in &self.list1 {
            let count = self.list2.iter().filter(|&n| *n == *item).count();
            sum += count * *item as usize;
        }

        sum.to_string()
    }
}
