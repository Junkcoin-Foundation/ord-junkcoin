use super::*;

#[derive(Boilerplate)]
pub(crate) struct JunesHtml {
  pub(crate) entries: Vec<(JuneId, JuneEntry)>,
}

impl PageContent for JunesHtml {
  fn title(&self) -> String {
    "Junes".to_string()
  }
}
