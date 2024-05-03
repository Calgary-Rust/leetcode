struct Solution;

impl Solution {
    pub fn validate_input(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) {
        if start_time.len() != end_time.len() {
            panic!("start_time and end_time must have the same length")
        }

        match start_time.len() {
            0..=100 => (),
            _ => panic!("start_time and end_time must have a length between 1 and 100"),
        }

        for i in 0..start_time.len() {
            if !(start_time[i] <= end_time[i]) {
                panic!("start_time must be less than or equal to end_time")
            }
        }

        match query_time {
            1..=1000 => (),
            _ => panic!("query_time must be between 1 and 1000"),
        }
    }
}

impl Solution {
    // Opinion: it's okay but the indexation with square brackets in Rust is checked at runtime
    // Note for newbies: the last expression is returned without the need to use the `return` keyword
    pub fn busy_student0(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut count = 0;
        for i in 0..start_time.len() {
            if start_time[i] <= query_time && query_time <= end_time[i] {
                count += 1;
            }
        }

        count
    }

    // Opinion: just good enough but the creation of the itereator over pairs is less obvious
    pub fn busy_student1(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .copied()
            .zip(end_time.iter().copied())
            .filter(|(start, end)| *start <= query_time && query_time <= *end)
            .count() as i32
    }

    // Opinion: a bit too verbose
    pub fn busy_student2(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let start_time_iter = start_time.iter().copied();
        let end_time_iter = end_time.iter().copied();

        let time_interval_iter = start_time_iter.zip(end_time_iter);

        let matching_time_intervals =
            time_interval_iter.filter(|(start, end)| *start <= query_time && query_time <= *end);

        matching_time_intervals.count() as i32
    }

    // Opinion: right balance between expressiveness and verbosity
    pub fn busy_student3(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let time_intervals_iter = start_time.iter().copied().zip(end_time.iter().copied());

        time_intervals_iter
            .filter(|(start, end)| *start <= query_time && query_time <= *end)
            .count() as i32
    }

    pub const SOLUTIONS: &'static [fn(Vec<i32>, Vec<i32>, i32) -> i32] = &[
        Solution::busy_student0,
        Solution::busy_student1,
        Solution::busy_student2,
        Solution::busy_student3,
    ];
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn leetcode_example1() {
        let start_time = vec![1, 2, 3];
        let end_time = vec![3, 2, 7];
        let query_time = 4;

        let expected_result = 1;

        for solution in Solution::SOLUTIONS {
            let actual_result = solution(start_time.clone(), end_time.clone(), query_time);
            assert_eq!(actual_result, expected_result);
        }
    }

    fn leetcode_example2() {
        let start_time = vec![4];
        let end_time = vec![4];
        let query_time = 4;

        let expected_result = 1;

        for solution in Solution::SOLUTIONS {
            let actual_result = solution(start_time.clone(), end_time.clone(), query_time);
            assert_eq!(actual_result, expected_result);
        }
    }
}
