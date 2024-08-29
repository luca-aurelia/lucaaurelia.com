use assets::ImageAsset;
use library_of_babel::date::{Month, Year};
use maud::Markup;

pub use shared::work_id::WorkId;

pub struct Work {
    pub name: &'static str,
    pub id: WorkId,
    pub cropped_preview_image: ImageAsset,
    pub image: ImageAsset,
    pub year: Year,
    pub month: Month,
    pub kind: Kind,
    pub accent_color: &'static str,
    pub page_fn: fn() -> Markup,
}

pub enum Kind {
    Art,
    Writing,
    SpecialProjects,
}

impl Work {
    pub fn all() -> impl Iterator<Item = &'static Work> {
        WorkId::all().map(Work::from_id)
    }

    pub fn from_id(id: WorkId) -> &'static Work {
        match id {
            WorkId::MostLightSpeaksSunish => &crate::routes::works::most_light_speaks_sunish::WORK,
            WorkId::WhenSunAndDirt => &crate::routes::works::when_sun_and_dirt::WORK,
            WorkId::Santoka => &crate::routes::santoka::WORK,
            WorkId::IntoMyBedroomQuietnessABirdIsShouting => {
                &crate::routes::works::into_my_bedroom_quietness_a_bird_is_shouting::WORK
            }
            WorkId::MyEyesUponTheSunUponMyFace => {
                &crate::routes::works::my_eyes_upon_the_sun_upon_my_face::WORK
            }
            WorkId::TheSunNotSettingTurnedToMoss => {
                &crate::routes::works::the_sun_not_setting_turned_to_moss::WORK
            }
            WorkId::QuietAndWatchingTheCloudsFlock => {
                &crate::routes::works::quiet_and_watching_the_clouds_flock::WORK
            }
            WorkId::EveningCrackingLikeAnEgg => {
                &crate::routes::works::evening_cracking_like_an_egg::WORK
            }
            WorkId::TakingABreathFromTheNightSky => {
                &crate::routes::works::taking_a_breath_from_the_night_sky::WORK
            }
            WorkId::ThreeDimensionsOfBirdsong => {
                &crate::routes::works::three_dimensions_of_birdsong::WORK
            }
            WorkId::DiggingUpNightFromTheGarden => {
                &crate::routes::works::digging_up_night_from_the_garden::WORK
            }
            WorkId::GreenLikeNoGrassIsGreen => {
                &crate::routes::works::green_like_no_grass_is_green::WORK
            }
            WorkId::ThrowingSoundIntoTheValley => {
                &crate::routes::works::throwing_sound_into_the_valley::WORK
            }
            WorkId::MetalTendonsOfMountainsMetalSkinsOfLakes => {
                &crate::routes::works::metal_tendons_of_mountains_metal_skins_of_lakes::WORK
            }
            WorkId::GlenOfTheBirches => &crate::routes::works::glen_of_the_birches::WORK,
            WorkId::WalkingTheGroundsOfKoyaMountain => {
                &crate::routes::works::walking_the_grounds_of_koya_mountain::WORK
            }
            WorkId::AHillOfSevenColors => &crate::routes::works::a_hill_of_seven_colors::WORK,
            WorkId::WallsFall => &crate::routes::works::walls_fall::WORK,
        }
    }

    pub fn page(&self) -> Markup {
        (self.page_fn)()
    }
}

