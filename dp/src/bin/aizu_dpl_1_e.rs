macro_rules ! input {(source = $ s : expr , $ ($ r : tt ) * ) => {let mut iter = $ s . split_whitespace () ; let mut next = || {iter . next () . unwrap () } ; input_inner ! {next , $ ($ r ) * } } ; ($ ($ r : tt ) * ) => {let stdin = std :: io :: stdin () ; let mut bytes = std :: io :: Read :: bytes (std :: io :: BufReader :: new (stdin . lock () ) ) ; let mut next = move || -> String {bytes . by_ref () . map (| r | r . unwrap () as char ) . skip_while (| c | c . is_whitespace () ) . take_while (| c |! c . is_whitespace () ) . collect () } ; input_inner ! {next , $ ($ r ) * } } ; }
macro_rules ! input_inner {($ next : expr ) => {} ; ($ next : expr , ) => {} ; ($ next : expr , $ var : ident : $ t : tt $ ($ r : tt ) * ) => {let $ var = read_value ! ($ next , $ t ) ; input_inner ! {$ next $ ($ r ) * } } ; }
macro_rules ! read_value {($ next : expr , ($ ($ t : tt ) ,* ) ) => {($ (read_value ! ($ next , $ t ) ) ,* ) } ; ($ next : expr , [$ t : tt ; $ len : expr ] ) => {(0 ..$ len ) . map (| _ | read_value ! ($ next , $ t ) ) . collect ::< Vec < _ >> () } ; ($ next : expr , chars ) => {read_value ! ($ next , String ) . chars () . collect ::< Vec < char >> () } ; ($ next : expr , usize1 ) => {read_value ! ($ next , usize ) - 1 } ; ($ next : expr , $ t : ty ) => {$ next () . parse ::<$ t > () . expect ("Parse error" ) } ; }

macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }

macro_rules ! echo {($ ($ e : expr ) ,* ) => {let mut s = Vec :: new () ; $ (s . push (format ! ("{}" , $ e ) ) ; ) * println ! ("{}" , s . join (" " ) ) ; } }

fn main() {
    input! {
        s: chars,
        t: chars,
    };
    let mut dp = vec![vec![std::usize::MAX; t.len() + 1]; s.len() + 1];
    dp[0][0] = 0;
    for s_i in 0..=s.len() {
        for t_i in 0..=t.len() {
            if s_i > 0 && t_i > 0 {
                if s[s_i - 1] == t[t_i - 1] {
                    chmin!(dp[s_i][t_i], dp[s_i - 1][t_i - 1]);
                } else {
                    chmin!(dp[s_i][t_i], dp[s_i - 1][t_i - 1] + 1);
                }
            }
            if s_i > 0 {
                chmin!(dp[s_i][t_i], dp[s_i - 1][t_i] + 1);
            }
            if t_i > 0 {
                chmin!(dp[s_i][t_i], dp[s_i][t_i - 1] + 1);
            }
        }
    }
    echo!(dp[s.len()][t.len()]);
}
