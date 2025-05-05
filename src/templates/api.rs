use super::*;

#[derive(Boilerplate)]
pub struct ApiHtml;

impl PageContent for ApiHtml {
  fn title(&self) -> String {
    "API Documentation".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_api_html() {
    assert_regex_match!(
      ApiHtml.to_string(),
      r"<h1>API Documentation</h1>"
    );
  }
}