// pub static WORK_INDEX: Lazy<WorkIndex> = Lazy::new(|| WorkIndex {
//     most_light_speaks_sunish: Work {
//         name: "most light speaks sunish",
//         id: WorkId::MostLightSpeaksSunish,
//         year: Year::new(2024),
//         month: Month::new(7),
//         kind: Kind::Art,
//         accent_color: "rgb(231, 143, 129)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/most light speaks sunish preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/most light speaks sunish.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     when_sun_and_dirt: Work {
//         name: "when sun and dirt",
//         id: WorkId::WhenSunAndDirt,
//         year: Year::new(2024),
//         month: Month::new(5),
//         kind: Kind::Art,
//         accent_color: "rgb(127, 157, 201)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/when sun and dirt preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/when sun and dirt.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     thistle_bright_morning: Work {
//         name: "thistle bright morning: haiku by taneda santōka",
//         id: WorkId::Santoka,
//         year: Year::new(2024),
//         month: Month::new(5),
//         kind: Kind::SpecialProjects,
//         accent_color: "rgb(224, 115, 78)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/thistle bright morning preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/thistle bright morning.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     into_my_bedroom_quietness_a_bird_is_shouting: Work {
//         name: "into my bedroom quietness a bird is shouting",
//         id: WorkId::IntoMyBedroomQuietnessABirdIsShouting,
//         year: Year::new(2023),
//         month: Month::new(11),
//         kind: Kind::Art,
//         accent_color: "rgb(211, 157, 89)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/into my bedroom quietness a bird is shouting preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/into my bedroom quietness a bird is shouting.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     my_eyes_upon_the_sun_upon_my_face: Work {
//         name: "my eyes upon the sun upon my face",
//         id: WorkId::MyEyesUponTheSunUponMyFace,
//         year: Year::new(2023),
//         month: Month::new(07),
//         kind: Kind::Art,
//         accent_color: "rgb(178, 199, 199)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/my eyes upon the sun upon my face preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/my eyes upon the sun upon my face.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     the_sun_not_setting_turned_to_moss: Work {
//         name: "the sun not setting turned to moss",
//         id: WorkId::TheSunNotSettingTurnedToMoss,
//         year: Year::new(2022),
//         month: Month::new(11),
//         kind: Kind::Art,
//         accent_color: "rgb(130, 151, 127)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/the sun not setting turned to moss preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/the sun not setting turned to moss.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     quiet_and_watching_the_clouds_flock: Work {
//         name: "quiet and watching the clouds flock",
//         id: WorkId::QuietAndWatchingTheCloudsFlock,
//         year: Year::new(2022),
//         month: Month::new(10),
//         kind: Kind::Art,
//         accent_color: "rgb(119, 214, 210)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/quiet and watching the clouds flock preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/quiet and watching the clouds flock.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     evening_cracking_like_an_egg: Work {
//         name: "evening cracking like an egg",
//         id: WorkId::EveningCrackingLikeAnEgg,
//         year: Year::new(2022),
//         month: Month::new(09),
//         kind: Kind::Art,
//         accent_color: "rgb(92, 42, 29)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/evening cracking like an egg preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/evening cracking like an egg.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     taking_a_breath_from_the_night_sky: Work {
//         name: "taking a breath from the night sky",
//         id: WorkId::TakingABreathFromTheNightSky,
//         year: Year::new(2022),
//         month: Month::new(09),
//         kind: Kind::Art,
//         accent_color: "rgb(51, 46, 72)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/taking a breath from the night sky preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/taking a breath from the night sky.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     three_dimensions_of_birdsong: Work {
//         name: "three dimensions of birdsong",
//         id: WorkId::ThreeDimensionsOfBirdsong,
//         year: Year::new(2022),
//         month: Month::new(09),
//         kind: Kind::Art,
//         accent_color: "rgb(177, 107, 90)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/three dimensions of birdsong preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/three dimensions of birdsong.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     digging_up_night_from_the_garden: Work {
//         name: "digging up night from the garden",
//         id: WorkId::DiggingUpNightFromTheGarden,
//         year: Year::new(2022),
//         month: Month::new(07),
//         kind: Kind::Art,
//         accent_color: "rgb(198, 185, 227)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/digging up night from the garden preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/digging up night from the garden.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     green_like_no_grass_is_green: Work {
//         name: "green like no grass is green",
//         id: WorkId::GreenLikeNoGrassIsGreen,
//         year: Year::new(2022),
//         month: Month::new(06),
//         kind: Kind::Art,
//         accent_color: "rgb(101, 142, 92)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/green like no grass is green preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/green like no grass is green.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     throwing_sound_into_the_valley: Work {
//         name: "throwing sound into the valley",
//         id: WorkId::ThrowingSoundIntoTheValley,
//         year: Year::new(2022),
//         month: Month::new(06),
//         kind: Kind::Art,
//         accent_color: "rgb(239, 166, 173)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/throwing sound into the valley preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/throwing sound into the valley.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     metal_tendons_of_mountains_metal_skins_of_lakes: Work {
//         name: "metal tendons of mountains, metal skins of lakes",
//         id: WorkId::MetalTendonsOfMountainsMetalSkinsOfLakes,
//         year: Year::new(2022),
//         month: Month::new(05),
//         kind: Kind::Art,
//         accent_color: "rgb(220, 219, 215)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/metal tendons of mountains, metal skins of lakes preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/metal tendons of mountains, metal skins of lakes.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     glen_of_the_birches: Work {
//         name: "glen of the birches",
//         id: WorkId::GlenOfTheBirches,
//         year: Year::new(2021),
//         month: Month::new(11),
//         kind: Kind::Art,
//         accent_color: "rgb(127, 97, 53)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/glen of the birches preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/glen of the birches.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     walking_the_grounds_of_koya_mountain: Work {
//         name: "walking the grounds of kōya mountain",
//         id: WorkId::WalkingTheGroundsOfKoyaMountain,
//         year: Year::new(2021),
//         month: Month::new(09),
//         kind: Kind::Art,
//         accent_color: "rgb(120, 119, 115)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/walking the grounds of kōya mountain preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/walking the grounds of kōya mountain.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     a_hill_of_seven_colors: Work {
//         name: "a hill of seven colors",
//         id: WorkId::AHillOfSevenColors,
//         year: Year::new(2021),
//         month: Month::new(06),
//         kind: Kind::Art,
//         accent_color: "rgb(178, 104, 110)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/a hill of seven colors preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/a hill of seven colors.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
//     walls_fall: Work {
//         name: "walls fall",
//         id: WorkId::WallsFall,
//         year: Year::new(2019),
//         month: Month::new(04),
//         kind: Kind::Art,
//         accent_color: "rgb(239, 75, 134)",
//         cropped_preview_image: assets::include_image!(
//             path_to_image: "server/src/assets/images/walls fall preview.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//         image: assets::include_image!(
//             path_to_image: "server/src/assets/images/walls fall.png",
//             alt: "",
//             placeholder: automatic_color,
//         ),
//     },
// });
