// P("abbca") = P("abbc") + a("abbca") + 
//
// P("a") = a("a")
//
// P("ab") = a("a") + a("b") + a("ab")
// P("abc") = 
//     a("abc") + 
//     [a("bc") + 1 + 1]->(P("bc")) + 
//     [a("ab") + 1 + 1]->(P("ab")) 
//     - 1
//
// P(1234) = a(1234) + 
//     a(123) + a(12) + a(23) + a(1) + a(2) + a(3) + = P(123)
//     a(234) + a(34) + a(4)
//
//
// P(123....n) = P(123...n-1) + a(123..n) + a(23..n) + a(3...n) + ... a(n)
//

// F(n) = SUM[i=1,n] n!/(n-i)!i! * (n-i)


//
// a("ab") * 3 + a("b") * 6 + a("a")
//
// P("ab")  = a("ab") + a("b") + a("a")
//
// P("abbb") = 
//     a("ab") + 
//     a("ab") + a("b") + 
//     a("ab") + a("b") + a("b") + 
//     a("a") + a("b") + a("b") + a("b")
    

use std::collections::HashSet;

struct Solution;

pub fn appeal(s: &str) -> usize {
    let mut letters: HashSet<char> = HashSet::new();
    for c in s.chars() {
        letters.insert(c);
    }
    letters.len()

} 
pub fn total_appeal(n: usize, s: String) -> usize {
    if n == 1 {
        return 1;
    }
    
    let mut sum = 0usize;       
    
    for i in 0..n {
        sum = sum + appeal(&s[i..n]);
    }

    return total_appeal(n-1, s) + sum;
}

impl Solution {

    pub fn appeal_sum(s: String) -> i64 {
        return total_appeal(s.len(), s) as i64;
    }
}

fn main() {
    println!("P(abbca) = {}", Solution::appeal_sum("rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrpppppppkkkkkkkkkkkkkkhdddddddduuuuuuuuuuuuuuuuuuuuuuuuuujjjjjjjwwwwwwwwwwwwwwwwwwwwwwwwwwwwwggggggggggggggggggnnnnnnmmmmmmmmmmmmmwwwwwwwwwwwwwwwwwwwwwwwwvvvvvvvvvvvvvvvvvvviiiiiiiiiiiiiiiiiiiiiiiiiiitttttttttttiiiiiiiisssssssssssssssssssssssssshhhhhhhhhhhhhhhhhhhhhhhhhjjjjjjjjjjjxxxxxxzzzzzzzzzzzzzzzzeeeeeeeeeeeeeeeeellllllllllllllllllllccccccccccccccccccccccccggggggggggggggggggccccccccccccccctttttttmmmmkkkkkkkkkkkkkkkkkkkkkkkkkksssssssssbbbbbbbbbbqqqqqqqqqqqmmmmmmmmmmmmmmmmllllllllllllooooooooooossssssyyyyyysssssssssssssssssnnnnnnnnnnnnnnnnn".to_string()));
    println!("P(code) = {}", Solution::appeal_sum("code".to_string()));
}

