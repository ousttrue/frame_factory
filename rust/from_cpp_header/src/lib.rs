mod translation_unit;
pub use translation_unit::*;

pub fn run(args: &[String]) -> Result<TranslationUnit, Error> {
    TranslationUnit::parse(args[0].as_str())
}
