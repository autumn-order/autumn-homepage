#[derive(PartialEq)]
pub struct CorpCardData {
    pub name: &'static str,
    pub corporation_id: u64,
    pub location: &'static str,
    pub cta_text: &'static str,
}

pub struct FAQEntry {
    pub question: &'static str,
    pub answer: &'static str,
}

pub const DISCORD_URL: &str = "https://discord.gg/WvA8Vb9C7D";
pub const GITHUB_URL: &str = "https://github.com/autumn-order";
pub const BLACK_ROSE_WEBSITE_URL: &str = "https://black-rose.space";
pub const APPLICATIONS_URL: &str = "https://apply.autumn-order.com";
pub const FEATURED_VIDEO: &str = "https://www.youtube.com/embed/AdfFnTt2UT0?si=x3rGt9pHRJHZ9g8i";

pub const AUTUMN_ORDER_CORP_INFO: CorpCardData = CorpCardData {
    name: "The Order of Autumn",
    corporation_id: 98785281,
    location: "Nullsec",
    cta_text: "Begin Your Journey in Nullsec",
};

pub const AUTUMN_HIGHSEC_CORP_INFO: CorpCardData = CorpCardData {
    name: "Autumn Highsec Division",
    corporation_id: 98784256,
    location: "Highsec",
    cta_text: "Begin Your Journey in Highsec",
};

pub const FAQ: &[FAQEntry] = &[
    FAQEntry {
        question: "What is Autumn's main focus?",
        answer: "Our primary focus is newer players & helping them grow, outside of that we focus on PvP with a mix of industry & PvE on the side.",
    },
    FAQEntry {
        question: "What is Autumn's ultimate goal?",
        answer: "Our ultimate goal is to make the new player experience in EVE more accessible and engaging. We also aim to build a tight knit community of like minded players who share in our dream of helping newer players grow.",
    },
    FAQEntry {
        question: "Is PvP mandatory?",
        answer: "While we have no monthly fleet participation minimums we do require nullsec member attendance for the occasional Call to Action fleets which involves a highly important strategic objective we needs all hands on deck to secure, this ONLY applies if you are online & in-game, don't lose sleep over CTAs, real life always comes first."
    },
    FAQEntry {
        question: "Where is Autumn located?",
        answer: "Our nullsec corporation, The Order of Autumn, is located in the region of Pure Blind. Our highsec corporation, Autumn Highsec Division, is located out of Torrinos in the region of Lonetrek."
    },
    FAQEntry {
        question: "What do you use for voice chat?",
        answer: "We primarily use Discord for anything corporation or alliance level. Larger scale coalition fleets use Mumble because multiple channels that can be linked together are essential for large fleets."
    },
    FAQEntry {
        question: "What are the requirements to join?",
        answer: "To join you simply need to put an application in via this Website, you will also need to add your character to an application called SeAT which we use for background checks to mitigate the risk of spies. Our application process will walk you through how to do all of that."
    },
    FAQEntry {
        question: "Do I need a microphone to join?",
        answer: "No, you do not need a microphone to join. However, we do recommend having one to participate in voice comms."
    },
    FAQEntry {
        question: "How does Autumn Highsec Division compare to The Order of Autumn?",
        answer: "Autumn Highsec Division is a lot more self-paced, you have access to our community and resources to grow and is intended to ultimately be a starting point before nullsec. The Order of Autumn is a lot more community driven as in nullsec space you must work together to keep your space secure & keep your market churning."
    }

];

pub static EVE_LEGAL_STATEMENT: &str = "EVE Online and the EVE logo are the registered trademarks of CCP hf. All rights are reserved worldwide. All other trademarks are the property of their respective owners. EVE Online, the EVE logo, EVE and all associated logos and designs are the intellectual property of CCP hf. All artwork, screenshots, characters, vehicles, storylines, world facts or other recognizable features of the intellectual property relating to these trademarks are likewise the intellectual property of CCP hf. CCP hf. has granted permission to Autumn to use EVE Online and all associated logos and designs for promotional and information purposes on its website but does not endorse, and is not in any way affiliated with, Autumn. CCP is in no way responsible for the content on or functioning of this website, nor can it be liable for any damage arising from the use of this website.";
