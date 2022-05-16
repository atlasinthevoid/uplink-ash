use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

pub async fn start_website() -> tide::Result<()> {
    println!("Initializing website...");
    let mut app = tide::new();
    app.at("/").serve_file("src/html/index.html")?;
    app.at("/").serve_dir("src/html/")?;
    app.at("/orders/shoes").post(order_shoes);
    let result = app.listen("0.0.0.0:80").await;
    match result {
        Ok(_x) => {
            println!("Socket created successfully");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
