
use crate::dict::dict::*;

struct EcdictWord {
    word: String,
    phonetic: String,
    translation: String,
    tag: String,
}

impl EcdictWord {
    fn build(dict: Dict,index: u64) -> Result<EcdictWord, DictError> {
        Ok(())
    }
}