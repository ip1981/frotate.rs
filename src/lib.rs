extern crate chrono;

use std::collections::LinkedList;

use chrono::NaiveDate;
use chrono::Duration;

pub fn partition (f: &Fn(u32) -> i64, v: &Vec<i64>) -> LinkedList<LinkedList<i64>> {
    let mut term: LinkedList<i64> = LinkedList::new();
    let mut res: LinkedList<LinkedList<i64>> = LinkedList::new();
    let mut n : u32 = 1;
    let mut a : i64 = v[0];

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

    return res;
}

pub fn partition_days (f: &Fn(u32) -> i64, days: &Vec<NaiveDate>) -> LinkedList<LinkedList<NaiveDate>> {
    let day1 = days[0];
    let part;

    {
        let mut v: Vec<i64> = Vec::with_capacity(days.len());

        for &d in days.iter() {
            v.push((day1-d).num_days());
        }
        v.sort_unstable();
        v.dedup();

        part = partition(f, &v);
    }

    let res = part.into_iter().map(
        |l| l.into_iter().map(
            |d| day1 - Duration::days(d)).rev().collect()
        ).collect();

    return res
}
