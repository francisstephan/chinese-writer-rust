use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

#[derive(Deserialize)]
pub struct CharData {
    pub carac: String,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct PinyinData {
    pub pinyin_ton: String,
}

#[derive(Serialize)]
pub struct Zi {
    pub id: i64,
    pub pinyin_ton: String,
    pub unicode: String,
    pub hanzi: char,
    pub sens: String,
    pub strokes: i64,
}

#[derive(sqlx::FromRow)]
pub struct DBidzi {
    id: i64,
    pinyin_ton: String,
    unicode: String,
    sens: String,
    strokes: i64,
}

#[derive(Deserialize)]
pub struct ZiStrData {
    pub zistr: String,
}

pub async fn getsize(client: Arc<AppState>) -> i64 {
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pyhz")
        .fetch_one(&client.db)
        .await
        .unwrap();

    result.0
}

pub async fn list_for_zi(data: Arc<AppState>, first: String) -> Vec<Zi> {
    let whereclause = format!(" unicode = '{}' ORDER BY pinyin_ton", &first);
    readdic(&data, &whereclause).await
}

pub async fn list_for_py(data: Arc<AppState>, first: String) -> Vec<Zi> {
    if let Some(last_char) = &first.chars().last() {
        let cond = matches!(last_char, '0'..='4');
        let whereclause = if !cond {
            // no tone given: check all tones 0 to 4
            format!(
                " pinyin_ton = '{}0' OR pinyin_ton = '{}1' OR pinyin_ton = '{}2' OR pinyin_ton = '{}3' OR pinyin_ton = '{}4' ORDER BY strokes, pinyin_ton, unicode",
                &first, &first, &first, &first, &first
            )
        } else {
            format!(" pinyin_ton = '{}' ORDER BY strokes, unicode", &first)
        };
        readdic(&data, &whereclause).await
    } else {
        let v: Vec<Zi> = Vec::new();
        v
    }
}

pub async fn readdic(data: &Arc<AppState>, whereclause: &str) -> Vec<Zi> {
    let basequery = "SELECT id, pinyin_ton, unicode, sens, strokes FROM pyhz";
    let qq: String;
    let query = if !whereclause.is_empty() {
        qq = format!("{} WHERE {}", basequery, whereclause);
        &qq
    } else {
        basequery
    };
    read_query(query, &data).await
}

pub async fn read_query(query: &str, data: &Arc<AppState>) -> Vec<Zi> {
    let mut disp = Vec::<Zi>::new();
    let dic = sqlx::query_as::<_, DBidzi>(&query)
        .fetch_all(&data.db)
        .await
        .unwrap();

    for dbidzi in dic.iter() {
        //https://stackoverflow.com/questions/69152223/unicode-codepoint-to-rust-string
        let unicodestring = dbidzi.unicode.as_str();
        let unicode = u32::from_str_radix(unicodestring, 16).unwrap();
        let carac = char::from_u32(unicode).unwrap();
        let zi = Zi {
            id: dbidzi.id,
            pinyin_ton: dbidzi.pinyin_ton.clone(),
            unicode: dbidzi.unicode.clone(),
            hanzi: carac,
            sens: dbidzi.sens.clone(),
            strokes: dbidzi.strokes,
        };
        disp.push(zi);
    }

    disp
}

pub async fn getcandidatelist(query: String, data: Arc<AppState>) -> String {
    let veczi = list_for_py(data, query).await;
    let mut retour = String::new();
    if veczi.len() == 0 {
        retour
    } else {
        for zi in veczi {
            retour.push_str("<button class='zilistbutton' onclick='add(\"");
            retour.push(zi.hanzi);
            retour.push_str("\")'>");
            retour.push(zi.hanzi);
            retour.push_str("</button>");
        }
        retour
    }
}

pub async fn zi_to_py(data: &Arc<AppState>, carac: char) -> Vec<String> {
    // get unicode from carac:
    let mut unicode = format!("{:x}", carac as u32);
    unicode = unicode.to_uppercase();
    let query = format!(
        "SELECT pinyin_ton FROM pyhz WHERE unicode = '{}' ORDER BY pinyin_ton ASC",
        unicode
    );
    let mut disp = Vec::<String>::new();
    let dic = sqlx::query_as::<_, PinyinData>(&query)
        .fetch_all(&data.db)
        .await
        .unwrap();
    for pinyindata in dic.iter() {
        disp.push(pinyindata.pinyin_ton.clone());
    }
    disp
}
