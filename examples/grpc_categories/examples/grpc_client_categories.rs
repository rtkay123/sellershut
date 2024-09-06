// run the server first
use sellershut_core::{
    categories::query_categories_client::QueryCategoriesClient, common::pagination::Cursor,
};
use tonic::IntoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = QueryCategoriesClient::connect("http://[::1]:50051").await?;

    let cursor = Cursor::default();

    let response = client.categories(cursor.into_request()).await?;

    println!("response={response:?}");

    Ok(())
}
