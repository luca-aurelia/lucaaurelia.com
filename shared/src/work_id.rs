use serde::{Deserialize, Serialize};

// If you change this, also update WorkIndex in work.rs in the server crate.
#[derive(Eq, Clone, Copy, PartialEq, Debug, Serialize, Deserialize, enum_iterator::Sequence)]
pub enum WorkId {
    MostLightSpeaksSunish,
    WhenSunAndDirt,
    Santoka,
    IntoMyBedroomQuietnessABirdIsShouting,
    MyEyesUponTheSunUponMyFace,
    TheSunNotSettingTurnedToMoss,
    QuietAndWatchingTheCloudsFlock,
    EveningCrackingLikeAnEgg,
    TakingABreathFromTheNightSky,
    ThreeDimensionsOfBirdsong,
    DiggingUpNightFromTheGarden,
    GreenLikeNoGrassIsGreen,
    ThrowingSoundIntoTheValley,
    MetalTendonsOfMountainsMetalSkinsOfLakes,
    GlenOfTheBirches,
    WalkingTheGroundsOfKoyaMountain,
    AHillOfSevenColors,
    WallsFall,
}

impl WorkId {
    pub fn all() -> impl Iterator<Item = WorkId> {
        enum_iterator::all::<WorkId>()
    }

    pub fn url_slug(&self) -> &'static str {
        match self {
            WorkId::MostLightSpeaksSunish => "most-light-speaks-sunish",
            WorkId::WhenSunAndDirt => "when-sun-and-dirt",
            WorkId::Santoka => "santoka",
            WorkId::IntoMyBedroomQuietnessABirdIsShouting => {
                "into-my-bedroom-quietness-a-bird-is-shouting"
            }
            WorkId::MyEyesUponTheSunUponMyFace => "my-eyes-upon-the-sun-upon-my-face",
            WorkId::TheSunNotSettingTurnedToMoss => "the-sun-not-setting-turned-to-moss",
            WorkId::QuietAndWatchingTheCloudsFlock => "quiet-and-watching-the-clouds-flock",
            WorkId::EveningCrackingLikeAnEgg => "evening-cracking-like-an-egg",
            WorkId::TakingABreathFromTheNightSky => "taking-a-breath-from-the-night-sky",
            WorkId::ThreeDimensionsOfBirdsong => "three-dimensions-of-birdsong",
            WorkId::DiggingUpNightFromTheGarden => "digging-up-night-from-the-garden",
            WorkId::GreenLikeNoGrassIsGreen => "green-like-no-grass-is-green",
            WorkId::ThrowingSoundIntoTheValley => "throwing-sound-into-the-valley",
            WorkId::MetalTendonsOfMountainsMetalSkinsOfLakes => {
                "metal-tendons-of-mountains-metal-skins-of-lakes"
            }
            WorkId::GlenOfTheBirches => "glen-of-the-birches",
            WorkId::WalkingTheGroundsOfKoyaMountain => "walking-the-grounds-of-koya-mountain",
            WorkId::AHillOfSevenColors => "a-hill-of-seven-colors",
            WorkId::WallsFall => "walls-fall",
        }
    }
}
