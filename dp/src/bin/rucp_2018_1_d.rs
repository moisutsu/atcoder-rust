macro_rules ! input {(source = $ s : expr , $ ($ r : tt ) * ) => {let mut iter = $ s . split_whitespace () ; let mut next = || {iter . next () . unwrap () } ; input_inner ! {next , $ ($ r ) * } } ; ($ ($ r : tt ) * ) => {let stdin = std :: io :: stdin () ; let mut bytes = std :: io :: Read :: bytes (std :: io :: BufReader :: new (stdin . lock () ) ) ; let mut next = move || -> String {bytes . by_ref () . map (| r | r . unwrap () as char ) . skip_while (| c | c . is_whitespace () ) . take_while (| c |! c . is_whitespace () ) . collect () } ; input_inner ! {next , $ ($ r ) * } } ; }
macro_rules ! input_inner {($ next : expr ) => {} ; ($ next : expr , ) => {} ; ($ next : expr , $ var : ident : $ t : tt $ ($ r : tt ) * ) => {let $ var = read_value ! ($ next , $ t ) ; input_inner ! {$ next $ ($ r ) * } } ; }
macro_rules ! read_value {($ next : expr , ($ ($ t : tt ) ,* ) ) => {($ (read_value ! ($ next , $ t ) ) ,* ) } ; ($ next : expr , [$ t : tt ; $ len : expr ] ) => {(0 ..$ len ) . map (| _ | read_value ! ($ next , $ t ) ) . collect ::< Vec < _ >> () } ; ($ next : expr , chars ) => {read_value ! ($ next , String ) . chars () . collect ::< Vec < char >> () } ; ($ next : expr , usize1 ) => {read_value ! ($ next , usize ) - 1 } ; ($ next : expr , $ t : ty ) => {$ next () . parse ::<$ t > () . expect ("Parse error" ) } ; }

macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }

macro_rules ! echo {($ ($ e : expr ) ,* ) => {let mut s = Vec :: new () ; $ (s . push (format ! ("{}" , $ e ) ) ; ) * println ! ("{}" , s . join (" " ) ) ; } }

fn main() {
    input! {
        N: usize, M: usize,
        a_s: [f64; N],
    };
    let mut dp = vec![vec![0.0; M + 1]; N];
    for n_i in 0..N {
        dp[n_i][0] = a_s[0..n_i + 1].iter().sum::<f64>();
    }
    for m_i in 1..=M {
        for n_i in 0..N {

        }
    }
}
