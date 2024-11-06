use crate::web::model::corporation::CorpCardData;

pub const DISCORD_URL: &str = "https://discord.gg/WvA8Vb9C7D";
pub const APPLICATIONS_URL: &str = "https://apply.autumn-order.com";
pub const FEATURED_VIDEO: &str = "https://www.youtube.com/embed/AdfFnTt2UT0?si=x3rGt9pHRJHZ9g8i";

pub const CORPORATIONS: &[CorpCardData] = &[
    CorpCardData {
        name: "The Order of Autumn",
        corporation_id: 98785281,
        location: "Nullsec",
        members: 0,
        ships_destroyed: 0,
        cta_text: "Begin Your Journey in Nullsec",
    },
    CorpCardData {
        name: "Autumn Highsec Division",
        corporation_id: 98784256,
        location: "Highsec",
        members: 0,
        ships_destroyed: 0,
        cta_text: "Begin Your Journey in Highsec",
    },
];
