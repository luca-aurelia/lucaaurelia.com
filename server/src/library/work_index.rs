use assets::ImageAsset;
use library_of_babel::date::Month;
use library_of_babel::date::Year;
use once_cell::sync::Lazy;

pub struct WorkIndex {
    pub most_light_speaks_sunish: Work,
    pub when_sun_and_dirt: Work,
    pub thistle_bright_morning: Work,
    pub into_my_bedroom_quietness_a_bird_is_shouting: Work,
    pub my_eyes_upon_the_sun_upon_my_face: Work,
    pub the_sun_not_setting_turned_to_moss: Work,
    pub quiet_and_watching_the_clouds_flock: Work,
    pub evening_cracking_like_an_egg: Work,
    pub taking_a_breath_from_the_night_sky: Work,
    pub three_dimensions_of_birdsong: Work,
    pub digging_up_night_from_the_garden: Work,
    pub green_like_no_grass_is_green: Work,
    pub throwing_sound_into_the_valley: Work,
    pub metal_tendons_of_mountains_metal_skins_of_lakes: Work,
    pub glen_of_the_birches: Work,
    pub walking_the_grounds_of_koya_mountain: Work,
    pub a_hill_of_seven_colors: Work,
    pub walls_fall: Work,
}

impl WorkIndex {
    pub fn works(&self) -> Vec<&Work> {
        vec![
            &self.most_light_speaks_sunish,
            &self.when_sun_and_dirt,
            &self.thistle_bright_morning,
            &self.into_my_bedroom_quietness_a_bird_is_shouting,
            &self.my_eyes_upon_the_sun_upon_my_face,
            &self.the_sun_not_setting_turned_to_moss,
            &self.quiet_and_watching_the_clouds_flock,
            &self.evening_cracking_like_an_egg,
            &self.taking_a_breath_from_the_night_sky,
            &self.three_dimensions_of_birdsong,
            &self.digging_up_night_from_the_garden,
            &self.green_like_no_grass_is_green,
            &self.throwing_sound_into_the_valley,
            &self.metal_tendons_of_mountains_metal_skins_of_lakes,
            &self.glen_of_the_birches,
            &self.walking_the_grounds_of_koya_mountain,
            &self.a_hill_of_seven_colors,
            &self.walls_fall,
        ]
    }
}

pub struct Work {
    pub name: &'static str,
    pub cropped_preview_image: ImageAsset,
    pub image: ImageAsset,
    pub year: Year,
    pub month: Month,
    pub kind: Kind,
    pub accent_color: &'static str,
}

enum Kind {
    Art,
    Writing,
    SpecialProjects,
}

