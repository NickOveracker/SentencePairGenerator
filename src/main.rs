use rusqlite::{params, Connection, Result}; use std::env;

struct SentenceTemplate {
    verb_types: Vec<u8>, // BITMASK. 1st nibble is valency (0-3), 2nd nibble is plurality (0-2)
    noun_count: usize,
    generate_ainu: fn(&[String], &[String]) -> String,
    generate_jpan: fn(&[String], &[String], &[String]) -> String,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let db_path = args.get(1).map_or("./vocab.db", String::as_str);
    let verbose: usize = args.get(2).map_or(0, |n| n.parse::<usize>().expect("args[2] must be int"));
    let conn = Connection::open(db_path)?;

    let templates = vec![
        SentenceTemplate {
            verb_types: vec![0x43],
            noun_count: 1,
            generate_ainu: |verbs, nouns| format!("{} ku= {}.", nouns[0], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("私が{}{}{}。", nouns[0], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![0x43],
            noun_count: 2,
            generate_ainu: |verbs, nouns| format!("{} {} {}.", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}が{}{}{}。", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![0x47],
            noun_count: 2,
            generate_ainu: |verbs, nouns| format!("{} anak {} {} ruwe ne.", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}んだよ。", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![0x47],
            noun_count: 2,
            generate_ainu: |verbs, nouns| format!("{} anak {} somo {} ya?", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}んじゃないの？", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![0x27],
            noun_count: 1,
            generate_ainu: |verbs, nouns| format!("{} anak {} ya?", nouns[0], verbs[0]),
            generate_jpan: |verbs, nouns, _particles| format!("{}は{}の？", nouns[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![0x27],
            noun_count: 1,
            generate_ainu: |verbs, nouns| format!("{} anak somo {} ya?", nouns[0], verbs[0]),
            generate_jpan: |verbs, nouns, _particles| format!("{}は{}んじゃないの？", nouns[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_count: 2,
            generate_ainu: |_verbs, nouns| format!("{} anak {} ne ya?", nouns[0], nouns[1]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}は{}なの？", nouns[0], nouns[1]),
        },
        SentenceTemplate {
            verb_types: vec![0x47],
            noun_count: 2,
            generate_ainu: |verbs, nouns| format!("{} anak {} {} ya?", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}の？", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_count: 1,
            generate_ainu: |_verbs, nouns| format!("toanpe {} ne wa.", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("あれは{}だよ。", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![0x47],
            noun_count: 2,
            generate_ainu: |verbs, nouns| format!("{} anak {} {} ya?", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}の？", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![0x27, 0x47],
            noun_count: 2,
            generate_ainu: |verbs, nouns| format!("toanta {} {} {} {} ya?", verbs[0], nouns[0], nouns[1], verbs[1]),
            generate_jpan: |verbs, nouns, particles| format!("あそこで{}{}{}{}{}の？", verbs[0], nouns[0], nouns[1], particles[1], verbs[1]),
        },
    ];

    for template in templates {
        let verbs = get_verbs(&conn, &template.verb_types, verbose)?;
        let nouns = get_nouns(&conn, template.noun_count, verbose)?;

        let ainu_sentence = (template.generate_ainu)(&verbs.0, &nouns.0);
        let jpan_sentence = (template.generate_jpan)(&verbs.1, &nouns.1, &verbs.2);

        println!("{}", ainu_sentence);
        println!("{}", jpan_sentence);
    }

    Ok(())
}

fn get_verbs(conn: &Connection, verb_types: &[u8], verbose: usize) -> Result<(Vec<String>, Vec<String>, Vec<String>)> {
    let mut verbs_ainu = Vec::new();
    let mut verbs_jpan = Vec::new();
    let mut particles  = Vec::new();
    let query = build_verb_query(verb_types);

    if verbose != 0 {
        println!("{}", query);
    }

    if verb_types.len() > 0 {
        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query(params![])?;

        while let Some(row) = rows.next()? {
            verbs_ainu.push(row.get(0)?);
            verbs_jpan.push(row.get(1)?);
            particles.push(row.get(2)?);
        }
    }

    Ok((verbs_ainu, verbs_jpan, particles))
}

fn build_verb_query(verb_types: &[u8]) -> String {
    let mut conditions = Vec::new();
    let mut needs_particle: bool = false;

    for &verb_type in verb_types {
        for ii in 0..4 {
            if (verb_type >> 4 & (1 << ii)) != 0 {
                if ii > 1 {
                    needs_particle = true;
                }

                for jj in 0..3 {
                    if (verb_type & (1 << jj)) != 0 {
                        conditions.push(format!("trans = {} AND plur = {}", ii, jj));
                    }
                }
            }
        }
    }

    let mut query = format!("SELECT ainu, jpan, particle FROM verbs WHERE ({})", conditions.join(" OR "));

    if needs_particle {
        query = format!("{} AND particle <> ''", query);
    }

    format!("{} ORDER BY RANDOM() LIMIT {}", query, verb_types.len())
}

fn get_nouns(conn: &Connection, noun_count: usize, verbose: usize) -> Result<(Vec<String>, Vec<String>)> {
    let mut nouns_ainu = Vec::new();
    let mut nouns_jpan = Vec::new();
    let query = &format!("SELECT ainu, jpan FROM nouns ORDER BY RANDOM() LIMIT {}", noun_count);

    if verbose != 0 {
        println!("{}", query);
    }

    if noun_count > 0 {
        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query(params![])?;

        while let Some(row) = rows.next()? {
            nouns_ainu.push(row.get(0)?);
            nouns_jpan.push(row.get(1)?);
        }
    }

    Ok((nouns_ainu, nouns_jpan))
}
