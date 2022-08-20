use anyhow::anyhow;

use pickup_delivery_problem::{solve, Input};

fn main() -> anyhow::Result<()> {
    simple_log::quick!("info");

    let input = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Missing input file name"))
        .and_then(|name| std::fs::read_to_string(name).map_err(|e| e.into()))?;

    let input = Input::try_from(input.as_str())?;
    let solution = solve(&input)?;

    println!("{}", solution.sort_by_time().to_string());
    println!("Total time: {}", solution.total_time());

    Ok(())
}
