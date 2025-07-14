use std::env;
use dotenvy::dotenv;

fn main() {
    dotenv().expect(".env does not exist");

    let mongo_uri = env::var("MONGO_URI").expect("Does not exist in .env");

    println!("{}", mongo_uri);
}