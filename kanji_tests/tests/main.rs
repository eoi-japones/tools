#[cfg(test)]
mod tests {
    #[test]
    fn can_get_character_list() {
        let fixtures_path =
            std::fs::canonicalize(std::path::PathBuf::from("tests/fixtures/kanjis"));
        let result: Vec<kanji_tests::Kanji> =
            kanji_tests::get_character_list(&fixtures_path.unwrap());

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].clave, "hoja");
        assert_eq!(result[1].clave, "alforfón");
        assert_eq!(result[2].clave, "desteñido");
    }

    #[test]
    fn can_format_character_list_as_csv() {
        let result_regex: regex::Regex = regex::Regex::new(r".+,.+").unwrap();
        let input: Vec<kanji_tests::Kanji> = vec![
            kanji_tests::Kanji {
                id: String::from("kanji_a"),
                clave: String::from("kanji_a_clave"),
                historia: String::from("kanji_a_historia"),
                componentes: Vec::new(),
                como_componente: Vec::new(),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_b"),
                clave: String::from("kanji_b_clave"),
                historia: String::from("kanji_b_historia"),
                componentes: Vec::new(),
                como_componente: Vec::new(),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_c"),
                clave: String::from("kanji_c_clave"),
                historia: String::from("kanji_c_historia"),
                componentes: Vec::new(),
                como_componente: Vec::new(),
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
                clave: String::from("kanji_a_clave"),
                historia: String::from("kanji_a_historia"),
                componentes: Vec::new(),
                como_componente: Vec::new(),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_b"),
                clave: String::from("kanji_b_clave"),
                historia: String::from("kanji_b_historia"),
                componentes: Vec::new(),
                como_componente: Vec::new(),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_c"),
                clave: String::from("kanji_c_clave"),
                historia: String::from("kanji_c_historia"),
                componentes: Vec::new(),
                como_componente: Vec::new(),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_d"),
                clave: String::from("kanji_d_clave"),
                historia: String::from("kanji_d_historia"),
                componentes: Vec::new(),
                como_componente: Vec::new(),
            },
            kanji_tests::Kanji {
                id: String::from("kanji_e"),
                clave: String::from("kanji_e_clave"),
                historia: String::from("kanji_e_historia"),
                componentes: Vec::new(),
                como_componente: Vec::new(),
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
