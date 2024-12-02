use std::fs;

#[derive(Clone)]
pub struct Reports {
    list: Vec<Vec<u16>>,
}

impl Reports {
    pub fn build() -> std::io::Result<Self> {
        let input = fs::read_to_string("../data/day_2")?;
        let list_vec = input
            .lines()
            .map(|x| {
                x.split(" ")
                    .map(|y| y.parse::<u16>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self { list: list_vec })
    }
    pub fn safety_check(&self, report: &[u16]) -> bool {
        let increasing = report[0] < report[1];
        let mut _in_order = true;
        let mut min_max_change = [16, 0];
        for values in report.windows(2) {
            if values[0].abs_diff(values[1]) < min_max_change[0] {
                min_max_change[0] = values[0].abs_diff(values[1]);
            }

            if values[0].abs_diff(values[1]) > min_max_change[1] {
                min_max_change[1] = values[0].abs_diff(values[1]);
            }

            if increasing {
                if values[0] > values[1] {
                    _in_order = false;
                }
            } else if values[0] < values[1] {
                _in_order = false;
            }

            if values[0] == values[1] {
                _in_order = false;
            }
        }
        _in_order && min_max_change[0] >= 1 && min_max_change[1] <= 3
    }
    pub fn get_safe_sum(&mut self) -> u64 {
        let mut safe_sum = 0u64;
        for report in &self.list {
            if self.safety_check(report) {
                safe_sum += 1;
            }
        }

        safe_sum
    }
    pub fn get_fixable_sum(&mut self) -> u16 {
        let mut fixable_sum = 0;
        for unsafe_report in &self.list {
            let mut can_be_fixed = false;
            for i in 0..unsafe_report.len() {
                let mut with_ignored = unsafe_report.clone();
                with_ignored.remove(i);
                if self.safety_check(&with_ignored) {
                    can_be_fixed = true;
                }
            }
            if can_be_fixed {
                fixable_sum += 1;
            }
        }
        fixable_sum
    }
}
