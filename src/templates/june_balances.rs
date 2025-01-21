use super::*;

#[derive(Boilerplate, Debug, PartialEq, Serialize, Deserialize)]
pub struct JuneBalancesHtml {
  pub balances: BTreeMap<SpacedJune, BTreeMap<OutPoint, u128>>,
}

impl PageContent for JuneBalancesHtml {
  fn title(&self) -> String {
    "June Balances".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const JUNE: u128 = 99246114928149462;

  #[test]
  fn display_june_balances() {
    let balances: BTreeMap<June, BTreeMap<OutPoint, u128>> = vec![
      (
        June(JUNE),
        vec![(
          OutPoint {
            txid: txid(1),
            vout: 1,
          },
          1000,
        )]
        .into_iter()
        .collect(),
      ),
      (
        June(JUNE + 1),
        vec![(
          OutPoint {
            txid: txid(2),
            vout: 2,
          },
          12345678,
        )]
        .into_iter()
        .collect(),
      ),
    ]
    .into_iter()
    .collect();

    assert_regex_match!(
      JuneBalancesHtml { balances }.to_string(),
      "<h1>June Balances</h1>
<table>
  <tr>
    <th>june</th>
    <th>balances</th>
  </tr>
  <tr>
    <td><a href=/june/AAAAAAAAAAAAA>.*</a></td>
    <td>
      <table>
        <tr>
          <td class=monospace>
            <a href=/output/1{64}:1>1{64}:1</a>
          </td>
          <td class=monospace>
            1000
          </td>
        </tr>
      </table>
    </td>
  </tr>
  <tr>
    <td><a href=/june/AAAAAAAAAAAAB>.*</a></td>
    <td>
      <table>
        <tr>
          <td class=monospace>
            <a href=/output/2{64}:2>2{64}:2</a>
          </td>
          <td class=monospace>
            12345678
          </td>
        </tr>
      </table>
    </td>
  </tr>
</table>
"
      .unindent()
    );
  }
}
