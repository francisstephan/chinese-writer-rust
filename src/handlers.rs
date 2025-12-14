use crate::dbase;
use crate::forms;
use axum::Form;
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};
use std::fs;
use std::sync::{Arc, OnceLock};
use tera::Tera;

use crate::AppState;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TERA: Tera = match Tera::new("vol/templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Unable to parse templates: {}", e);
            std::process::exit(1);
        }
    };
}

static DBSIZE: OnceLock<i64> = OnceLock::new();

pub async fn index(State(client): State<Arc<AppState>>) -> impl IntoResponse {
    if DBSIZE.get().is_none() {
        let size = dbase::getsize(&client).await;
        let _ = DBSIZE.get_or_init(|| size);
    }
    let size = DBSIZE.get().unwrap();
    let mut context = tera::Context::new();
    if *size > 0 {
        context.insert("contenu", "Connected to database");
    } else {
        context.insert("contenu", "Could not connect to database");
    }
    let output = TERA.render("index.html", &context);
    Html(output.unwrap())
}

pub async fn size(State(client): State<Arc<AppState>>) -> impl IntoResponse {
    if DBSIZE.get().is_none() {
        let size = dbase::getsize(&client).await;
        let _ = DBSIZE.get_or_init(|| size);
    }
    let size = DBSIZE.get().unwrap();

    let metadata = fs::metadata("vol/zidian.db").expect("Failed to read file metadata");
    let time = metadata.modified().unwrap();
    use chrono::prelude::{DateTime, Utc};
    let dt: DateTime<Utc> = time.clone().into();
    let content = format!(
        "The dictionary presently contains {} entries. Last updated on {}",
        &size,
        &dt.format("%Y-%m-%d")
    );
    content.into_response()
}

pub async fn getpyform() -> impl IntoResponse {
    forms::pyform().into_response()
}

pub async fn getziform() -> impl IntoResponse {
    forms::ziform().into_response()
}

pub async fn getparseform() -> impl IntoResponse {
    forms::zistringform().into_response()
}

pub async fn zilist(
    State(client): State<Arc<AppState>>,
    Form(zi): Form<dbase::CharData>, // caution:the extractor should follow the state
) -> impl IntoResponse {
    let chain = &zi.carac;
    let first: char = chain.chars().next().unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("query", &chain);
    let disp = dbase::list_for_zi(client, format!("{:X}", first as u32)).await;
    ctx.insert("dico", &disp);
    let output = TERA.render("components/zilist.html", &ctx);
    Html(output.unwrap())
}

pub async fn pylist(
    State(client): State<Arc<AppState>>,
    Form(chardata): Form<dbase::PinyinData>, // caution:the extractor should follow the state
) -> impl IntoResponse {
    let chain = &chardata.pinyin_ton;
    let mut ctx = tera::Context::new();
    ctx.insert("query", &chain);
    let disp = dbase::list_for_py(client, String::from(chain)).await;
    ctx.insert("dico", &disp);
    let output = TERA.render("components/zilist.html", &ctx);
    Html(output.unwrap())
}

pub async fn listdic(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("query", "List dictionary");
    let query = "SELECT id, pinyin_ton, unicode, sens, strokes FROM pyhz ORDER BY pinyin_ton, strokes, unicode";
    let disp = dbase::read_query(query, &data).await;
    ctx.insert("dico", &disp);
    let output = TERA.render("components/zilist.html", &ctx);
    Html(output.unwrap())
}

pub async fn cancel() -> impl IntoResponse {
    String::from("Form canceled").into_response()
}

pub async fn writehanzistring() -> impl IntoResponse {
    forms::whs().into_response()
}

pub async fn candidatelist(
    State(data): State<Arc<AppState>>,
    Form(chardata): Form<dbase::PinyinData>,
) -> impl IntoResponse {
    let chain = chardata.pinyin_ton;
    let answer = dbase::getcandidatelist(chain, data).await;
    let mut resp: String;
    if answer.is_empty() {
        resp = "<br /><br />No hanzi available for request".to_owned()
    } else {
        resp = String::from("<br /><br />Select one hanzi from this list:</br>");
        resp.push_str(&answer)
    }
    resp.into_response()
}

pub async fn stringparse(
    State(data): State<Arc<AppState>>,
    Form(formdata): Form<dbase::ZiStrData>,
) -> impl IntoResponse {
    let chain = &formdata.zistr;
    let mut chars = chain.chars();
    let mut parsed = String::new();
    let mut unknown = Vec::<String>::new();
    let mut nonzi: bool = false; // special fromatting when first non zi character in sequence
    while let Some(carac) = chars.next() {
        // 1. If carac is not a chinese character or is a punctuation mark, simply append it to parsed
        if (carac as i64) < 0x2000
            || "。，“”（）、《》—；：！？「」 【】『』％‘’•".find(carac) != None
        {
            if nonzi {
                parsed = format!("{}{}", parsed, carac)
            } else {
                parsed = format!("{}   {}", parsed, carac); // insert spaces before first non zi character
                nonzi = true;
            }
        } else {
            nonzi = false; // this is a zi: reset nonzi
            // 2. get all pinyin for the carac character in the database
            let disp = dbase::zi_to_py(&data, carac).await;
            if disp.len() > 0 {
                // 3. The character exists in the database: give all pinyin separated by /
                parsed = format!("{} ", parsed); // insert space for better readability
                for (i, py) in disp.iter().enumerate() {
                    if i > 0 {
                        parsed = format!("{}{}", parsed, "/");
                    }
                    parsed = format!("{}{}", parsed, py);
                }
            } else {
                // 4. The character is not in the base: add it to the unknown Vec
                // 5. and append it as such (unparsed) to parsed
                unknown.push(carac.to_string());
                parsed = format!("{} {}", parsed, carac);
            }
        }
    }

    let mut ctx = tera::Context::new();
    ctx.insert("query", &chain);
    ctx.insert("parsedstr", &parsed);
    ctx.insert("unknownzi", &unknown);

    let output = TERA.render("components/parsed.html", &ctx);
    Html(output.unwrap())
}

pub async fn askquiz(State(client): State<Arc<AppState>>) -> impl IntoResponse {
    if DBSIZE.get().is_none() {
        let size = dbase::getsize(&client).await;
        let _ = DBSIZE.get_or_init(|| size);
    }
    let size = DBSIZE.get().unwrap() - 1; // max offset = dbsize -1
    let mut numlin: i64 = (&client.next() * (size as f64)).round() as i64;
    if numlin >= size {
        numlin = size - 1;
    }
    let zi = dbase::zi_from_linenum(&client, numlin).await;
    let mut ctx = tera::Context::new();
    ctx.insert("hanzi", &zi);
    let output = TERA.render("components/askquiz.html", &ctx);
    Html(output.unwrap())
}

pub async fn ansquiz(
    State(client): State<Arc<AppState>>,
    Path(hanzi): Path<String>,
) -> impl IntoResponse {
    let zi: char = hanzi.chars().next().unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("hanzi", &hanzi);
    let disp = dbase::list_for_zi(client, format!("{:X}", zi as u32)).await;
    ctx.insert("dico", &disp);
    let output = TERA.render("components/ansquiz.html", &ctx);
    Html(output.unwrap())
}
