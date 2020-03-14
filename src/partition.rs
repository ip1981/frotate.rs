extern crate chrono;

use std::collections::LinkedList;

use chrono::Duration;
use chrono::NaiveDate;

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;
    use std::collections::LinkedList;

    use super::partition;
    use super::partition_days;

    macro_rules! list {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_list = LinkedList::new();
                $(
                    temp_list.push_back($x);
                )*
                temp_list
            }
        };
    }

    #[test]
    fn all_const1() {
        let ins = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let outs = list![
            list![1],
            list![2],
            list![3],
            list![4],
            list![5],
            list![6],
            list![7],
            list![8],
            list![9]
        ];

        assert_eq!(partition(&|_n| 1, &ins), outs);
    }

    #[test]
    fn all_const2() {
        let ins = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let outs = list![list![1, 2], list![3, 4], list![5, 6], list![7, 8], list![9]];

        assert_eq!(partition(&|_n| 2, &ins), outs);
    }

    #[test]
    fn odds_const1() {
        let ins = vec![1, 3, 5, 7, 9, 11, 13, 15];
        let outs = list![
            list![1],
            list![],
            list![3],
            list![],
            list![5],
            list![],
            list![7],
            list![],
            list![9],
            list![],
            list![11],
            list![],
            list![13],
            list![],
            list![15]
        ];

        assert_eq!(partition(&|_n| 1, &ins), outs);
    }

    #[test]
    fn odds_const2() {
        let ins = vec![1, 3, 5, 7, 9, 11, 13, 15];
        let outs = list![
            list![1],
            list![3],
            list![5],
            list![7],
            list![9],
            list![11],
            list![13],
            list![15]
        ];

        assert_eq!(partition(&|_n| 2, &ins), outs);
    }

    fn exp2(n: u32) -> i64 {
        let i: i32 = n as i32 - 1;
        2_f32.powi(i).ceil() as i64
    }

    #[test]
    fn all_exp2() {
        let ins = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let outs = list![
            list![1],
            list![2, 3],
            list![4, 5, 6, 7],
            list![8, 9, 10, 11, 12, 13]
        ];

        assert_eq!(partition(&exp2, &ins), outs);
    }

    #[test]
    fn dates_exp2() {
        let d = NaiveDate::from_ymd;
        let ins = vec![
            d(2019, 08, 30),
            d(2019, 08, 31),
            d(2019, 09, 01),
            d(2019, 09, 02),
            d(2019, 09, 03),
            d(2019, 09, 04),
        ];
        let outs = list![
            list![d(2019, 09, 04)],
            list![d(2019, 09, 03), d(2019, 09, 02)],
            list![d(2019, 09, 01), d(2019, 08, 31), d(2019, 08, 30)]
        ];

        assert_eq!(partition_days(&exp2, &ins), outs);
    }
}

pub fn partition(f: &dyn Fn(u32) -> i64, v: &[i64]) -> LinkedList<LinkedList<i64>> {
    let mut term: LinkedList<i64> = LinkedList::new();
    let mut res: LinkedList<LinkedList<i64>> = LinkedList::new();
    let mut n: u32 = 1;
    let mut a: i64 = v[0];

    for &i in v.iter() {
        while i >= a + f(n) {
            res.push_back(term);
            term = LinkedList::new();
            a += f(n);
            n += 1;
        }
        term.push_back(i);
    }
    res.push_back(term);

    res
}

pub fn partition_days(
    f: &dyn Fn(u32) -> i64,
    days: &[NaiveDate],
) -> LinkedList<LinkedList<NaiveDate>> {
    let day1 = days[0];
    let part;

    {
        let mut v: Vec<i64> = Vec::with_capacity(days.len());

        for &d in days.iter() {
            v.push((day1 - d).num_days());
        }
        v.sort_unstable();
        v.dedup();

        part = partition(f, &v);
    }

    part.into_iter()
        .map(|l| l.into_iter().map(|d| day1 - Duration::days(d)).collect())
        .collect()
}
