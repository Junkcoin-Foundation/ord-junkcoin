use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub junes: BTreeMap<June, JuneInfo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JuneInfo {
  pub block: u64,
  pub burned: u128,
  pub divisibility: u8,
  pub etching: Txid,
  pub height: u64,
  pub id: JuneId,
  pub index: u32,
  pub terms: Option<Terms>,
  pub mints: u128,
  pub number: u64,
  pub premine: u128,
  pub june: June,
  pub spacers: u32,
  pub supply: u128,
  pub symbol: Option<char>,
  pub timestamp: DateTime<Utc>,
  pub turbo: bool,
  pub tx: u32,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;

  ensure!(
    index.has_june_index(),
    "`ord junes` requires index created with `--index-junes` flag",
  );

  index.update()?;

  Ok(Box::new(Output {
    junes: index
      .junes()?
      .into_iter()
      .map(
        |(
          id,
          entry @ JuneEntry {
            block,
            burned,
            divisibility,
            etching,
            terms,
            mints,
            number,
            premine,
            june,
            spacers,
            supply,
            symbol,
            timestamp,
            turbo,
          },
        )| {
          (
            june,
            JuneInfo {
              block,
              burned,
              divisibility,
              etching,
              height: id.height,
              id,
              index: id.index,
              terms,
              mints,
              number,
              premine,
              timestamp: crate::timestamp(timestamp),
              june,
              spacers,
              supply,
              symbol,
              turbo,
              tx: id.index,
            },
          )
        },
      )
      .collect::<BTreeMap<June, JuneInfo>>(),
  }))
}
