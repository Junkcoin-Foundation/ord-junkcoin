use super::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PType {
  #[serde(rename = "junk-20")]
  Junk20,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
  Transfer,
  Mint,
  Deploy,
  Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct JUNK20 {
  pub p: Option<PType>,
  pub op: Option<Operation>,
  pub tick: Option<String>,
  pub amt: Option<String>,
  pub max: Option<String>,
  pub limit: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct JUNK20Balance {
  tick: String,
  transferable: String,
  available: String,
  utxos: Option<Vec<JUNK20Output>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct JUNK20Output {
  #[serde(flatten)]
  pub utxo: Utxo,
  pub junk20: JUNK20UtxoOutput,
  pub inscription_id: InscriptionId,
  pub inscription_number: u64,
  pub offset: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct JUNK20UtxoOutput {
  pub balance: String,
  pub operation: Operation,
  pub valid: bool,
}

impl JUNK20Balance {
  pub fn from_strings(
    tick: &str,
    transferable: &str,
    available: &str,
    utxos: Vec<JUNK20Output>,
  ) -> Option<Self> {
    Some(Self {
      tick: tick.to_string(),
      transferable: transferable.to_string(),
      available: available.to_string(),
      utxos: if utxos.is_empty() { None } else { Some(utxos) },
    })
  }
}

impl JUNK20 {
  pub fn from_json_string(json_str: &str) -> Option<Self> {
    match serde_json::from_str::<JUNK20>(json_str) {
      Ok(junk20) => {
        if junk20.is_valid() {
          Some(junk20)
        } else {
          None
        }
      }
      Err(err) => {
        log::debug!("Error deserializing JSON: {}", err);
        None
      }
    }
  }

  fn is_valid(&self) -> bool {
    self.p.is_some()
        && self.tick.is_some()
        && self.clone().op.is_some_and(|op| op != Operation::Unknown)
  }
}
