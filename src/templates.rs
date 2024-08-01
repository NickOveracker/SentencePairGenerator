// Convert &str parameters in a vec definition to Strings.
macro_rules! string_vec {
    ( $( ($x:expr, $y:expr) ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push((String::from($x), String::from($y)));
            )*
            temp_vec
        }
    };
}

/** BITMASKS **/
// Valency masks
const VERB0: u8 = 0x10; // 完動   (impersonal/avalent)
const VERB1: u8 = 0x20; // 自動   (intransitive)
const VERB2: u8 = 0x40; // 他動   (transitive)
const VERB3: u8 = 0x80; // 複他動 (ditransitive)

// Plurality masks
const V_BOTH: u8 = 0x01; // Singular and plural both allowed
const V_SING: u8 = 0x02; // Singular only
const V_PLUR: u8 = 0x04; // Plural only

pub struct SentenceTemplate {
    // BITMASK
    pub verb_types: Vec<u8>,
    pub noun_types: Vec<(String, String)>, // ("category1", "category2")
    pub generate_ainu: fn(&[String], &[String]) -> String,
    pub generate_jpan: fn(&[String], &[String], &[String]) -> String,
}

pub fn get_templates() -> Vec<SentenceTemplate> {
    vec![
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING],
            noun_types: string_vec![("%", "%")],
            generate_ainu: |verbs, nouns| format!("{} ku= {}.", nouns[0], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("私が{}{}{}。", nouns[0], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING],
            noun_types: string_vec![("%", "%"), ("%", "%")],
            generate_ainu: |verbs, nouns| format!("{} {} {}.", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}が{}{}{}。", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec![("%", "%"), ("%", "%")],
            generate_ainu: |verbs, nouns| format!("{} anak {} {} ruwe ne.", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}んだよ。", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec![("%", "%"), ("%", "%")],
            generate_ainu: |verbs, nouns| format!("{} anak {} somo {} ya?", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}んじゃないの？", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB1 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec![("%", "%")],
            generate_ainu: |verbs, nouns| format!("{} anak {} ya?", nouns[0], verbs[0]),
            generate_jpan: |verbs, nouns, _particles| format!("{}は{}の？", nouns[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB1 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec![("%", "%")],
            generate_ainu: |verbs, nouns| format!("{} anak somo {} ya?", nouns[0], verbs[0]),
            generate_jpan: |verbs, nouns, _particles| format!("{}は{}んじゃないの？", nouns[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("%", "%"), ("%", "%")],
            generate_ainu: |_verbs, nouns| format!("{} anak {} ne ya?", nouns[0], nouns[1]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}は{}なの？", nouns[0], nouns[1]),
        },
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec![("%", "%"), ("%", "%")],
            generate_ainu: |verbs, nouns| format!("{} anak {} {} ya?", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}の？", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("%", "%")],
            generate_ainu: |_verbs, nouns| format!("toanpe {} ne wa.", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("あれは{}だよ。", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec![("%", "%"), ("%", "%")],
            generate_ainu: |verbs, nouns| format!("{} anak {} {} ya?", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}の？", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![
                VERB1 | V_BOTH | V_SING | V_PLUR,
                VERB2 | V_BOTH | V_SING | V_PLUR,
            ],
            noun_types: string_vec![("%", "%"), ("%", "%")],
            generate_ainu: |verbs, nouns| format!("toanta {} {} {} {} ya?", verbs[0], nouns[0], nouns[1], verbs[1]),
            generate_jpan: |verbs, nouns, particles| format!("あそこで{}{}は{}{}{}の？", verbs[0], nouns[0], nouns[1], particles[1], verbs[1]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("動物", "動物"), ("植物", "植物")],
            generate_ainu: |_verbs, nouns| format!("{} {} e ya?", nouns[0], nouns[1]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}は{}を食べるの？", nouns[0], nouns[1]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("動物", "動物"), ("植物", "植物")],
            generate_ainu: |_verbs, nouns| format!("{} anak {} e ya?", nouns[0], nouns[1]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}は{}を食べるの？", nouns[0], nouns[1]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人", "人"), ("植物", "植物")],
            generate_ainu: |_verbs, nouns| format!("{} anak {} e ya?", nouns[0], nouns[1]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}は{}を食べるの？", nouns[0], nouns[1]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人工物", "SKIP")],
            generate_ainu: |_verbs, nouns| format!("{} ku=kor rusuy", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}が欲しい", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人工物", "SKIP")],
            generate_ainu: |_verbs, nouns| format!("{} ci=kor rusuy", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("私たちは{}が欲しい", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人工物", "SKIP")],
            generate_ainu: |_verbs, nouns| format!("{} a=kor rusuy", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("我々は{}が欲しい", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人工物", "SKIP")],
            generate_ainu: |_verbs, nouns| format!("{} e=kor rusuy ya?", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}が欲しい？", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人工物", "SKIP")],
            generate_ainu: |_verbs, nouns| format!("{} e=kor rusuy ya?", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}が欲しい？", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人工物", "SKIP")],
            generate_ainu: |_verbs, nouns| format!("{} e=kor rusuy ya?", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}が欲しい？", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人工物", "SKIP")],
            generate_ainu: |_verbs, nouns| format!("{} eci=kor rusuy ya?", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("あなたたちは{}が欲しい？", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人工物", "SKIP")],
            generate_ainu: |_verbs, nouns| format!("{} en=kore yan.", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}をください。", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人工物", "SKIP")],
            generate_ainu: |_verbs, nouns| format!("{} un=kore", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}を我々にくれた。", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec![("人工物", "SKIP")],
            generate_ainu: |_verbs, nouns| format!("{} e=kore yan.", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}をあげて下さい。", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![
                VERB0 | V_BOTH | V_PLUR | V_SING,
                VERB1 | V_BOTH | V_SING,
            ],
            noun_types: vec![],
            generate_ainu: |verbs, _nouns| format!("{} kusu ku= {}", verbs[0], verbs[1]),
            generate_jpan: |verbs, _nouns, _particles| format!("{}ので私は{}", verbs[0], verbs[1]),
        }
    ]
}
