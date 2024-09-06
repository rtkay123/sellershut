use sellershut_core::categories::Category;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let category = Category::default();

    let category_json = serde_json::to_string(&category)?;

    println!("{category_json}");
    Ok(())
}

