pub mod config;
use std::sync::mpsc;

use anyhow::Result;
use core_services::state::config::env_var;
use rust_bert::{
    pipelines::{
        common::ModelResource,
        sequence_classification::Label,
        zero_shot_classification::{ZeroShotClassificationConfig, ZeroShotClassificationModel},
    },
    resources::LocalResource,
};
use tch::Device;
use tokio::{
    sync::oneshot,
    task::{self, JoinHandle},
};
use tracing::{debug, error, trace};

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
        debug!("initialising nlp");
        let config = Self::create_resources();
        debug!("config read");
        // Needs to be in sync runtime, async doesn't work
        let config = ZeroShotClassificationConfig {
            model_type: rust_bert::pipelines::common::ModelType::Bart,
            model_resource: ModelResource::Torch(Box::new(LocalResource {
                local_path: config.model_resource,
            })),
            config_resource: Box::new(LocalResource {
                local_path: config.config_resource,
            }),
            vocab_resource: Box::new(LocalResource {
                local_path: config.vocab_resource,
            }),
            merges_resource: Some(Box::new(LocalResource {
                local_path: config.merges_resource,
            })),
            lower_case: false,
            strip_accents: None,
            add_prefix_space: None,
            device: Device::cuda_if_available(),
            kind: None,
        };
        trace!("creating model");
        let model = ZeroShotClassificationModel::new(config)?;
        debug!("model created");

        let candidate_labels = &["politics", "public health", "economics", "sports"];

        while let Ok((texts, sender)) = receiver.recv() {
            let texts: Vec<&str> = texts.iter().map(String::as_str).collect();
            match model.predict_multilabel(texts, candidate_labels, None, 128) {
                Ok(sentiments) => {
                    sender.send(sentiments).expect("channel dropped");
                }
                Err(e) => {
                    error!("{e}");
                }
            }
        }

        Ok(())
    }

    fn create_resources() -> config::Config {
        // https://huggingface.co/facebook/bart-large-mnli/tree/main
        config::Config {
            config_resource: env_var("NLP_CONFIG_RESOURCE_PATH").into(),
            model_resource: env_var("NLP_MODEL_RESOURCE_PATH").into(),
            vocab_resource: env_var("NLP_VOCAB_RESOURCE_PATH").into(),
            merges_resource: env_var("NLP_MERGES_RESOURCE_PATH").into(),
        }
    }

    /// Make the runner predict a sample and return the result
    pub async fn predict(&self, texts: Vec<String>) -> Result<Vec<Vec<Label>>> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send((texts, sender))?;
        Ok(receiver.await?)
    }
}
