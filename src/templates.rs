// Convert &str parameters in a vec definition to Strings.
macro_rules! string_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(String::from($x));
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
    pub noun_types: Vec<String>,
    pub generate_ainu: fn(&[String], &[String]) -> String,
    pub generate_jpan: fn(&[String], &[String], &[String]) -> String,
}

pub fn get_templates() -> Vec<SentenceTemplate> {
    vec![
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING],
            noun_types: string_vec!["*"],
            generate_ainu: |verbs, nouns| format!("{} ku= {}.", nouns[0], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("私が{}{}{}。", nouns[0], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING],
            noun_types: string_vec!["*", "*"],
            generate_ainu: |verbs, nouns| format!("{} {} {}.", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}が{}{}{}。", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec!["*", "*"],
            generate_ainu: |verbs, nouns| format!("{} anak {} {} ruwe ne.", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}んだよ。", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec!["*", "*"],
            generate_ainu: |verbs, nouns| format!("{} anak {} somo {} ya?", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}んじゃないの？", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB1 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec!["*"],
            generate_ainu: |verbs, nouns| format!("{} anak {} ya?", nouns[0], verbs[0]),
            generate_jpan: |verbs, nouns, _particles| format!("{}は{}の？", nouns[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB1 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec!["*"],
            generate_ainu: |verbs, nouns| format!("{} anak somo {} ya?", nouns[0], verbs[0]),
            generate_jpan: |verbs, nouns, _particles| format!("{}は{}んじゃないの？", nouns[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec!["*", "*"],
            generate_ainu: |_verbs, nouns| format!("{} anak {} ne ya?", nouns[0], nouns[1]),
            generate_jpan: |_verbs, nouns, _particles| format!("{}は{}なの？", nouns[0], nouns[1]),
        },
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec!["*", "*"],
            generate_ainu: |verbs, nouns| format!("{} anak {} {} ya?", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}の？", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![],
            noun_types: string_vec!["*"],
            generate_ainu: |_verbs, nouns| format!("toanpe {} ne wa.", nouns[0]),
            generate_jpan: |_verbs, nouns, _particles| format!("あれは{}だよ。", nouns[0]),
        },
        SentenceTemplate {
            verb_types: vec![VERB2 | V_BOTH | V_SING | V_PLUR],
            noun_types: string_vec!["*", "*"],
            generate_ainu: |verbs, nouns| format!("{} anak {} {} ya?", nouns[0], nouns[1], verbs[0]),
            generate_jpan: |verbs, nouns, particles| format!("{}は{}{}{}の？", nouns[0], nouns[1], particles[0], verbs[0]),
        },
        SentenceTemplate {
            verb_types: vec![
                VERB1 | V_BOTH | V_SING | V_PLUR,
                VERB2 | V_BOTH | V_SING | V_PLUR,
            ],
            noun_types: string_vec!["*", "*"],
            generate_ainu: |verbs, nouns| format!("toanta {} {} {} {} ya?", verbs[0], nouns[0], nouns[1], verbs[1]),
            generate_jpan: |verbs, nouns, particles| format!("あそこで{}{}{}{}{}の？", verbs[0], nouns[0], nouns[1], particles[1], verbs[1]),
        },
    ]
}
