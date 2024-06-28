// run the server first

use sellershut_core::{
    categories::query_categories_client::QueryCategoriesClient, common::Paginate,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = QueryCategoriesClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(Paginate::default());

    let response = client.categories(request).await?;

    println!("response={response:?}");

    Ok(())
}
