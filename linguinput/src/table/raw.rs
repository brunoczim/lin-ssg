type Code = &'static str;
type Char = &'static str;

pub const TABLE: &'static [(Code, Char)] = &[
    /* Signs and Punctuation */
    ("<", "⟨"),
    (">", "⟩"),
    /* Generic Notational */
    ("_0", "₀"),
    ("_1", "₁"),
    ("_2", "₂"),
    ("_3", "₃"),
    ("_4", "₄"),
    ("_5", "₅"),
    ("_6", "₆"),
    ("_7", "₇"),
    ("_8", "₈"),
    ("_9", "₉"),
    ("^0", "⁰"),
    ("^1", "¹"),
    ("^2", "²"),
    ("^3", "³"),
    ("^4", "⁴"),
    ("^5", "⁵"),
    ("^6", "⁶"),
    ("^7", "⁷"),
    ("^8", "⁸"),
    ("^9", "⁹"),
    ("_a", "ₐ"),
    ("_e", "ₑ"),
    ("_o", "ₒ"),
    ("_h", "ₕ"),
    /* Miscellaneous */
    ("0", "∅"),
    /* IPA Tone */
    ("1", "˩"),
    ("2", "˧"),
    ("3", "˧"),
    ("4", "˦"),
    ("5", "˥"),
    /* IPA Vowel Letters */
    ("a", "ɐ"),
    ("ae", "æ"),
    ("OE", "ɶ"),
    ("aa", "ɑ"),
    ("ao", "ɒ"),
    ("e", "ɛ"),
    ("oe", "œ"),
    ("eA", "ɜ"),
    ("oA", "ɞ"),
    ("A", "ʌ"),
    ("o", "ɔ"),
    ("ea", "ə"),
    ("oi", "ø"),
    ("ia", "ɘ"),
    ("io", "ɵ"),
    ("oa", "ɤ"),
    ("I", "ɪ"),
    ("Y", "ʏ"),
    ("U", "ʊ"),
    ("i", "ɨ"),
    ("u", "ʉ"),
    ("ua", "ɯ"),
    /* IPA Consonant Letters */
    ("sr", "ʂ"),
    ("sj", "ɕ"),
    /* IP Phonation */
    ("^h", "ʰ"),
    ("^hv", "ʱ"),
    ("^=", "˭"),
    /* IPA Diacritics */
    ("#.", "\u{32f}"),
    ("#^.", "\u{311}"),
    ("#^_", "\u{361}"),
    ("#_", "\u{35c}"),
];
