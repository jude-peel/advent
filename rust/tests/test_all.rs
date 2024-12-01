use std::error::Error;

use rust::day_1::LocationList;

#[test]
fn day_1() -> Result<(), Box<dyn Error>> {
    let list = LocationList::build()?;

    let distance = list.get_total_distance();
    let similarity = list.get_similarity();

    assert_eq!(distance, 1530215);
    assert_eq!(similarity, 26800609);
    Ok(())
}
