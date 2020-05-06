extern crate csv

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.gstatic.com/ct/compliance/uptime.csv";
    let resp = reqwest::get(url)
        .await?;
    println!("{:#?}", resp);
    let mut rdr = csv::Rea
    Ok(())
}
