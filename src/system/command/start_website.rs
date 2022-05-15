use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

pub async fn start_website() -> tide::Result<()> {
    println!("STARTING WEBSITE...");
    let mut app = tide::new();
    app.at("/").serve_file("src/html/index.html")?;
    app.at("/").serve_dir("src/html/")?;
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}