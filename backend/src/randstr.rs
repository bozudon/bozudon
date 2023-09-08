use base64::{engine::general_purpose::URL_SAFE_NO_PAD as base64, Engine};
use rand::{thread_rng, RngCore};

pub fn gen(prefix: Option<String>) -> String {
    let mut rand_bytes = [0u8; 32];
    thread_rng().fill_bytes(&mut rand_bytes);
    let encoded = base64.encode(rand_bytes);
    match prefix {
        Some(prefix) => prefix + "_" + &encoded,
        None => encoded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_no_pad_and_no_plus() {
        for _ in 0..20 {
            let rand_str = gen(None);
            assert!(!rand_str.contains('='));
            assert!(!rand_str.contains('+'));
        }
    }

    #[test]
    fn gen_prefixed() {
        let rand_str = gen(Some(String::from("hoge")));
        assert!(rand_str.starts_with("hoge"));
    }
}
