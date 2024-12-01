use lin_ssg_core::LinSsg;
use transc::TranscFn;

mod transc;

pub fn install(ssg: &mut LinSsg) {
    ssg.register_symbol("Phonemic");
    ssg.register_symbol("Phonetic");
    ssg.register_symbol("Graphemic");
    ssg.register_symbol("GraphemicRaw");
    ssg.register_symbol("Morphophonemic");
    ssg.register_const("GraRaw", "GraphemicRaw");
    ssg.register_const("Morpho", "Morphophonemic");
    ssg.register_fn("transc", TranscFn);
}
