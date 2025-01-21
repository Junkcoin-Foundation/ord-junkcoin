use super::*;

#[derive(Boilerplate)]
pub(crate) struct InscriptionsHtml {
  pub(crate) inscriptions: Vec<InscriptionId>,
  pub(crate) prev: Option<u64>,
  pub(crate) next: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InscriptionsJson {
  pub inscriptions: Vec<InscriptionId>,
}

impl PageContent for InscriptionsHtml {
  fn title(&self) -> String {
    "junkscription".into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn without_prev_and_next() {
    assert_regex_match!(
      InscriptionsHtml {
        inscriptions: vec![inscription_id(1), inscription_id(2)],
        prev: None,
        next: None,
      },
      "
        <h1>junkscription</h1>
        <div class=thumbnails>
          <a href=/junkscription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>
          <a href=/junkscription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
        </div>
        .*
        prev
        next
        .*
      "
      .unindent()
    );
  }

  #[test]
  fn with_prev_and_next() {
    assert_regex_match!(
      InscriptionsHtml {
        inscriptions: vec![inscription_id(1), inscription_id(2)],
        prev: Some(1),
        next: Some(2),
      },
      "
        <h1>junkscription</h1>
        <div class=thumbnails>
          <a href=/junkscription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>
          <a href=/junkscription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
        </div>
        .*
        <a class=prev href=/junkscriptions/1>prev</a>
        <a class=next href=/junkscriptions/2>next</a>
        .*
      "
      .unindent()
    );
  }
}
