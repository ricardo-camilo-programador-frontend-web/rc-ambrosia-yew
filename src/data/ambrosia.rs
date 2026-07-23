/// All data about the Ambrosia apple variety.
/// Source: Wikipedia, OrangePippin, OrganicAmbrosiaApple.ca

pub const NAME: &str = "Ambrosia";
pub const EMOJI: &str = "🍎";
pub const TAGLINE: &str = "Food of the Gods";

pub const ORIGIN_STORY: &str = "In the early 1990s, in the sun-drenched Similkameen Valley of British Columbia, Canada, the Mennell family noticed something unusual growing among their orchard rows. A chance seedling — neither planted nor planned — had sprung up on its own, bearing fruit unlike any they had seen before. Wilfrid Mennell, struck by the apple's honeyed sweetness and juicy crunch, named it 'Ambrosia' — the food of the gods from Greek mythology. Pickers in the orchard, who seldom eat apples while working, loved the fruit so much they stripped the tree bare before the rest of the harvest was ready. It was a born star.";

pub const PARENTAGE_FATHER: &str = "Starking Delicious";
pub const PARENTAGE_MOTHER: &str = "Golden Delicious";

pub const POLLINATORS: &[&str] = &["Cortland", "Fuji", "Granny Smith"];

pub const CHARACTERISTICS: &[(&str, &str, &str)] = &[
    (
        "🎨",
        "Appearance",
        "Bi-colored with glossy red over creamy yellow. Distinctive conical shape.",
    ),
    (
        "📏",
        "Size",
        "Medium to large, 6.8–7.5 cm in diameter, averaging 215 grams.",
    ),
    (
        "🍯",
        "Taste",
        "Honeyed sweetness with low acidity. Delicate flavor notes reminiscent of pear.",
    ),
    (
        "🥖",
        "Texture",
        "Cream-colored flesh that is firm, crisp, and satisfyingly juicy.",
    ),
    (
        "⏳",
        "Anti-Browning",
        "Resists oxidation remarkably well — stays fresh-looking in salads and displays.",
    ),
    (
        "🌱",
        "Low Ethylene",
        "Produces very little ethylene, meaning it ripens slowly and stores longer.",
    ),
];

pub const MYTHOLOGY_QUOTE: &str = "As the sapling grew, it bore a few beautiful apples. Wilfrid named it Ambrosia — food of the gods — for its honeyed sweetness and juicy crunch!";

pub const NUTRITION: &[(&str, &str)] = &[
    ("52", "kcal / 100g"),
    ("2.4", "g fiber"),
    ("4.6", "mg vitamin C"),
    ("Low", "ethylene producer"),
    ("High", "antioxidants"),
];

pub const REGIONS: &[(&str, &str, &str)] = &[
    (
        "🇨🇦",
        "British Columbia",
        "Primary growing region — the birthplace of Ambrosia",
    ),
    ("🇨🇦", "Ontario", "Major Canadian production"),
    ("🇨🇦", "Nova Scotia", "Expanding Canadian production"),
    ("🇺🇸", "Washington State", "Primary US growing region"),
    ("🇺🇸", "New York", "Significant US production"),
    ("🇨🇱", "Chile", "South American production"),
    ("🇳🇿", "New Zealand", "Southern hemisphere supply"),
    ("🇳🇱", "Netherlands", "European production"),
    ("🇮🇹", "Italy", "European production"),
];

pub const CULINARY_USES: &[(&str, &str, &str)] = &[
    ("🍽", "Fresh Eating", "The ideal snacking apple — sweet, low acid, and the satisfying crunch makes every bite a pleasure."),
    ("🥗", "Salads", "Resists browning better than almost any other variety. Stays beautiful in salads for hours."),
    ("🍰", "Baking", "Holds its shape under heat. Excellent for pies, tarts, and baked desserts."),
    ("🍺", "Cider & Juice", "The honeyed sweetness translates beautifully into fresh juice and artisanal ciders."),
    ("🧀", "Cheese Pairings", "Pairs exceptionally well with aged cheddar, creamy brie, and sharp blue cheeses."),
];

pub const TIMELINE_EVENTS: &[(&str, &str, &str)] = &[
    ("~1980s", "A Chance Discovery", "A mysterious seedling appears in the Mennell family orchard in Cawston, British Columbia."),
    ("Early 1990s", "The Naming", "Wilfrid Mennell names the apple 'Ambrosia' after the food of the gods, for its divine taste."),
    ("1990s–2000s", "Commercial Growth", "Ambrosia is propagated and established as a premium 'club variety' with strict quality controls."),
    ("2015", "Patent Expires — Canada", "The Canadian patent expires, allowing broader cultivation across the country."),
    ("2017", "Patent Expires — USA", "American growers gain freedom to cultivate without patent restrictions."),
    ("2021", "Patent Expires — Chile", "Chilean patent expires. Still active in other countries until 2034."),
    ("2024", "A Canadian Favorite", "Ambrosia becomes one of the most-produced apple varieties in Canada."),
];
