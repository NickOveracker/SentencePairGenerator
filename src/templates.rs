pub struct SentenceTemplate {
    pub verb_types: Vec<u8>, // BITMASK. 1st nibble is valency (0-3), 2nd nibble is plurality (0-2)
    pub noun_count: usize,
    pub generate_ainu: fn(&[String], &[String]) -> String,
    pub generate_jpan: fn(&[String], &[String], &[String]) -> String,
}

pub fn get_templates() -> Vec<SentenceTemplate> {
    vec![
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
    ]
}
