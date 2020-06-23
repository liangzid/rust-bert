// Copyright 2019-present, the HuggingFace Inc. team, The Google AI Language Team and Facebook, Inc.
// Copyright 2019 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Named Entity Recognition pipeline
//! Extracts entities (Person, Location, Organization, Miscellaneous) from text.
//! BERT cased large model finetuned on CoNNL03, contributed by the [MDZ Digital Library team at the Bavarian State Library](https://github.com/dbmdz)
//! All resources for this model can be downloaded using the Python utility script included in this repository.
//! 1. Set-up a Python virtual environment and install dependencies (in ./requirements.txt)
//! 2. Run the conversion script python /utils/download-dependencies_bert_ner.py.
//! The dependencies will be downloaded to the user's home directory, under ~/rustbert/bert-ner
//!
//! ```no_run
//! use rust_bert::pipelines::ner::NERModel;
//! # fn main() -> failure::Fallible<()> {
//! let ner_model = NERModel::new(Default::default())?;
//!
//! let input = [
//!     "My name is Amy. I live in Paris.",
//!     "Paris is a city in France.",
//! ];
//! let output = ner_model.predict(&input);
//! # Ok(())
//! # }
//! ```
//! Output: \
//! ```no_run
//! # use rust_bert::pipelines::question_answering::Answer;
//! # use rust_bert::pipelines::ner::Entity;
//! # let output =
//! [
//!     Entity {
//!         word: String::from("Amy"),
//!         score: 0.9986,
//!         label: String::from("I-PER"),
//!     },
//!     Entity {
//!         word: String::from("Paris"),
//!         score: 0.9985,
//!         label: String::from("I-LOC"),
//!     },
//!     Entity {
//!         word: String::from("Paris"),
//!         score: 0.9988,
//!         label: String::from("I-LOC"),
//!     },
//!     Entity {
//!         word: String::from("France"),
//!         score: 0.9993,
//!         label: String::from("I-LOC"),
//!     },
//! ]
//! # ;
//! ```

use crate::pipelines::token_classification::{TokenClassificationConfig, TokenClassificationModel};

#[derive(Debug)]
/// # Entity generated by a `NERModel`
pub struct Entity {
    /// String representation of the Entity
    pub word: String,
    /// Confidence score
    pub score: f64,
    /// Entity label (e.g. ORG, LOC...)
    pub label: String,
}

//type alias for some backward compatibility
type NERConfig = TokenClassificationConfig;

/// # NERModel to extract named entities
pub struct NERModel {
    token_classification_model: TokenClassificationModel,
}

impl NERModel {
    /// Build a new `NERModel`
    ///
    /// # Arguments
    ///
    /// * `ner_config` - `NERConfig` object containing the resource references (model, vocabulary, configuration) and device placement (CPU/GPU)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() -> failure::Fallible<()> {
    /// use rust_bert::pipelines::ner::NERModel;
    ///
    /// let ner_model = NERModel::new(Default::default())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(ner_config: NERConfig) -> failure::Fallible<NERModel> {
        let model = TokenClassificationModel::new(ner_config)?;
        Ok(NERModel {
            token_classification_model: model,
        })
    }

    /// Extract entities from a text
    ///
    /// # Arguments
    ///
    /// * `input` - `&[&str]` Array of texts to extract entities from.
    ///
    /// # Returns
    ///
    /// * `Vec<Entity>` containing extracted entities
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() -> failure::Fallible<()> {
    /// # use rust_bert::pipelines::ner::NERModel;
    ///
    /// let ner_model = NERModel::new(Default::default())?;
    /// let input = [
    ///     "My name is Amy. I live in Paris.",
    ///     "Paris is a city in France.",
    /// ];
    /// let output = ner_model.predict(&input);
    /// # Ok(())
    /// # }
    /// ```
    pub fn predict(&self, input: &[&str]) -> Vec<Entity> {
        self.token_classification_model
            .predict(input, true, false)
            .into_iter()
            .filter(|token| token.label != "O")
            .map(|token| Entity {
                word: token.text,
                score: token.score,
                label: token.label,
            })
            .collect()
    }
}
