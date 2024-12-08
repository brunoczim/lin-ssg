type Code = &'static str;
type Char = &'static str;

pub const TABLE: &'static [(Code, Char)] = &[
    /* Signs and Punctuation */
    ("<", "⟨"),
    (">", "⟩"),
    /* Generic Notational */
    ("_ 0", "₀"),
    ("_ 1", "₁"),
    ("_ 2", "₂"),
    ("_ 3", "₃"),
    ("_ 4", "₄"),
    ("_ 5", "₅"),
    ("_ 6", "₆"),
    ("_ 7", "₇"),
    ("_ 8", "₈"),
    ("_ 9", "₉"),
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
    ("_ a", "ₐ"),
    ("_ e", "ₑ"),
    ("_ o", "ₒ"),
    ("_ h", "ₕ"),
    /* Miscellaneous */
    ("0", "∅"),
    ("t", "þ"),
    /* IPA Tone */
    ("1", "˩"),
    ("2", "˨"),
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
    ("ph", "ɸ"),
    ("b", "β"),
    ("g", "ɣ"),
    ("d", "ð"),
    ("th", "θ"),
    ("lo", "ɫ"),
    ("r", "ɹ"),
    ("rd", "ɾ"),
    ("sr", "ʂ"),
    ("sc", "ʃ"),
    ("sj", "ɕ"),
    ("c", "ç"),
    ("j", "ʝ"),
    ("J", "ɟ"),
    ("x", "χ"),
    ("R", "ʀ"),
    ("Rh", "ʁ"),
    ("y", "ɥ"),
    /* IPA length */
    (":", "ː"),
    (".", "ˑ"),
    ("#.", "\u{32f}"),
    ("#^.", "\u{311}"),
    /* IPA Prosody */
    ("'", "ˈ"),
    (",", "ˌ"),
    /* IP Phonation */
    ("^h", "ʰ"),
    ("^hv", "ʱ"),
    ("^=", "˭"),
    /* IPA Articulation */
    ("#:", "\u{308}"),
    /* IPA Coarticulation */
    ("#^", "\u{361}"),
    ("#_ ", "\u{35c}"),
    ("^w", "ʷ"),
    ("^j", "ʲ"),
    ("^-g", "ˠ"),
    ("^-y", "ᶣ"),
];
