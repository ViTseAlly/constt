use reqwest::Client;
use std::{fmt, collections::HashMap, error::Error};

pub enum Langs {
    It, Zh, De, Fr, Es,
}

impl Langs
{
  pub fn hsh_pm() -> HashMap<i32, Langs>
  {
    let mp = HashMap::from([
      (1, Langs::It),
      (2, Langs::Zh),
      (3, Langs::De),
      (4, Langs::Fr),
      (5, Langs::Es),
  ]);
  mp
  }
}

pub struct Request {
    res: Option<String>
}

impl Request {
    pub fn new() -> Self {
        Request { res: None }
    }

    pub async fn gen_words(&mut self, arr_sz: i32, lang: &Langs) -> Result<(), Box<dyn Error>> {
        let lang_f = match lang {
            Langs::It => "de",
            Langs::Zh => "fr",
            Langs::De => "es",
            Langs::Fr => "it",
            Langs::Es => "zh",
        };
        let frmt = format!("https://random-word-api.herokuapp.com/word?number={}&lang={}", arr_sz, lang_f);
        let client = Client::new();
        let req = client.get(&frmt)
            .send()
            .await?
            .text()
            .await?;
        self.res = Some(req);
        Ok(())
    }
}

impl fmt::Display for Request {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match &self.res {
          Some(result) => write!(f, "{}", result),
          None => write!(f, "No data"),
      }
  }
}
