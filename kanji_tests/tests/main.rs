#[cfg(test)]
mod tests {
    #[test]
    fn can_get_character_list() {
        let fixtures_path =
            std::fs::canonicalize(std::path::PathBuf::from("tests/fixtures/kanjis"));
        let result: Vec<kanji_tests::Kanji> =
            kanji_tests::get_character_list(&fixtures_path.unwrap(), false);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].clave.clone().unwrap(), "hoja");
        assert_eq!(result[1].clave.clone().unwrap(), "alforfón");
        assert_eq!(result[2].clave.clone().unwrap(), "desteñido");
    }

    #[test]
    fn can_format_character_list_as_csv() {
        let result_regex: regex::Regex = regex::Regex::new(r".+,.+").unwrap();
        let input: Vec<kanji_tests::Kanji> = vec![
            kanji_tests::Kanji {
                id: String::from("kanji_a"),
                clave: Some(String::from("kanji_a_clave")),
                historia: Some(String::from("kanji_a_historia")),
                componentes: Some(Vec::new()),
                como_componente: Some(Vec::new()),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_b"),
                clave: Some(String::from("kanji_b_clave")),
                historia: Some(String::from("kanji_b_historia")),
                componentes: Some(Vec::new()),
                como_componente: Some(Vec::new()),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_c"),
                clave: Some(String::from("kanji_c_clave")),
                historia: Some(String::from("kanji_c_historia")),
                componentes: Some(Vec::new()),
                como_componente: Some(Vec::new()),
            },
        ];

        let result = kanji_tests::format_character_list_as_csv(&input, 2);
        assert_eq!(result_regex.is_match(&result), true);
    }

    #[test]
    fn character_list_is_randomized_each_time() {
        let input: Vec<kanji_tests::Kanji> = vec![
            kanji_tests::Kanji {
                id: String::from("kanji_a"),
                clave: Some(String::from("kanji_a_clave")),
                historia: Some(String::from("kanji_a_historia")),
                componentes: Some(Vec::new()),
                como_componente: Some(Vec::new()),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_b"),
                clave: Some(String::from("kanji_b_clave")),
                historia: Some(String::from("kanji_b_historia")),
                componentes: Some(Vec::new()),
                como_componente: Some(Vec::new()),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_c"),
                clave: Some(String::from("kanji_c_clave")),
                historia: Some(String::from("kanji_c_historia")),
                componentes: Some(Vec::new()),
                como_componente: Some(Vec::new()),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_d"),
                clave: Some(String::from("kanji_d_clave")),
                historia: Some(String::from("kanji_d_historia")),
                componentes: Some(Vec::new()),
                como_componente: Some(Vec::new()),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_e"),
                clave: Some(String::from("kanji_e_clave")),
                historia: Some(String::from("kanji_e_historia")),
                componentes: Some(Vec::new()),
                como_componente: Some(Vec::new()),
            },
        ];

        let result1 = kanji_tests::format_character_list_as_csv(&input, 4);
        let result2 = kanji_tests::format_character_list_as_csv(&input, 4);
        let result3 = kanji_tests::format_character_list_as_csv(&input, 4);
        assert_ne!(result1, result2);
        assert_ne!(result1, result3);
        assert_ne!(result2, result3);
    }
}
