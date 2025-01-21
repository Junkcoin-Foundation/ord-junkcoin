use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct SpacedJune {
  pub(crate) june: June,
  pub(crate) spacers: u32,
}

impl SpacedJune {
  pub fn new(june: June, spacers: u32) -> Self {
    Self { june, spacers }
  }
}

impl FromStr for SpacedJune {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut june = String::new();
    let mut spacers = 0u32;

    for c in s.chars() {
      match c {
        'A'..='Z' => june.push(c),
        '.' | '•' => {
          let flag = 1 << june.len().checked_sub(1).context("leading spacer")?;
          if spacers & flag != 0 {
            bail!("double spacer");
          }
          spacers |= flag;
        }
        _ => bail!("invalid character"),
      }
    }

    if 32 - spacers.leading_zeros() >= june.len().try_into().unwrap() {
      bail!("trailing spacer")
    }

    Ok(SpacedJune {
      june: june.parse()?,
      spacers,
    })
  }
}

impl Display for SpacedJune {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let june = self.june.to_string();

    for (i, c) in june.chars().enumerate() {
      write!(f, "{c}")?;

      if i < june.len() - 1 && self.spacers & 1 << i != 0 {
        write!(f, "•")?;
      }
    }

    Ok(())
  }
}

impl Serialize for SpacedJune {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for SpacedJune {
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
  fn display() {
    assert_eq!("A.B".parse::<SpacedJune>().unwrap().to_string(), "A•B");
    assert_eq!("A.B.C".parse::<SpacedJune>().unwrap().to_string(), "A•B•C");
  }

  #[test]
  fn from_str() {
    #[track_caller]
    fn case(s: &str, june: &str, spacers: u32) {
      assert_eq!(
        s.parse::<SpacedJune>().unwrap(),
        SpacedJune {
          june: june.parse().unwrap(),
          spacers
        },
      );
    }

    assert_eq!(
      ".A".parse::<SpacedJune>().unwrap_err().to_string(),
      "leading spacer",
    );

    assert_eq!(
      "A..B".parse::<SpacedJune>().unwrap_err().to_string(),
      "double spacer",
    );

    assert_eq!(
      "A.".parse::<SpacedJune>().unwrap_err().to_string(),
      "trailing spacer",
    );

    assert_eq!(
      "Ax".parse::<SpacedJune>().unwrap_err().to_string(),
      "invalid character",
    );

    case("A.B", "AB", 0b1);
    case("A.B.C", "ABC", 0b11);
    case("A•B", "AB", 0b1);
    case("A•B•C", "ABC", 0b11);
  }

  #[test]
  fn serde() {
    let spaced_june = SpacedJune {
      june: June(26),
      spacers: 1,
    };
    let json = "\"A•A\"";
    assert_eq!(serde_json::to_string(&spaced_june).unwrap(), json);
    assert_eq!(
      serde_json::from_str::<SpacedJune>(json).unwrap(),
      spaced_june
    );
  }
}
