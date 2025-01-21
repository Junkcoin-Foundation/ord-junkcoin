use {super::*, std::num::TryFromIntError};

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq, Ord, PartialOrd)]
pub struct JuneId {
  pub height: u64,
  pub index: u32,
}

impl TryFrom<u128> for JuneId {
  type Error = TryFromIntError;

  fn try_from(n: u128) -> Result<Self, Self::Error> {
    Ok(Self {
      height: u64::try_from(n >> 16)?,
      index: u32::try_from(n & 0xFFFF).unwrap(),
    })
  }
}

impl From<JuneId> for u128 {
  fn from(id: JuneId) -> Self {
    u128::from(id.height) << 16 | u128::from(id.index)
  }
}

impl Display for JuneId {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}:{}", self.height, self.index,)
  }
}

impl FromStr for JuneId {
  type Err = crate::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (height, index) = s
        .split_once(':')
        .ok_or_else(|| anyhow!("invalid june ID: {s}"))?;

    Ok(Self {
      height: height.parse()?,
      index: index.parse()?,
    })
  }
}

impl Serialize for JuneId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for JuneId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
  {
    Ok(DeserializeFromStr::deserialize(deserializer)?.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn june_id_to_128() {
    assert_eq!(
      0b11_0000_0000_0000_0001u128,
      JuneId {
        height: 3,
        index: 1,
      }
      .into()
    );
  }

  #[test]
  fn display() {
    assert_eq!(
      JuneId {
        height: 1,
        index: 2
      }
      .to_string(),
      "1:2"
    );
  }

  #[test]
  fn from_str() {
    assert!(":".parse::<JuneId>().is_err());
    assert!("1:".parse::<JuneId>().is_err());
    assert!(":2".parse::<JuneId>().is_err());
    assert!("a:2".parse::<JuneId>().is_err());
    assert!("1:a".parse::<JuneId>().is_err());
    assert_eq!(
      "1:2".parse::<JuneId>().unwrap(),
      JuneId {
        height: 1,
        index: 2
      }
    );
  }

  #[test]
  fn try_from() {
    assert_eq!(
      JuneId::try_from(0x060504030201).unwrap(),
      JuneId {
        height: 0x06050403,
        index: 0x0201
      }
    );

    assert!(JuneId::try_from(0x07060504030201).is_err());
  }

  #[test]
  fn serde() {
    let june_id = JuneId {
      height: 1,
      index: 2,
    };
    let json = "\"1:2\"";
    assert_eq!(serde_json::to_string(&june_id).unwrap(), json);
    assert_eq!(serde_json::from_str::<JuneId>(json).unwrap(), june_id);
  }
}
