use crate::Searcher;

impl Searcher {
    #[allow(dead_code)]
    fn naive_pattern_search_not_optimized(string: String, pattern: String) -> Option<Vec<usize>> {
        let string: Vec<char> = string.chars().collect();
        let pattern: Vec<char> = pattern.chars().collect();
        let pattern_len = pattern.len();
        let mut output: Vec<usize> = Vec::new();

        for i in 0..(string.len() - pattern_len + 1) {
            for j in 0..pattern_len {
                if string[i + j] != pattern[j] {
                    break;
                }

                if j == pattern_len - 1 {
                    output.push(i);
                }
            }
        }

        if output.len() > 0 {
            return Some(output);
        }

        None
    }

    #[allow(dead_code)]
    fn naive_pattern_search(string: String, pattern: String) -> Option<Vec<usize>> {
        let string: Vec<char> = string.chars().collect();
        let pattern: Vec<char> = pattern.chars().collect();
        let pattern_len = pattern.len();
        let mut output: Vec<usize> = Vec::new();

        for i in 0..(string.len() - pattern_len + 1) {
            let mut j = 0;
            while j < pattern_len {
                if string[i + j] != pattern[j] {
                    break;
                }

                j += 1;
            }

            if j == pattern_len {
                output.push(i);
            }
        }

        if output.len() > 0 {
            return Some(output);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_match() {
        let rna_strand = String::from("AAUAGCCAUGAACUCCA");
        let codon = String::from("UAA");
        let output = Searcher::naive_pattern_search(rna_strand, codon);

        assert_eq!(output, None);
    }

    #[test]
    fn starting_codon() {
        let rna_strand = String::from("AAUAGCCAUGAACUCCA");
        let codon = String::from("AUG");
        let output = Searcher::naive_pattern_search(rna_strand, codon);

        assert_eq!(output, Some(vec![7]));
    }

    #[test]
    fn find_last_occurrence() {
        let fruits = String::from("apple orange pineapple apple");
        let apple = String::from("apple");
        let output = Searcher::naive_pattern_search(fruits, apple);

        assert_eq!(output, Some(vec![0, 17, 23]));
    }
}
