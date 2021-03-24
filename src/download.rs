use std::fs;
use std::io::Write;

#[tokio::main]
pub async fn download(args: Vec<String>) -> Result<(), reqwest::Error> {
    // Downloads a file 

    // Makes a GET request and gets the data as bytes
    let data = reqwest::get(args[0].clone()).await?.bytes().await?;

    // Creates a file

    let mut file = fs::File::create(args[1].clone()).expect("Error occured while creating file");

    // Writes that file with the data stored in the data variable

    file.write_all(&data).expect("Error while downloading file");

    Ok(())
}