pub static WORK_INDEX: Lazy<WorkIndex> = Lazy::new(|| WorkIndex {
    most_light_speaks_sunish: Work {
        name: "most light speaks sunish",
        year: Year::new(2024),
        month: Month::new(7),
        kind: Kind::Art,
        accent_color: "rgb(231, 143, 129)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/most light speaks sunish preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/most light speaks sunish.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    when_sun_and_dirt: Work {
        name: "when sun and dirt",
        year: Year::new(2024),
        month: Month::new(5),
        kind: Kind::Art,
        accent_color: "rgb(127, 157, 201)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/when sun and dirt preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/when sun and dirt.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    thistle_bright_morning: Work {
        name: "thistle bright morning: haiku by taneda santōka",
        year: Year::new(2024),
        month: Month::new(5),
        kind: Kind::SpecialProjects,
        accent_color: "rgb(224, 115, 78)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/thistle bright morning preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/thistle bright morning.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    into_my_bedroom_quietness_a_bird_is_shouting: Work {
        name: "into my bedroom quietness a bird is shouting",
        year: Year::new(2023),
        month: Month::new(11),
        kind: Kind::Art,
        accent_color: "rgb(211, 157, 89)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/into my bedroom quietness a bird is shouting preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/into my bedroom quietness a bird is shouting.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    my_eyes_upon_the_sun_upon_my_face: Work {
        name: "my eyes upon the sun upon my face",
        year: Year::new(2023),
        month: Month::new(07),
        kind: Kind::Art,
        accent_color: "rgb(178, 199, 199)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/my eyes upon the sun upon my face preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/my eyes upon the sun upon my face.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    the_sun_not_setting_turned_to_moss: Work {
        name: "the sun not setting turned to moss",
        year: Year::new(2022),
        month: Month::new(11),
        kind: Kind::Art,
        accent_color: "rgb(130, 151, 127)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/the sun not setting turned to moss preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/the sun not setting turned to moss.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    quiet_and_watching_the_clouds_flock: Work {
        name: "quiet and watching the clouds flock",
        year: Year::new(2022),
        month: Month::new(10),
        kind: Kind::Art,
        accent_color: "rgb(119, 214, 210)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/quiet and watching the clouds flock preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/quiet and watching the clouds flock.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    evening_cracking_like_an_egg: Work {
        name: "evening cracking like an egg",
        year: Year::new(2022),
        month: Month::new(09),
        kind: Kind::Art,
        accent_color: "rgb(92, 42, 29)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/evening cracking like an egg preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/evening cracking like an egg.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    taking_a_breath_from_the_night_sky: Work {
        name: "taking a breath from the night sky",
        year: Year::new(2022),
        month: Month::new(09),
        kind: Kind::Art,
        accent_color: "rgb(51, 46, 72)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/taking a breath from the night sky preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/taking a breath from the night sky.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    three_dimensions_of_birdsong: Work {
        name: "three dimensions of birdsong",
        year: Year::new(2022),
        month: Month::new(09),
        kind: Kind::Art,
        accent_color: "rgb(177, 107, 90)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/three dimensions of birdsong preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/three dimensions of birdsong.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    digging_up_night_from_the_garden: Work {
        name: "digging up night from the garden",
        year: Year::new(2022),
        month: Month::new(07),
        kind: Kind::Art,
        accent_color: "rgb(198, 185, 227)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/digging up night from the garden preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/digging up night from the garden.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    green_like_no_grass_is_green: Work {
        name: "green like no grass is green",
        year: Year::new(2022),
        month: Month::new(06),
        kind: Kind::Art,
        accent_color: "rgb(101, 142, 92)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/green like no grass is green preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/green like no grass is green.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    throwing_sound_into_the_valley: Work {
        name: "throwing sound into the valley",
        year: Year::new(2022),
        month: Month::new(06),
        kind: Kind::Art,
        accent_color: "rgb(239, 166, 173)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/throwing sound into the valley preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/throwing sound into the valley.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    metal_tendons_of_mountains_metal_skins_of_lakes: Work {
        name: "metal tendons of mountains, metal skins of lakes",
        year: Year::new(2022),
        month: Month::new(05),
        kind: Kind::Art,
        accent_color: "rgb(220, 219, 215)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/metal tendons of mountains, metal skins of lakes preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/metal tendons of mountains, metal skins of lakes.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    glen_of_the_birches: Work {
        name: "glen of the birches",
        year: Year::new(2021),
        month: Month::new(11),
        kind: Kind::Art,
        accent_color: "rgb(127, 97, 53)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/glen of the birches preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/glen of the birches.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    walking_the_grounds_of_koya_mountain: Work {
        name: "walking the grounds of kōya mountain",
        year: Year::new(2021),
        month: Month::new(09),
        kind: Kind::Art,
        accent_color: "rgb(120, 119, 115)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/walking the grounds of kōya mountain preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/walking the grounds of kōya mountain.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    a_hill_of_seven_colors: Work {
        name: "a hill of seven colors",
        year: Year::new(2021),
        month: Month::new(06),
        kind: Kind::Art,
        accent_color: "rgb(178, 104, 110)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/a hill of seven colors preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/a hill of seven colors.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
    walls_fall: Work {
        name: "walls fall",
        year: Year::new(2019),
        month: Month::new(04),
        kind: Kind::Art,
        accent_color: "rgb(239, 75, 134)",
        cropped_preview_image: assets::include_image!(
            path_to_image: "server/src/assets/images/walls fall preview.png",
            alt: "",
            placeholder: automatic_color,
        ),
        image: assets::include_image!(
            path_to_image: "server/src/assets/images/walls fall.png",
            alt: "",
            placeholder: automatic_color,
        ),
    },
});
