const CSZ: &str = "cszCSZ";
const CSZ_WITH_CARON: &str = "cszčšžCSZČŠŽ";

pub fn convert_to_czs(word: &str) -> String {
    word.replace("š", "s")
        .replace("č", "c")
        .replace("ž", "z")
}

pub fn contains_czs(word: &str) -> bool {
    word.chars()
        .any(|c| CSZ.contains(c))
}

pub fn contains_czs_plus_with_caron(word: &str) -> bool {
    word.chars()
        .any(|c| CSZ_WITH_CARON.contains(c))
}

pub fn fix_case(target: &str, desired_format: &str) -> String {
    target.chars()
        .zip(desired_format.chars())
        .map(|(a, b)| {
            if "ABCĆDEFGHIJKLMNOPQRSŠTUVWXYZŽ".contains(b) {
                a.to_uppercase().next().unwrap()
            } else {
                a
            }
        }).collect()
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_fix_case() {
        let target = "priča";
        let desired_format = "Prica";
        assert_eq!(fix_case(&target, &desired_format), "Priča");
    }

    #[test]
    fn test_convert_to_czs() {
        let word = "priča";
        assert_eq!(convert_to_czs(word), "prica");
    }

    #[test]
    fn test_contains_czs_1() {
        let word = "priča";
        assert_eq!(contains_czs(word), false);
    }

    #[test]
    fn test_contains_czs_2() {
        let word = "prica";
        assert_eq!(contains_czs(word), true);
    }

    #[test]
    fn test_contains_czs_plus_with_caron_1() {
        let word = "priča";
        assert_eq!(contains_czs_plus_with_caron(word), true);
    }

    #[test]
    fn test_contains_czs_plus_with_caron_2() {
        let word = "prica";
        assert_eq!(contains_czs_plus_with_caron(word), false);
    }
}
