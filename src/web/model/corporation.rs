#[derive(PartialEq)]
pub struct CorpCardData {
    pub name: &'static str,
    pub corporation_id: u64,
    pub location: &'static str,
    pub members: usize,
    pub ships_destroyed: usize,
    pub cta_text: &'static str,
}
