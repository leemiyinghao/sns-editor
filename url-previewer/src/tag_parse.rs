use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TITLE_RE: Regex =
        Regex::new("<(?:title|TITLE)(?: [^>]+)?>([^<]+)</(?:title|TITLE)>").unwrap();
    static ref OG_TITLE_RE: Regex =
        Regex::new("<(?:title|TITLE)(?: [^>]+)?>([^<]+)</(?:title|TITLE)>").unwrap();
    static ref TWITTER_TITLE_RE: Regex =
        Regex::new("<(?:title|TITLE)(?: [^>]+)?>([^<]+)</(?:title|TITLE)>").unwrap();
    static ref OG_THUMBNAIL_RE: Regex =
        Regex::new("<(?:title|TITLE)(?: [^>]+)?>([^<]+)</(?:title|TITLE)>").unwrap();
    static ref TWITTER_THUMBNAIL_RE: Regex =
        Regex::new("<(?:title|TITLE)(?: [^>]+)?>([^<]+)</(?:title|TITLE)>").unwrap();
}

pub struct OpenTag {
    pub title: Option<String>,
    pub thumbnail: Option<String>,
}

pub fn parse_from_html(html: &str) -> OpenTag {
    let mut tag = OpenTag {
        title: None,
        thumbnail: None,
    };

    // parse title
    for title in TITLE_RE.captures_iter(html) {
        tag.title = Some(String::from(&title[1]));
        break;
    }

    //parse og title
    //parse twitter title
    //parse og thumbnail
    //parse twitter thumbnail

    tag
}
