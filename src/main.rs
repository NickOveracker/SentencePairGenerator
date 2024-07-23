use rusqlite::{params, Connection, Result};
use std::env;

struct SentenceTemplate {
    verb_types: Vec<u8>,
    noun_count: usize,
    generate_ainu: fn(&[String], &[String]) -> String,
    generate_jpan: fn(&[String], &[String]) -> String,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let db_path = args.get(1).map_or("./vocab.db", String::as_str);
    let conn = Connection::open(db_path)?;

    let templates = vec![
        SentenceTemplate {
            verb_types: vec![0x43],
            noun_count: 1,
            generate_ainu: |verbs, nouns| format!("{} ku= {}.", nouns[0], verbs[0]),
            generate_jpan: |verbs, nouns| format!("私が{}を{}。", nouns[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![0x43],
            noun_count: 2,
            generate_ainu: |verbs, nouns| format!("{} {} {}.", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns| format!("{}が{}を{}。", nouns[0], nouns[1], verbs[0]),
        },
    ];

    for template in templates {
        let verbs = get_verbs(&conn, &template.verb_types)?;
        let nouns = get_nouns(&conn, template.noun_count)?;

        let ainu_sentence = (template.generate_ainu)(&verbs.0, &nouns.0);
        let jpan_sentence = (template.generate_jpan)(&verbs.1, &nouns.1);

        println!("{}", ainu_sentence);
        println!("{}", jpan_sentence);
    }

    Ok(())
}

fn get_verbs(conn: &Connection, verb_types: &[u8]) -> Result<(Vec<String>, Vec<String>)> {
    let mut verbs_ainu = Vec::new();
    let mut verbs_jpan = Vec::new();
    let query = build_verb_query(verb_types);

    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(params![])?;
    println!("{}", query);

    while let Some(row) = rows.next()? {
        verbs_ainu.push(row.get(0)?);
        verbs_jpan.push(row.get(1)?);
    }

    Ok((verbs_ainu, verbs_jpan))
}

fn build_verb_query(verb_types: &[u8]) -> String {
    let mut conditions = Vec::new();

    for &verb_type in verb_types {
        for ii in 0..4 {
            if (verb_type >> 4 & (1 << ii)) != 0 {
                for jj in 0..3 {
                    if (verb_type & (1 << jj)) != 0 {
                        conditions.push(format!("trans = {} AND plur = {}", ii, jj));
                    }
                }
            }
        }
    }

    format!("SELECT ainu, jpan FROM verbs WHERE {} ORDER BY RANDOM() LIMIT {}", conditions.join(" OR "), verb_types.len())
}

fn get_nouns(conn: &Connection, noun_count: usize) -> Result<(Vec<String>, Vec<String>)> {
    let mut nouns_ainu = Vec::new();
    let mut nouns_jpan = Vec::new();

    let mut stmt = conn.prepare(&format!("SELECT ainu, jpan FROM nouns ORDER BY RANDOM() LIMIT {}", noun_count))?;
    let mut rows = stmt.query(params![])?;

    while let Some(row) = rows.next()? {
        nouns_ainu.push(row.get(0)?);
        nouns_jpan.push(row.get(1)?);
    }

    Ok((nouns_ainu, nouns_jpan))
}
