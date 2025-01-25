use reqwest::Client;

pub struct Token(pub String);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

pub async fn get_customer_groups(token: &Token, client: &Client) -> Result<(), Error> {
    let res = client.get("https://aptusbookingservice.afbostader.se/bookingservice.svc/GetCustomerGroups").query(&[("Token", &token.0)]).send().await?.text().await?;

    println!("{res}");

    Ok(())
}