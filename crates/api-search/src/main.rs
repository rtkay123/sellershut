mod nlp;

use anyhow::Result;
use nlp::ZeroshotClassifier;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "nlp")]
    let (_handle, classifier) = ZeroshotClassifier::spawn();

    #[cfg(feature = "nlp")]
    {
        let texts = vec![
        "Who are you voting for in 2020?".into(),
        "The prime minister has announced a stimulus package which was widely criticized by the opposition.".into()
    ];
        let sentiments = classifier.predict(texts).await?;
        println!("Results: {sentiments:?}");
    }

    Ok(())
}
