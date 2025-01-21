use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub junes: BTreeMap<SpacedJune, BTreeMap<OutPoint, u128>>,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;

  ensure!(
    index.has_june_index(),
    "`ord balances` requires index created with `--index-junes` flag",
  );

  index.update()?;

  Ok(Box::new(Output {
    junes: index.get_june_balance_map()?,
  }))
}
