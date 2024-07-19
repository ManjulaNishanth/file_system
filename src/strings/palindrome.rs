use std::char;

fn is_palindrome(input: &String) -> bool {
    let rev = input.chars().rev().collect::<String>();
    println!("\n {rev:?}");
    input == &rev
}

/*

*/
fn possible_longest_palindrome(s: &String) -> Option<String> {
    let (n, mut ans) = (s.len(), &s[..1]);
    println!("\n tuple {n:?}, {ans:?}");
    let mut dp = vec![vec![false; n]; n];
    println!("\n dp {dp:?}");
    let data: Vec<char> = s.chars().collect();
    println!("\n str_chars {data:?}");

    for end in 1..n {
        for start in 0..=end {
            println!("\n start : {start:?}, end : {end:?}");
            if data[start] == data[end] {
                dp[start][end] = end - start < 2 || dp[start + 1][end - 1];
                if dp[start][end] && end - start + 1 > ans.len() {
                    ans = &s[start..=end];
                }
            }
        }
    }
    println!("\n ans {ans:?}");
    None
}

mod tests {
    use super::*;

    #[test]
    pub fn possible_longest_palindrome_cfg() {
        let input = "dadm".to_string();
        let result = possible_longest_palindrome(&input);
        assert_eq!(result, None)
    }

    #[test]
    pub fn is_palindrome_cfg() {
        let input = "dad".to_string();
        let result = is_palindrome(&input);
        assert_eq!(result, true)
    }
}
