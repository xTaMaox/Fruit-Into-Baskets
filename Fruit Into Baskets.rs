impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let (mut curr, mut prev) = (fruits[0], None);
        let (mut total_max, mut curr_max) = (0, 0);
        let mut left = 0;

        for (right, fruit) in fruits.iter().copied().enumerate() {
            if curr != fruit {
                if prev.map_or(false, |prev| prev != fruit) {
                    total_max = total_max.max(curr_max);
                    curr_max = (right - left) as i32;
                }
                prev = Some(curr);
                curr = fruit;
                left = right;
            }
            curr_max += 1;
        }

        total_max.max(curr_max)
    }
}