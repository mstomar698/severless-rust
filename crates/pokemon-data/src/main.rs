mod db;
mod pokemon_csv;
use db::*;
use pokemon_csv::*;

fn main() -> Result<(), csv::Error> {
    // To take csv file as input
    let mut rdr = csv::Reader::from_path("./crates/pokemon-data/pokemon.csv")?;
    for result in rdr.records() {
        let record: PokemonCsv = result?;
        println!("{:?}", record);
    }
    Ok(())
}
