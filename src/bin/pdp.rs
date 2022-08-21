use anyhow::anyhow;

use log::LevelFilter;
use pickup_delivery_problem::{solve, Input};
use simplelog::ConfigBuilder;

fn main() -> anyhow::Result<()> {
    simplelog::SimpleLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .set_target_level(LevelFilter::Off)
            .set_thread_level(LevelFilter::Off)
            .build(),
    )
    .unwrap();

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
