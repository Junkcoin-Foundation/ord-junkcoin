use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use crate::InscriptionId;

#[derive(Debug, Clone, PartialEq, thiserror::Error, Deserialize, Serialize)]
pub enum JUNK20Error {
  #[error("invalid number: {0}")]
  InvalidNum(String),

  #[error("tick invalid supply {0}")]
  InvalidSupply(String),

  #[error("tick: {0} has been existed")]
  DuplicateTick(String),

  #[error("tick: {0} not found")]
  TickNotFound(String),

  #[error("illegal tick length '{0}'")]
  InvalidTickLen(String),

  #[error("decimals {0} too large")]
  DecimalsTooLarge(u8),

  #[error("tick: {0} has been minted")]
  TickMinted(String),

  #[error("tick: {0} mint limit out of range {0}")]
  MintLimitOutOfRange(String, String),

  #[error("zero amount not allowed")]
  InvalidZeroAmount,

  #[error("amount overflow: {0}")]
  AmountOverflow(String),

  #[error("insufficient balance: {0} {1}")]
  InsufficientBalance(String, String),

  #[error("amount exceed limit: {0}")]
  AmountExceedLimit(String),

  #[error("transferable inscriptionId not found: {0}")]
  TransferableNotFound(InscriptionId),

  #[error("invalid inscribe to coinbase")]
  InscribeToCoinbase,

  #[error("transferable owner not match {0}")]
  TransferableOwnerNotMatch(InscriptionId),

  /// an InternalError is an error that happens exceed our expect
  /// and should not happen under normal circumstances
  #[error("internal error: {0}")]
  InternalError(String),

  // num error
  #[error("{op} overflow: original: {org}, other: {other}")]
  Overflow {
    op: String,
    org: String,
    other: String,
  },

  #[error("invalid integer {0}")]
  InvalidInteger(String),
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum JSONError {
  #[error("invalid content type")]
  InvalidContentType,

  #[error("unsupport content type")]
  UnSupportContentType,

  #[error("invalid json string")]
  InvalidJson,

  #[error("not junk20 json")]
  NotJUNK20Json,

  #[error("parse operation json error: {0}")]
  ParseOperationJsonError(String),
}

pub trait DataStore {
  type Error: Debug + Display;
}

// Define the Error enum
#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum Error<L: DataStore> {
  #[error("junk20 error: {0}")]
  JUNK20Error(JUNK20Error),

  #[error("ledger error: {0}")]
  LedgerError(L::Error),
}

impl DataStore for JUNK20Error {
  type Error = redb::Error; // Replace with your actual error type
}

impl<L: DataStore> From<JUNK20Error> for Error<L> {
  fn from(e: JUNK20Error) -> Self {
    Self::JUNK20Error(e)
  }
}
