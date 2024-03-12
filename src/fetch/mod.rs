use std::error::Error;



///  [ https://random-word-api.herokuapp.com/ ]
/*
    ----------------------------|     Requests API     |----------------------------

    {get}    /all                  -      getting all words;
    {get}    /word                 -      getting 1 random word;
    {get}    /word?number={i32}    -      set words array size;
    {get}    /word?length={i32}    -      set words size;
    {get}    /word?lang={lang}     -      set language;  ["it","zh","de","fr","es"]

    ------------------------| Requests API { information } |------------------------

    {get}    /languages            -      view all supported languages;
*/
///



pub enum Langs
{
  it, zh, de, fr, es,
}

pub struct Request
{
  res: Option<String>
}

impl Request
{
  pub fn new() -> Self
  {
    Request{res: None}
  }

  pub async fn gen_words(&mut self, arr_sz: i32, lang: Langs) -> Result<(), Box<dyn Error>>
  {
    let lang_f = match lang {
      Langs::de => "de",
      Langs::fr => "fr",
      Langs::es => "es",
      Langs::it => "it",
      Langs::zh => "zh",
    };
    let frmt = format!("https://random-word-api.herokuapp.com/word?number={arr_sz}&lang={lang_f}");
    let req = reqwest::get(&frmt)
        .await?
        .text()
        .await?;
      self.res = Some(req);
      Ok(())
  }
}
