/// All 31 apple varieties from the series hub.
/// Source: click-on-the-malus-domestica-ide.netlify.app/apple-varieties

pub struct Variety {
    pub name: &'static str,
    pub emoji: &'static str,
    pub origin: &'static str,
    pub framework: &'static str,
}

pub const AMBROSIA_INDEX: usize = 0;

pub const VARIETIES: &[Variety] = &[
    Variety {
        name: "Ambrosia",
        emoji: "🍎",
        origin: "Canada (BC)",
        framework: "Yew",
    },
    Variety {
        name: "Baldwin",
        emoji: "🍎",
        origin: "USA (MA)",
        framework: "?",
    },
    Variety {
        name: "Braeburn",
        emoji: "🍎",
        origin: "New Zealand",
        framework: "?",
    },
    Variety {
        name: "Cameo",
        emoji: "🍎",
        origin: "USA (WA)",
        framework: "?",
    },
    Variety {
        name: "Cortland",
        emoji: "🍎",
        origin: "USA (NY)",
        framework: "?",
    },
    Variety {
        name: "Cox's Orange Pippin",
        emoji: "🍎",
        origin: "England",
        framework: "?",
    },
    Variety {
        name: "Crabapple",
        emoji: "🍎",
        origin: "Wild",
        framework: "?",
    },
    Variety {
        name: "Empire",
        emoji: "🍎",
        origin: "USA (NY)",
        framework: "?",
    },
    Variety {
        name: "Envy",
        emoji: "🍎",
        origin: "New Zealand",
        framework: "?",
    },
    Variety {
        name: "Fuji",
        emoji: "🍎",
        origin: "Japan",
        framework: "?",
    },
    Variety {
        name: "Golden Delicious",
        emoji: "🍎",
        origin: "USA (WV)",
        framework: "?",
    },
    Variety {
        name: "Granny Smith",
        emoji: "🍎",
        origin: "Australia",
        framework: "?",
    },
    Variety {
        name: "Honeycrisp",
        emoji: "🍎",
        origin: "USA (MN)",
        framework: "?",
    },
    Variety {
        name: "Idared",
        emoji: "🍎",
        origin: "USA (ID)",
        framework: "?",
    },
    Variety {
        name: "Jazz",
        emoji: "🍎",
        origin: "New Zealand",
        framework: "?",
    },
    Variety {
        name: "Jonagold",
        emoji: "🍎",
        origin: "USA (NY)",
        framework: "?",
    },
    Variety {
        name: "Jonathan",
        emoji: "🍎",
        origin: "USA (NY)",
        framework: "?",
    },
    Variety {
        name: "Macoun",
        emoji: "🍎",
        origin: "USA (NY)",
        framework: "?",
    },
    Variety {
        name: "McIntosh",
        emoji: "🍎",
        origin: "Canada (ON)",
        framework: "?",
    },
    Variety {
        name: "Mutsu",
        emoji: "🍎",
        origin: "Japan",
        framework: "?",
    },
    Variety {
        name: "Northern Spy",
        emoji: "🍎",
        origin: "USA (CT)",
        framework: "?",
    },
    Variety {
        name: "Opal",
        emoji: "🍎",
        origin: "Czech Republic",
        framework: "?",
    },
    Variety {
        name: "Pacific Rose",
        emoji: "🍎",
        origin: "New Zealand",
        framework: "?",
    },
    Variety {
        name: "Pink Lady",
        emoji: "🍎",
        origin: "Australia",
        framework: "?",
    },
    Variety {
        name: "Red Delicious",
        emoji: "🍎",
        origin: "USA (IA)",
        framework: "?",
    },
    Variety {
        name: "Rome",
        emoji: "🍎",
        origin: "USA (OH)",
        framework: "?",
    },
    Variety {
        name: "Royal Gala",
        emoji: "🍎",
        origin: "New Zealand",
        framework: "?",
    },
    Variety {
        name: "SnapDragon",
        emoji: "🍎",
        origin: "USA (NY)",
        framework: "?",
    },
    Variety {
        name: "Sonya",
        emoji: "🍎",
        origin: "New Zealand",
        framework: "?",
    },
    Variety {
        name: "SugarBee",
        emoji: "🍎",
        origin: "USA (MN)",
        framework: "?",
    },
    Variety {
        name: "SweeTango",
        emoji: "🍎",
        origin: "USA (MN)",
        framework: "?",
    },
    Variety {
        name: "Winter Banana",
        emoji: "🍎",
        origin: "USA (IN)",
        framework: "?",
    },
];

pub const HUB_URL: &str = "https://click-on-the-malus-domestica-ide.netlify.app/apple-varieties";
pub const GITHUB_URL: &str = "https://github.com/ricardo-camilo-programador-frontend-web";
