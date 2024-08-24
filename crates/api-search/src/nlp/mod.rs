use std::sync::mpsc;

use anyhow::Result;
use rust_bert::pipelines::{
    sequence_classification::Label, zero_shot_classification::ZeroShotClassificationModel,
};
use tokio::{
    sync::oneshot,
    task::{self, JoinHandle},
};

/// Message type for internal channel, passing around texts and return value
/// senders
type Message = (Vec<String>, oneshot::Sender<Vec<Vec<Label>>>);

/// Runner for sentiment classification
#[derive(Debug, Clone)]
pub struct ZeroshotClassifier {
    sender: mpsc::SyncSender<Message>,
}

impl ZeroshotClassifier {
    /// Spawn a classifier on a separate thread and return a classifier instance
    /// to interact with it
    pub fn spawn() -> (JoinHandle<Result<()>>, ZeroshotClassifier) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = task::spawn_blocking(move || Self::runner(receiver));
        (handle, ZeroshotClassifier { sender })
    }

    /// The classification runner itself
    fn runner(receiver: mpsc::Receiver<Message>) -> Result<()> {
        // Needs to be in sync runtime, async doesn't work
        let model = ZeroShotClassificationModel::new(Default::default())?;
        let candidate_labels = &["politics", "public health", "economics", "sports"];

        while let Ok((texts, sender)) = receiver.recv() {
            let texts: Vec<&str> = texts.iter().map(String::as_str).collect();
            let sentiments = model
                .predict_multilabel(texts, candidate_labels, None, 128)
                .unwrap();
            sender.send(sentiments).expect("sending results");
        }

        Ok(())
    }

    /// Make the runner predict a sample and return the result
    pub async fn predict(&self, texts: Vec<String>) -> Result<Vec<Vec<Label>>> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send((texts, sender))?;
        Ok(receiver.await?)
    }
}
