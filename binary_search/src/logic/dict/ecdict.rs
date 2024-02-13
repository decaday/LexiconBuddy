use crate::logic::dict::dict::*;

struct EcdictWord {
    word: String,
    phonetic: String,
    translation: String,
    tag: String,
}

impl EcdictWord {
    fn build(dict: Dict, index: u64) -> Result<Self, DictError> {
        todo!()
    }
}
