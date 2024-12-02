use std::error::Error;

use rust::{day_1::LocationList, day_2::Reports};

#[test]
fn day_1() -> Result<(), Box<dyn Error>> {
    let list = LocationList::build()?;

    let distance = list.get_total_distance();
    let similarity = list.get_similarity();

    assert_eq!(distance, 1530215);
    assert_eq!(similarity, 26800609);
    Ok(())
}

#[test]
fn day_2() -> Result<(), Box<dyn Error>> {
    let mut reports = Reports::build()?;

    assert_eq!(reports.get_safe_sum(), 334);
    assert_eq!(reports.get_fixable_sum(), 400);
    Ok(())
}
