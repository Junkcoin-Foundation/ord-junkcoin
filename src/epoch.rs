use super::*;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use crate::sat::Sat;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Display, PartialOrd)]
pub(crate) struct Epoch(pub(crate) u32);

fn read_sat_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Sat>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let sats: Vec<u64> = serde_json::from_reader(reader)?;

    Ok(sats.into_iter().map(Sat).collect())
}

lazy_static! {
    pub(crate) static ref STARTING_SATS: Vec<Sat> = {
        let path = env::var("STARTING_SATS_PATH").expect("STARTING_SATS_PATH must be set");
        read_sat_from_file(&path).expect("Failed to read JSON")
    };
}

#[derive(Debug, Serialize, Deserialize)]
struct Epochs {
    epochs: HashMap<u32, u64>,
}

static EPOCHS: Lazy<Epochs> = Lazy::new(|| {
    let path = env::var("SUBSIDIES_PATH").expect("SUBSIDIES_PATH must be set");
    let data = fs::read_to_string(&path).expect("Unable to read file");
    serde_json::from_str(&data).expect("Unable to parse JSON")
});

impl Epoch {
  pub(crate) const FIRST_POST_SUBSIDY: Self = Self(11); // Last epoch for Junkcoin

  pub fn get_starting_sats() -> &'static Vec<Sat> {
    &STARTING_SATS
  }

  pub(crate) fn subsidy(self) -> u64 {
      match EPOCHS.epochs.get(&self.0) {
          Some(&value) => value,
          None => panic!("bad epoch"),
      }
  }

  pub(crate) fn starting_sat(self) -> Sat {
    *Self::get_starting_sats()
      .get(usize::try_from(self.0).unwrap())
      .unwrap_or_else(|| Self::get_starting_sats().last().unwrap())
  }

  pub(crate) fn starting_height(self) -> Height {
    if self.0 < 101 {
      Height(0)
    } else if self.0 < 1541 {
      Height(101)
    } else if self.0 < 2981 {
      Height(1541)
    } else if self.0 < 5861 {
      Height(2981)
    } else if self.0 < 262800 {
      Height(5861)
    } else if self.0 < 394200 {
      Height(262800)
    } else if self.0 < 657000 {
      Height(394200)
    } else if self.0 < 1182600 {
      Height(657000)
    } else if self.0 < 1971000 {
      Height(1182600)
    } else if self.0 < 2759400 {
      Height(1971000)
    } else if self.0 < 3547800 {
      Height(2759400)
    } else {
      Height(3547800)
    }
  }
}

impl PartialEq<u32> for Epoch {
  fn eq(&self, other: &u32) -> bool {
    self.0 == *other
  }
}

impl From<Sat> for Epoch {
  fn from(sat: Sat) -> Self {
    let starting_sats = Self::get_starting_sats();

    let len = starting_sats.len();
    for i in 0..len-1 {
      if sat < starting_sats[i+1] {
        return Epoch(i as u32);
      }
    }

    // If none of the starting sats is greater than the given sat, return the last Epoch
    Epoch(145_005)
  }
}

impl From<Height> for Epoch {
  fn from(height: Height) -> Self {
    if height.0 < 101 {
      Epoch(0)
    } else if height.0 < 1541 {
      Epoch(1)
    } else if height.0 < 2981 {
      Epoch(2)
    } else if height.0 < 5861 {
      Epoch(3)
    } else if height.0 < 262800 {
      Epoch(4)
    } else if height.0 < 394200 {
      Epoch(5)
    } else if height.0 < 657000 {
      Epoch(6)
    } else if height.0 < 1182600 {
      Epoch(7)
    } else if height.0 < 1971000 {
      Epoch(8)
    } else if height.0 < 2759400 {
      Epoch(9)
    } else if height.0 < 3547800 {
      Epoch(10)
    } else {
      Epoch(11)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::*;

  #[test]
  fn starting_sat() {
    assert_eq!(Epoch(0).starting_sat(), 0);
  }

  #[test]
  fn subsidy() {
    assert_eq!(Epoch(0).subsidy(), 1000 * COIN_VALUE);
    assert_eq!(Epoch(1).subsidy(), 500 * COIN_VALUE);
    assert_eq!(Epoch(2).subsidy(), 200 * COIN_VALUE);
    assert_eq!(Epoch(11).subsidy(), (0.390625 * COIN_VALUE as f64) as u64);
  }

  #[test]
  fn starting_height() {
    assert_eq!(Epoch(0).starting_height(), 0);
    assert_eq!(Epoch(1).starting_height(), 101);
    assert_eq!(Epoch(2).starting_height(), 1541);
    assert_eq!(Epoch(3).starting_height(), 2981);
    assert_eq!(Epoch(11).starting_height(), 3547800);
  }

  #[test]
  fn from_height() {
    assert_eq!(Epoch::from(Height(0)), 0);
    assert_eq!(Epoch::from(Height(101)), 1);
    assert_eq!(Epoch::from(Height(1541)), 2);
    assert_eq!(Epoch::from(Height(2981)), 3);
    assert_eq!(Epoch::from(Height(3547800)), 11);
  }

  #[test]
  fn from_sat() {
    for (epoch, starting_sat) in Epoch::get_starting_sats().into_iter().enumerate() {
      if epoch > 0 {
        assert_eq!(
          Epoch::from(Sat(starting_sat.n() - 1)),
          Epoch(epoch as u64 - 1)
        );
      }
      assert_eq!(Epoch::from(starting_sat), Epoch(epoch as u64));
      assert_eq!(Epoch::from(starting_sat + 1), Epoch(epoch as u64));
    }
    assert_eq!(Epoch::from(Sat(0)), 0);
    assert_eq!(Epoch::from(Sat(1)), 0);
    assert_eq!(Epoch::from(Epoch(1).starting_sat()), 1);
    assert_eq!(Epoch::from(Epoch(1).starting_sat() + 1), 1);
  }

  #[test]
  fn eq() {
    assert_eq!(Epoch(0), 0);
    assert_eq!(Epoch(100), 100);
  }
}
