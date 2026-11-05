// MIT - TwoTimeMilestone4 - Forsaken TUI Roleplay Engine
// Complete roster as of November 2026 & canonical maps verified from https://forsaken.wiki
use colored::Colorize;
use rustyline::DefaultEditor;
use std::io::{self, Write};
use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerRole {
    Survivor,
    Killer,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SurvivorId {
    TwoTime,
    Chance,
    Shedletsky,
    Elliot,
    Guest1337,
    Noob,
    ZeroZeroSevenNSeven, // 007n7
    Builderman,
    Dusekkar,
    JaneDoe,
    Taph,
    Veeronica,
    Helperbot,
}

#[allow(dead_code)]
impl SurvivorId {
    pub fn all() -> &'static [SurvivorId] {
        &[
            SurvivorId::TwoTime,
            SurvivorId::Chance,
            SurvivorId::Shedletsky,
            SurvivorId::Elliot,
            SurvivorId::Guest1337,
            SurvivorId::Noob,
            SurvivorId::ZeroZeroSevenNSeven,
            SurvivorId::Builderman,
            SurvivorId::Dusekkar,
            SurvivorId::JaneDoe,
            SurvivorId::Taph,
            SurvivorId::Veeronica,
            SurvivorId::Helperbot,
        ]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let low = s.to_lowercase().replace(['-', '_', ' '], "");
        match low.as_str() {
            "twotime" | "two" => Some(Self::TwoTime),
            "chance" => Some(Self::Chance),
            "shedletsky" | "telamon" => Some(Self::Shedletsky),
            "elliot" => Some(Self::Elliot),
            "guest1337" | "1337" | "guest" => Some(Self::Guest1337),
            "noob" => Some(Self::Noob),
            "007n7" | "seven" => Some(Self::ZeroZeroSevenNSeven),
            "builderman" | "builder" => Some(Self::Builderman),
            "dusekkar" | "dusek" => Some(Self::Dusekkar),
            "janedoe" | "jane" => Some(Self::JaneDoe),
            "taph" => Some(Self::Taph),
            "veeronica" | "vee" => Some(Self::Veeronica),
            "helperbot" | "helpy" | "d3functb0t" | "defunctbot" => Some(Self::Helperbot),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KillerId {
    Azure,      // Aspen / Azurewrath
    Slasher,    // Project 13
    C00lkidd,
    JohnDoe,
    OneByOne,   // 1x1x1x1
    Guest666,
    Noli,
    Nosferatu,
}

#[allow(dead_code)]
impl KillerId {
    pub fn all() -> &'static [KillerId] {
        &[
            KillerId::Azure,
            KillerId::Slasher,
            KillerId::C00lkidd,
            KillerId::JohnDoe,
            KillerId::OneByOne,
            KillerId::Guest666,
            KillerId::Noli,
            KillerId::Nosferatu,
        ]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let low = s.to_lowercase().replace(['-', '_', ' '], "");
        match low.as_str() {
            "azure" | "aspen" | "azurewrath" => Some(Self::Azure),
            "slasher" | "project13" | "p13" => Some(Self::Slasher),
            "c00lkidd" | "coolkidd" | "ck" => Some(Self::C00lkidd),
            "johndoe" | "john" | "doe" => Some(Self::JohnDoe),
            "1x1x1x1" | "onebyone" | "1x" => Some(Self::OneByOne),
            "guest666" | "666" => Some(Self::Guest666),
            "noli" => Some(Self::Noli),
            "nosferatu" | "vampire" => Some(Self::Nosferatu),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stance {
    Defiant,
    Sorrowful,
    Fanatical,
    Cold,
    Terrified,
    Mocking,
}

impl Stance {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Defiant => "Defiant (Steadfast)",
            Self::Sorrowful => "Sorrowful / Regretful",
            Self::Fanatical => "Fanatical (Glory to The Spawn)",
            Self::Cold => "Cold & Calculating",
            Self::Terrified => "Terrified / Panicked",
            Self::Mocking => "Mocking & Taunting",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arena {
    RebornFamilyAbode,
    BotanicalConservatory,
    GlassHouses,
    RobloxHQ,
    BuilderBrothersPizza,
    BloodfellManor,
    FamiliarRuins,
    NaturalDisasterIsland,
    CakeFactory,
    YoricksRestingPlace,
    Area51,
    SFOTH,
    UndergroundWar,
    PlanetVoss,
}

impl Arena {
    pub fn all() -> &'static [Arena] {
        &[
            Arena::RebornFamilyAbode,
            Arena::BotanicalConservatory,
            Arena::GlassHouses,
            Arena::RobloxHQ,
            Arena::BuilderBrothersPizza,
            Arena::BloodfellManor,
            Arena::FamiliarRuins,
            Arena::NaturalDisasterIsland,
            Arena::CakeFactory,
            Arena::YoricksRestingPlace,
            Arena::Area51,
            Arena::SFOTH,
            Arena::UndergroundWar,
            Arena::PlanetVoss,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::RebornFamilyAbode => "Reborn Family Abode (Family Home For The Reborn Altar)",
            Self::BotanicalConservatory => "Overgrown Botanical Conservatory (Aspen's Greenhouse)",
            Self::GlassHouses => "Glass Houses (Classic Roblox Arena)",
            Self::RobloxHQ => "Roblox HQ (Corporate Cubicles & Rooftop)",
            Self::BuilderBrothersPizza => "Builder Brothers Pizza (Haunted Kitchen)",
            Self::BloodfellManor => "Bloodfell Manor (Gothic Vampire Estate)",
            Self::FamiliarRuins => "Familiar Ruins (Ancient Robloxia Altars)",
            Self::NaturalDisasterIsland => "Natural Disaster Island (Cataclysm Watchtower)",
            Self::CakeFactory => "Cake Factory (Industrial Mixer Concourse)",
            Self::YoricksRestingPlace => "Yorick's Resting Place (Haunted Graveyard & Crypts)",
            Self::Area51 => "Area 51 (Classified Containment Bunkers)",
            Self::SFOTH => "Sword Fight on the Heights (SFOTH Skybridges)",
            Self::UndergroundWar => "Underground War (Trenches & Mud Bunkers)",
            Self::PlanetVoss => "Planet Voss (Extraterrestrial Void Wasteland)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::RebornFamilyAbode => "Ancient bloodstained stone oblation slabs lit by pale tallow candles. The sacred symbol ꥟ flickers upon the pillars.",
            Self::BotanicalConservatory => "Shattered greenhouse glass, mutated briars, and glowing bioluminescent seeker bulbs humming with toxic poison.",
            Self::GlassHouses => "Transparent azure and emerald glass sky-walkways reflecting a fractured moon. Every step echoes into the dark below.",
            Self::RobloxHQ => "Flickering fluorescent tubes over abandoned cubicles, rain beating against executive windowpanes, and a stormy rooftop.",
            Self::BuilderBrothersPizza => "Bloodstained checkered tiles, glowing brick pizza ovens casting fiery shadows, and buzzing neon restaurant signs.",
            Self::BloodfellManor => "Towering Victorian gothic estate where lightning tears through velvet drapery and rain washes over gargoyle statues.",
            Self::FamiliarRuins => "Crumbling marble pillars and ivy-entangled stone altars left over from the dawn of Robloxia's golden age.",
            Self::NaturalDisasterIsland => "An isolated coastal rock battered by acid downpours, violent earthquakes, and the creaking frame of an old lighthouse.",
            Self::CakeFactory => "Massive churning industrial gears, conveyor belts, and boiling strawberry-glaze vats grinding with deafening noise.",
            Self::YoricksRestingPlace => "Thick graveyard fog drifting between crooked tombstones, open mossy graves, and ancient iron mausoleum gates.",
            Self::Area51 => "Subterranean military research chambers with breached hazard bulkheads and flashing emergency red beacon lights.",
            Self::SFOTH => "Slippery stone skybridges suspended tens of thousands of studs in the clouds above an endless bottomless void.",
            Self::UndergroundWar => "Narrow muddy trench tunnels braced with rotting timbers and barbed wire, shaking under subterranean artillery.",
            Self::PlanetVoss => "Extraterrestrial crimson desert dunes under two eclipsing fractured moons and pulsing alien monoliths.",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let low = s.to_lowercase().replace(['-', '_', ' '], "");
        match low.as_str() {
            "family" | "abode" | "reborn" | "altar" => Some(Self::RebornFamilyAbode),
            "conservatory" | "greenhouse" | "botanical" => Some(Self::BotanicalConservatory),
            "glasshouses" | "glass" => Some(Self::GlassHouses),
            "hq" | "robloxhq" => Some(Self::RobloxHQ),
            "pizza" | "builderbrothers" => Some(Self::BuilderBrothersPizza),
            "manor" | "bloodfell" => Some(Self::BloodfellManor),
            "ruins" | "familiar" => Some(Self::FamiliarRuins),
            "island" | "disaster" => Some(Self::NaturalDisasterIsland),
            "cake" | "factory" => Some(Self::CakeFactory),
            "yorick" | "graveyard" => Some(Self::YoricksRestingPlace),
            "area51" | "51" => Some(Self::Area51),
            "sfoth" | "heights" => Some(Self::SFOTH),
            "war" | "underground" => Some(Self::UndergroundWar),
            "voss" | "planetvoss" => Some(Self::PlanetVoss),
            _ => None,
        }
    }
}

pub struct Ability {
    pub name: &'static str,
    pub desc: &'static str,
    pub power: i32, // Positive = damage to enemy, negative = self heal, 0 = evasion/buff
}

pub struct SurvivorData {
    pub id: SurvivorId,
    pub name: &'static str,
    pub title: &'static str,
    pub class_type: &'static str, // Sentinel, Support, Survivalist
    pub weapon: &'static str,
    pub max_hp: i32,
    pub hp: i32,
    pub resource_name: &'static str,
    pub resource_val: i32,
    pub second_life_used: bool,
    pub price: &'static str,
    pub lore_summary: &'static str,
}

pub struct KillerData {
    pub id: KillerId,
    pub name: &'static str,
    pub title: &'static str,
    pub weapon: &'static str,
    pub max_hp: i32,
    pub hp: i32,
    pub resource_name: &'static str,
    pub resource_val: i32,
    pub price: &'static str,
    pub lore_summary: &'static str,
}

#[derive(Clone)]
pub struct RPLogEntry {
    pub speaker: String,
    pub role_tag: String,
    pub text: String,
    pub is_narrative: bool,
}

pub struct RoleplaySession {
    pub player_role: PlayerRole,
    pub survivor: SurvivorData,
    pub killer: KillerData,
    pub arena: Arena,
    pub stance: Stance,
    pub log: Vec<RPLogEntry>,
    pub turn: u32,
    pub tension: &'static str,
}

impl SurvivorData {
    pub fn new(id: SurvivorId) -> Self {
        match id {
            SurvivorId::TwoTime => Self {
                id,
                name: "Two Time",
                title: "The Fragile Cultist",
                class_type: "Sentinel / Survivalist",
                weapon: "Ghostfire Dagger (Dagger of Sacrifice)",
                max_hp: 100,
                hp: 100,
                resource_name: "Oblation Gauge",
                resource_val: 35,
                second_life_used: false,
                price: "500 PP",
                lore_summary: "A pale robloxian with Dark Slate Blue messy scene hair and black fingerless gloves. Former companion to Aspen in Family Home For The Reborn (Spawn Cult). Sacrificed Aspen with the sleek dagger to appease The Spawn, bearing a manic smile and suppressed torment. Gains a second life through risk and sacrifice ('Never again shall I wonder. Never again shall I hurt. Never again shall I love'). (forsaken.wiki)",
            },
            SurvivorId::Chance => Self {
                id,
                name: "Chance",
                title: "The High-Stakes Gambler",
                class_type: "Sentinel",
                weapon: "Loaded Six-Shooter & Lucky Gold Coin",
                max_hp: 100,
                hp: 100,
                resource_name: "Ammunition",
                resource_val: 6,
                second_life_used: false,
                price: "500 PP",
                lore_summary: "Charismatic gambler adorned with sleek fedora and sunglasses. Gambles his life against nightmare entities with coin flips, high-risk wagers, and trick shots. (forsaken.wiki)",
            },
            SurvivorId::Shedletsky => Self {
                id,
                name: "Shedletsky",
                title: "Telamon, Swordmaster of Robloxia",
                class_type: "Sentinel",
                weapon: "Telamon Illumina Sword & Crispy Fried Chicken",
                max_hp: 110,
                hp: 110,
                resource_name: "Chicken Drumsticks",
                resource_val: 3,
                second_life_used: false,
                price: "Free Starter",
                lore_summary: "Legendary admin and veteran of Robloxia. Faces horrors with booming laughter, his iconic Illumina sword, and a greasy bucket of fried chicken to restore vigor. (forsaken.wiki)",
            },
            SurvivorId::Elliot => Self {
                id,
                name: "Elliot",
                title: "The Selfless Delivery Boy",
                class_type: "Support",
                weapon: "Piping Hot Pizza Boxes & Work Ethic",
                max_hp: 95,
                hp: 95,
                resource_name: "Fresh Pizzas",
                resource_val: 3,
                second_life_used: false,
                price: "Free Starter",
                lore_summary: "A nervous but genuinely heroic delivery boy trapped in the nightmare. Feeds allies to heal them and throws scorching pizza grease to stun hunters. (forsaken.wiki)",
            },
            SurvivorId::Guest1337 => Self {
                id,
                name: "Guest 1337",
                title: "The Last Guest Soldier",
                class_type: "Sentinel",
                weapon: "Tactical Riot Shield & Combat Knuckles",
                max_hp: 115,
                hp: 115,
                resource_name: "Shield Integrity",
                resource_val: 100,
                second_life_used: false,
                price: "Free Starter",
                lore_summary: "The disciplined combat veteran from the Guest faction (The Last Guest). Unbreakable resolve, shielding allies and delivering punishing counter-strikes against horrific pursuers. (forsaken.wiki)",
            },
            SurvivorId::Noob => Self {
                id,
                name: "Noob",
                title: "The Classic Survivor",
                class_type: "Survivalist",
                weapon: "Chilled Bloxy Cola & Unadulterated Panic",
                max_hp: 90,
                hp: 90,
                resource_name: "Bloxy Cola Cans",
                resource_val: 3,
                second_life_used: false,
                price: "Free Starter",
                lore_summary: "Classic yellow, blue, and green robloxian. Survives on pure adrenaline, Bloxy Cola speed boosts, and absurdly lucky dodges accompanied by iconic 'OOF's. (forsaken.wiki)",
            },
            SurvivorId::ZeroZeroSevenNSeven => Self {
                id,
                name: "007n7",
                title: "The Reformed Exploiter / Father",
                class_type: "Survivalist",
                weapon: "Script Injector PDA & Holographic Projector",
                max_hp: 95,
                hp: 95,
                resource_name: "Packet Overclock",
                resource_val: 60,
                second_life_used: false,
                price: "600 PP",
                lore_summary: "Infamous early hacker of Robloxia turned remorseful single adoptive father to c00lkidd. Caught in the realm trying to save his son from demonic corruption, wielding holographic decoys, desync tricks, and protective scripts. (forsaken.wiki)",
            },
            SurvivorId::Builderman => Self {
                id,
                name: "Builderman",
                title: "The Master Architect",
                class_type: "Support",
                weapon: "Heavy Construction Wrench & Architect PDA",
                max_hp: 105,
                hp: 105,
                resource_name: "Building Materials",
                resource_val: 100,
                second_life_used: false,
                price: "500 PP",
                lore_summary: "The legendary architect and founder of Robloxia (David Baszucki). Trapped in the nightmare by The Spectre, he constructs automated Sentries to repel killers and Dispensers to sustain survivors. (forsaken.wiki)",
            },
            SurvivorId::Dusekkar => Self {
                id,
                name: "Dusekkar",
                title: "The Arch-Engineer",
                class_type: "Support",
                weapon: "Golden Ban Hammer & Staff of Protection",
                max_hp: 100,
                hp: 100,
                resource_name: "Spawn Shield Charges",
                resource_val: 2,
                second_life_used: false,
                price: "600 PP",
                lore_summary: "Based on Matt Dusek, legendary Roblox Chief Architect. A revered engineer who channels divine network protocols to create Spawn Protection shields and dispel supernatural corruption. (forsaken.wiki)",
            },
            SurvivorId::JaneDoe => Self {
                id,
                name: "Jane Doe",
                title: "The March 18th Sentinel",
                class_type: "Sentinel / Support",
                weapon: "Classified Investigation Dossier & Flashlight Taser",
                max_hp: 100,
                hp: 100,
                resource_name: "Encrypted Files",
                resource_val: 4,
                second_life_used: false,
                price: "Cabin Quest Unlock",
                lore_summary: "A relentless investigator who searched the Cabin basement for lost documents. Linked to John Doe and the eerie March 18th legend, she fights to uncover the truth and save corrupted souls from eternal void. (forsaken.wiki)",
            },
            SurvivorId::Taph => Self {
                id,
                name: "Taph",
                title: "The Strategic Trapper",
                class_type: "Support",
                weapon: "Spring Trap Launcher & Spiked Tripwires",
                max_hp: 95,
                hp: 95,
                resource_name: "Mechanical Traps",
                resource_val: 3,
                second_life_used: false,
                price: "500 PP",
                lore_summary: "A crafty, resourceful survivalist wearing makeshift field gear. Secures choke points with spring-loaded snares, caltrops, and explosive tripwires that leave killers immobilized. (forsaken.wiki)",
            },
            SurvivorId::Veeronica => Self {
                id,
                name: "Veeronica",
                title: "The Rebellious Cyber-Skater",
                class_type: "Survivalist / Mobility",
                weapon: "Neon Jet-Skateboard & Broadcast Comm",
                max_hp: 100,
                hp: 100,
                resource_name: "Battery Charge",
                resource_val: 80,
                second_life_used: false,
                price: "600 PP",
                lore_summary: "A former Helper RO-Bot turned free-spirited cyber-skater. Trapped by The Spectre, she uses high-velocity skateboard kicks, phase-shift dashes, and broadcast bumper repulsion to outmaneuver hunters. (forsaken.wiki)",
            },
            SurvivorId::Helperbot => Self {
                id,
                name: "Helperbot (Helpy)",
                title: "The Rebuilt Assistant",
                class_type: "Support",
                weapon: "Cardboard Box & Aluminum Foil Limbs",
                max_hp: 80,
                hp: 80,
                resource_name: "Pulse Charges",
                resource_val: 1,
                second_life_used: false,
                price: "650 PP",
                lore_summary: "One of seven Helperbot robots created by Builderman & Roblox Staff for Welcome to Roblox Building. Abandoned after Admins vanished; arrived in Forsaken as broken lobby NPC D3functB0t. Rebuilt by Builderman and Veeronica. Excitable, kind, uses emoticons, and assists survivors with Radar scans, Pulse buffs, and Inspire Haste V. (forsaken.wiki)",
            },
        }
    }

    pub fn abilities(&self) -> Vec<Ability> {
        match self.id {
            SurvivorId::TwoTime => vec![
                Ability { name: "Ghostfire Slash", desc: "Slash with the Dagger of Sacrifice (+25% Oblation)", power: 22 },
                Ability { name: "Sacrificial Oblation", desc: "Chant Spawn prayers, consuming 35% Oblation to heal 30 HP", power: -30 },
                Ability { name: "Shadow Cloak", desc: "Melt into the altar's shadows, evading the next attack", power: 0 },
            ],
            SurvivorId::Chance => vec![
                Ability { name: "Coin Flip Trick Shot", desc: "Heads = 40 critical damage & stun! Tails = quick dodge & reload", power: 40 },
                Ability { name: "Six-Shooter Fan", desc: "Fires 2 rapid rounds into the killer for 24 damage", power: 24 },
                Ability { name: "Double Down", desc: "Raises stakes: +100% damage dealt or received next round", power: 15 },
            ],
            SurvivorId::Shedletsky => vec![
                Ability { name: "Telamon Sword Cleave", desc: "A mighty illuminated swing of the Illumina blade for 26 damage", power: 26 },
                Ability { name: "Feast on Fried Chicken", desc: "Devours crispy drumstick, restoring 35 HP instantly", power: -35 },
                Ability { name: "Admin Battle Cry", desc: "Roars at the killer, staggering them and blunting their fury", power: 12 },
            ],
            SurvivorId::Elliot => vec![
                Ability { name: "Hot Pizza Box Heal", desc: "Opens a fresh box to restore 35 HP", power: -35 },
                Ability { name: "Scalding Pizza Throw", desc: "Hurls boiling cheese and sauce into killer's face: 18 damage + stun", power: 18 },
                Ability { name: "Delivery Sprint", desc: "Frantic speed burst escaping killer's reach", power: 0 },
            ],
            SurvivorId::Guest1337 => vec![
                Ability { name: "Riot Shield Bash", desc: "Smashes heavy shield into killer: 22 damage + knockback", power: 22 },
                Ability { name: "Defensive Phalanx", desc: "Raises tactical guard, cutting incoming damage by 75%", power: 0 },
                Ability { name: "Last Stand Punch", desc: "Soldier's haymaker: 28 damage when under pressure", power: 28 },
            ],
            SurvivorId::Noob => vec![
                Ability { name: "Bloxy Cola Chug", desc: "Chugs soda vigorously: restores 25 HP & high speed boost", power: -25 },
                Ability { name: "Clumsy Flail (OOF!)", desc: "Trips headfirst into killer: deals 15 damage and interrupts them", power: 15 },
                Ability { name: "Blind Panic Sprint", desc: "Screams and scrambles away behind rubble", power: 0 },
            ],
            SurvivorId::ZeroZeroSevenNSeven => vec![
                Ability { name: "Holographic Decoy", desc: "Projects glitch clone that absorbs damage and blinds killer", power: 0 },
                Ability { name: "Packet Desync Strike", desc: "Exploits physics desync to backstab killer: 24 damage", power: 24 },
                Ability { name: "Paternal Resolve", desc: "Channels protective willpower: restores 28 HP", power: -28 },
            ],
            SurvivorId::Builderman => vec![
                Ability { name: "Deploy Sentry Turret", desc: "Constructs automated turret firing studs: 26 damage + slow", power: 26 },
                Ability { name: "Deploy Dispenser", desc: "Constructs medical depot, restoring 35 HP to user", power: -35 },
                Ability { name: "Wrench Concussion", desc: "Strikes killer with heavy steel pipe wrench: 20 damage + stun", power: 20 },
            ],
            SurvivorId::Dusekkar => vec![
                Ability { name: "Spawn Protection Ward", desc: "Envelops self in divine barrier, negating incoming strike", power: 0 },
                Ability { name: "Ban Hammer Judgment", desc: "Brings down golden hammer: 25 radiant damage + stagger", power: 25 },
                Ability { name: "Server Heartbeat Pulse", desc: "Calibrates connection, restoring 30 HP", power: -30 },
            ],
            SurvivorId::JaneDoe => vec![
                Ability { name: "Flashlight Taser Stun", desc: "Blinds killer with tactical strobe: 22 damage + stun", power: 22 },
                Ability { name: "Dossier Decryption", desc: "Analyzes enemy pattern: restores 25 HP & raises defense", power: -25 },
                Ability { name: "Investigative Counter", desc: "Parries blow and drives pen-dagger: 26 damage", power: 26 },
            ],
            SurvivorId::Taph => vec![
                Ability { name: "Spring Trap Snare", desc: "Triggers mechanical trap: 24 damage + locks killer in place", power: 24 },
                Ability { name: "Caltrop Scatter", desc: "Litters floor with razor spikes: 18 damage + bleed", power: 18 },
                Ability { name: "Adrenaline Patch", desc: "Applies combat tourniquet: restores 30 HP", power: -30 },
            ],
            SurvivorId::Veeronica => vec![
                Ability { name: "Skateboard Kickflip Rush", desc: "Rams killer at top speed on jet-board: 25 damage", power: 25 },
                Ability { name: "Broadcast Bumper Mode", desc: "Emits repulsive energy barrier, deflecting attacks", power: 0 },
                Ability { name: "Cyber-Battery Overdrive", desc: "Recharges robotic core: restores 28 HP", power: -28 },
            ],
            SurvivorId::Helperbot => vec![
                Ability { name: "Pulse Wave", desc: "Expanding wave granting Regen I + Resistance I (-20% dmg). Consumes stored charges (1-3) for +radius/duration", power: -23 },
                Ability { name: "Inspire Aura", desc: "10s aura (14 studs) granting allies Haste V (50% faster CD recovery) & generates Pulse charges", power: 0 },
                Ability { name: "Radar Pulse Scan", desc: "Scans 100 studs every 12s, pinging unfinished generators, items, and tracking survivor auras", power: 15 },
            ],
        }
    }
}

impl KillerData {
    pub fn new(id: KillerId) -> Self {
        match id {
            KillerId::Azure => Self {
                id,
                name: "Azure (Aspen)",
                title: "The Azurewrath Botanist",
                weapon: "Bio-Corrupted Tentacles & Seeker Bulbs",
                max_hp: 130,
                hp: 130,
                resource_name: "Seeker Bulbs Active",
                resource_val: 3,
                price: "$2,000 PP",
                lore_summary: "Formerly Aspen, botanist of the Family Home For The Reborn. Betrayed and sacrificed by Two Time to The Spawn. Reborn through The Spectre as Azurewrath: purple torso, dark skin, and 4 writhing black tentacles driven by bitter sorrow and wrath. (forsaken.wiki)",
            },
            KillerId::Slasher => Self {
                id,
                name: "Slasher (Project 13)",
                title: "The Undead Necro-Behemoth",
                weapon: "Rusted Heavy Machete & Roaring Chainsaw",
                max_hp: 145,
                hp: 145,
                resource_name: "Raging Pace Fury",
                resource_val: 20,
                price: "Free Starter",
                lore_summary: "Reanimated juggernaut guided by the ancient Necrobloxicon jammed into his shoulder. Relentless pursuer with accelerating speed and brutal cleaving weapons. (forsaken.wiki)",
            },
            KillerId::C00lkidd => Self {
                id,
                name: "c00lkidd",
                title: "Leader of Team c00lkidd",
                weapon: "Corrupt Nature & Walkspeed Script Exploit",
                max_hp: 120,
                hp: 120,
                resource_name: "Packet Injection",
                resource_val: 50,
                price: "Free Starter",
                lore_summary: "Notorious hacker killer enveloped in fiery red aura and glitch particles. Adopted by 007n7 before being consumed by corrupted scripts. Bends reality with Walkspeed Override and summons flaming explosive pizza minions. (forsaken.wiki)",
            },
            KillerId::JohnDoe => Self {
                id,
                name: "John Doe",
                title: "The March 18th Anomaly",
                weapon: "Corrupted Digital Spikes & Static Claws",
                max_hp: 125,
                hp: 125,
                resource_name: "Digital Corruption",
                resource_val: 45,
                price: "Shop Killer",
                lore_summary: "Corrupted myth of early Robloxia. A shifting shadowy figure corrupting space with digital spikes, binary code whispers, and reality crashes. Linked tragically to Jane Doe. (forsaken.wiki)",
            },
            KillerId::OneByOne => Self {
                id,
                name: "1x1x1x1",
                title: "The 4th-Dimensional Destroyer",
                weapon: "Shattered Emerald Swords of Entropy",
                max_hp: 135,
                hp: 135,
                resource_name: "Dimensional Fractures",
                resource_val: 4,
                price: "Shop Killer",
                lore_summary: "The cosmic terror inscribed in the Necrobloxicon. Commands floating shattered emerald blades and tears rifts through reality, viewing mortals as fleeting bugs. (forsaken.wiki)",
            },
            KillerId::Guest666 => Self {
                id,
                name: "Guest 666",
                title: "The Demonic Legend",
                weapon: "Obsidian Claws & Hellfire Roar",
                max_hp: 140,
                hp: 140,
                resource_name: "Blood Frenzy",
                resource_val: 30,
                price: "Shop Killer",
                lore_summary: "Mythical demonic guest with red skin, curled ram horns, and ferocious claws. Overwhelms prey with earth-shattering roars and feral pounces. (forsaken.wiki)",
            },
            KillerId::Noli => Self {
                id,
                name: "Noli",
                title: "The Void Star Harbinger",
                weapon: "Corrupted Void Star & Hallucination Phantoms",
                max_hp: 125,
                hp: 125,
                resource_name: "Void Decay",
                resource_val: 50,
                price: "Shop Killer",
                lore_summary: "Classic Roblox platform myth from February 2010. Wielder of the Void Star, trapped in the realm after making a deal to halt the star's decay, leaving him with half a body and half a mind. Uses hallucinations, decoy clones, fake generators, and high-velocity Void Rush. (forsaken.wiki)",
            },
            KillerId::Nosferatu => Self {
                id,
                name: "Nosferatu",
                title: "The Sin of Gluttony",
                weapon: "Blood Hook & Crimson Vampire Claws",
                max_hp: 135,
                hp: 135,
                resource_name: "Blood Vials",
                resource_val: 4,
                price: "$1,922 PP",
                lore_summary: "A ruthless aristocratic vampire hunter embodying the Sin of Gluttony. Pursues survivors relentlessly with Blood Hook snares, dynamic chase dread, and catastrophic blood surges. (forsaken.wiki)",
            },
        }
    }

    pub fn abilities(&self) -> Vec<Ability> {
        match self.id {
            KillerId::Azure => vec![
                Ability { name: "Seeker Bulb Detonation", desc: "Triggers bio-luminescent flora: 26 poison damage + blind", power: 26 },
                Ability { name: "Stigmatize Lash", desc: "Four tentacles impale victim: 28 damage + bleed & remorse", power: 28 },
                Ability { name: "Enstrangle Grip", desc: "Chokes survivor with black tentacles: 32 damage", power: 32 },
            ],
            KillerId::Slasher => vec![
                Ability { name: "Heavy Machete Cleave", desc: "Brutal rusted blade chop: 28 damage", power: 28 },
                Ability { name: "Chainsaw Overdrive", desc: "Revs chainsaw with terrifying whine: 38 massive damage", power: 38 },
                Ability { name: "Raging Pace Stalk", desc: "Surges forward, locking survivor in relentless chase", power: 15 },
            ],
            KillerId::C00lkidd => vec![
                Ability { name: "Walkspeed Override", desc: "Teleports behind target with red tracer: 26 backstab damage", power: 26 },
                Ability { name: "Corrupted Pizza Swarm", desc: "Sends fiery homing pizzas: 24 burn damage", power: 24 },
                Ability { name: "Server Glitch Stun", desc: "Freezes target's connection: 18 damage + skips turn", power: 18 },
            ],
            KillerId::JohnDoe => vec![
                Ability { name: "Digital Spike Eruption", desc: "Corrupted yellow spikes erupt from the ground: 27 damage", power: 27 },
                Ability { name: "Binary Static Blind", desc: "Fills mind with static: 20 damage + reduces accuracy", power: 20 },
                Ability { name: "Crash Protocol", desc: "Distorts target's physical code: 32 damage", power: 32 },
            ],
            KillerId::OneByOne => vec![
                Ability { name: "Shattered Blade Barrage", desc: "Four emerald swords impale the arena: 30 damage", power: 30 },
                Ability { name: "Dimensional Rift", desc: "Slices space itself: 36 cosmic damage", power: 36 },
                Ability { name: "Entropic Zone", desc: "Corrupts floor with decay: 22 damage over time", power: 22 },
            ],
            KillerId::Guest666 => vec![
                Ability { name: "Hellfire Pounce", desc: "Crushes survivor to the earth: 28 damage", power: 28 },
                Ability { name: "Demonic Roar", desc: "Deafening sonic terror: 18 damage + panic", power: 18 },
                Ability { name: "Infernal Rend", desc: "Obsidian claws shred armor: 34 damage", power: 34 },
            ],
            KillerId::Noli => vec![
                Ability { name: "Void Rush Ambush", desc: "Flashes through void space to strike survivor from behind: 28 damage", power: 28 },
                Ability { name: "Hallucination Decoy", desc: "Spawns deceptive phantom clones that strike simultaneously: 22 damage", power: 22 },
                Ability { name: "Void Star Singularity", desc: "Pours unstable void energy into the victim: 34 dark damage", power: 34 },
            ],
            KillerId::Nosferatu => vec![
                Ability { name: "Blood Hook Impalement", desc: "Hurls rusted chain hook, dragging survivor close: 27 damage", power: 27 },
                Ability { name: "Vampiric Feast", desc: "Drains victim's blood: 24 damage + heals Nosferatu 20 HP", power: 24 },
                Ability { name: "Cataclysmic Blood Surge", desc: "Detonates crimson torrent across the arena: 35 damage", power: 35 },
            ],
        }
    }
}

pub fn draw_hp_bar(cur: i32, max: i32, length: usize) -> String {
    let cur = cur.max(0).min(max);
    let ratio = cur as f32 / max as f32;
    let filled = (ratio * length as f32).round() as usize;
    let empty = length.saturating_sub(filled);

    let filled_str = "█".repeat(filled);
    let empty_str = "░".repeat(empty);

    let bar = format!("{}{}", filled_str, empty_str);
    if ratio > 0.6 {
        bar.green().to_string()
    } else if ratio > 0.3 {
        bar.yellow().to_string()
    } else {
        bar.red().to_string()
    }
}

impl RoleplaySession {
    pub fn new(player_role: PlayerRole, survivor_id: SurvivorId, killer_id: KillerId, arena: Arena) -> Self {
        let survivor = SurvivorData::new(survivor_id);
        let killer = KillerData::new(killer_id);

        let mut session = Self {
            player_role,
            survivor,
            killer,
            arena,
            stance: Stance::Defiant,
            log: Vec::new(),
            turn: 1,
            tension: "TENSE STALKING",
        };

        session.seed_intro_narrative();
        session
    }

    fn seed_intro_narrative(&mut self) {
        let s_name = self.survivor.name;
        let k_name = self.killer.name;
        let arena_name = self.arena.name();

        self.log.push(RPLogEntry {
            speaker: "꥟ NARRATION".to_string(),
            role_tag: "ENVIRONMENT".to_string(),
            text: format!("The confrontation ignites at {}... Heavy mist blankets the floor as {} encounters {}.", arena_name, s_name, k_name),
            is_narrative: true,
        });

        // Specialized dialogue for canonical lore matchups
        if self.survivor.id == SurvivorId::TwoTime && self.killer.id == KillerId::Azure {
            self.log.push(RPLogEntry {
                speaker: "Azure (Aspen)".to_string(),
                role_tag: "KILLER".to_string(),
                text: "*four black tentacles twitch as bioluminescent bulbs pulse* \"Two Time... did you think the Spawn could hide you from the corpse you buried in the greenhouse?\"".to_string(),
                is_narrative: false,
            });
            self.log.push(RPLogEntry {
                speaker: "Two Time".to_string(),
                role_tag: "SURVIVOR".to_string(),
                text: "*clutches Ghostfire Dagger with white knuckles, manic smile trembling* \"Aspen... the ritual had to be fulfilled. The Spawn required an oblation for our ascension!\"".to_string(),
                is_narrative: false,
            });
        } else if self.survivor.id == SurvivorId::ZeroZeroSevenNSeven && self.killer.id == KillerId::C00lkidd {
            self.log.push(RPLogEntry {
                speaker: "007n7".to_string(),
                role_tag: "SURVIVOR".to_string(),
                text: "*lowers PDA screen, voice thick with paternal heartbreak* \"c00lkidd... son. Look at what those corrupted scripts have done to you. Put the exploits down and let me take you home!\"".to_string(),
                is_narrative: false,
            });
            self.log.push(RPLogEntry {
                speaker: "c00lkidd".to_string(),
                role_tag: "KILLER".to_string(),
                text: "*red glitch aura flares violently with manic laughter* \"Home?! I run Team c00lkidd now, Dad! Your old scripts are obsolete trash! I don't need your protection!\"".to_string(),
                is_narrative: false,
            });
        } else if self.survivor.id == SurvivorId::JaneDoe && self.killer.id == KillerId::JohnDoe {
            self.log.push(RPLogEntry {
                speaker: "Jane Doe".to_string(),
                role_tag: "SURVIVOR".to_string(),
                text: "*flashes tactical light through the digital static* \"John... I found your documents in the cabin basement. March 18th wasn't your fault! You have to fight the corruption!\"".to_string(),
                is_narrative: false,
            });
            self.log.push(RPLogEntry {
                speaker: "John Doe".to_string(),
                role_tag: "KILLER".to_string(),
                text: "*spikes tremble as shifting binary code leaks from hollow eyes* \"01001010... Jane...? Memory sector corrupted... The Spectre demands your deletion... RUN!\"".to_string(),
                is_narrative: false,
            });
        } else if self.survivor.id == SurvivorId::Builderman && self.killer.id == KillerId::OneByOne {
            self.log.push(RPLogEntry {
                speaker: "Builderman".to_string(),
                role_tag: "SURVIVOR".to_string(),
                text: "*raises heavy construction wrench* \"1x1x1x1! I laid the bedrock of Robloxia with my bare hands! You won't dissolve this platform into entropy!\"".to_string(),
                is_narrative: false,
            });
            self.log.push(RPLogEntry {
                speaker: "1x1x1x1".to_string(),
                role_tag: "KILLER".to_string(),
                text: "*four emerald blades hum in geometric orbit* \"Puny architect... your bricks are mere dust in the face of the fourth dimension. Watch your creation shatter!\"".to_string(),
                is_narrative: false,
            });
        } else if self.survivor.id == SurvivorId::Guest1337 && self.killer.id == KillerId::Guest666 {
            self.log.push(RPLogEntry {
                speaker: "Guest 1337".to_string(),
                role_tag: "SURVIVOR".to_string(),
                text: "*slams tactical riot shield against the ground* \"666! You abandoned our brotherhood for hellfire and demonic blood! Today, the Last Guest puts an end to your curse!\"".to_string(),
                is_narrative: false,
            });
            self.log.push(RPLogEntry {
                speaker: "Guest 666".to_string(),
                role_tag: "KILLER".to_string(),
                text: "*EARTH-SPLITTING DEMONIC ROAR* \"THE GUESTS WERE WEAK AND EXILED! I CHOSE UNLIMITED POWER! YOUR SHIELD WILL CRUMBLE BENEATH MY CLAWS!\"".to_string(),
                is_narrative: false,
            });
        } else if self.survivor.id == SurvivorId::Shedletsky && self.killer.id == KillerId::Slasher {
            self.log.push(RPLogEntry {
                speaker: "Slasher".to_string(),
                role_tag: "KILLER".to_string(),
                text: "*chainsaw revs with a violent metallic screech, heavy breathing echoing behind the mask* \"Old warrior... the Necrobloxicon hungers for ancient blood...\"".to_string(),
                is_narrative: false,
            });
            self.log.push(RPLogEntry {
                speaker: "Shedletsky".to_string(),
                role_tag: "SURVIVOR".to_string(),
                text: "*bites a drumstick and brandishes Illumina blade* \"HA! I survived the dawn of Robloxia, you lumbering freak! Let's see that blade of yours!\"".to_string(),
                is_narrative: false,
            });
        } else if self.survivor.id == SurvivorId::Helperbot && self.killer.id == KillerId::C00lkidd {
            self.log.push(RPLogEntry {
                speaker: "Helperbot".to_string(),
                role_tag: "SURVIVOR".to_string(),
                text: "*antennas wobble frantically with a warning chime* \"[o_o;] Warning! Unauthorized script injection detected! Master Builderman and Veeronica taught me building safety! Please do not delete our teammates!\"".to_string(),
                is_narrative: false,
            });
            self.log.push(RPLogEntry {
                speaker: "c00lkidd".to_string(),
                role_tag: "KILLER".to_string(),
                text: "*red glitch aura flares with mocking laughter* \"A cardboard robot held together by tin foil and tape?! D3functB0t, you should have stayed in the scrap heap where the Bacon Empire dumped you!\"".to_string(),
                is_narrative: false,
            });
        } else if self.survivor.id == SurvivorId::Helperbot && self.killer.id == KillerId::Azure {
            self.log.push(RPLogEntry {
                speaker: "Helperbot".to_string(),
                role_tag: "SURVIVOR".to_string(),
                text: "*rotates cardboard head with a friendly harmonic chime* \"[^_^] Hello plant friend! Radar indicates severe biological distress! Stay close for Regeneration and Pulse repairs!\"".to_string(),
                is_narrative: false,
            });
            self.log.push(RPLogEntry {
                speaker: "Azure (Aspen)".to_string(),
                role_tag: "KILLER".to_string(),
                text: "*tentacles lash across the stone floor with bitter wrath* \"Repairs?! No amount of aluminum foil or clockwork innocence can mend what Two Time butchered on that altar! Begone, mechanical toy!\"".to_string(),
                is_narrative: false,
            });
        } else {
            self.log.push(RPLogEntry {
                speaker: self.killer.name.to_string(),
                role_tag: "KILLER".to_string(),
                text: format!("*steps out from the shadows, eyes locked on {}* \"Your time in this realm has run out.\"", self.survivor.name),
                is_narrative: false,
            });
        }
    }

    pub fn render_screen(&self) {
        print!("\x1b[2J\x1b[1;1H"); // Clear screen
        println!("{}", "╔════════════════════════════════════════════════════════════════════════════════════════════════════════╗".bright_blue());
        println!("║ {} │ {} ║", 
            "꥟ FORSAKEN ROLEPLAY — SURVIVOR & KILLER ARENA".bold().truecolor(122, 165, 214),
            "Lore Source: https://forsaken.wiki".cyan());
        println!("{}", "╠════════════════════════════════════════════════════════════════════════════════════════════════════════╣".bright_blue());

        // Location & Tension
        let arena_line = format!(" Realm: {} │ Stance: {} │ Tension: {}", 
            self.arena.name().yellow(), 
            self.stance.name().magenta(), 
            self.tension.red().bold());
        println!("║ {:<114} ║", arena_line);

        println!("{}", "╠════════════════════════════════════════════════════════════════════════════════════════════════════════╣".bright_blue());

        // Combatant 1: Survivor
        let s_tag = if self.player_role == PlayerRole::Survivor { "[YOU - SURVIVOR]".bold().green() } else { "[NPC - SURVIVOR]".yellow() };
        let s_bar = draw_hp_bar(self.survivor.hp, self.survivor.max_hp, 14);
        let s_extra = format!("{}: {}", self.survivor.resource_name, self.survivor.resource_val);
        println!("║ {} {:<16} HP: {:>3}/{:<3} [{}] │ {:<32} ║",
            s_tag,
            self.survivor.name.bold(),
            self.survivor.hp,
            self.survivor.max_hp,
            s_bar,
            s_extra.cyan()
        );

        // Combatant 2: Killer
        let k_tag = if self.player_role == PlayerRole::Killer { "[YOU - KILLER]".bold().red() } else { "[NPC - KILLER]".yellow() };
        let k_bar = draw_hp_bar(self.killer.hp, self.killer.max_hp, 14);
        let k_extra = format!("{}: {}", self.killer.resource_name, self.killer.resource_val);
        println!("║ {} {:<18} HP: {:>3}/{:<3} [{}] │ {:<32} ║",
            k_tag,
            self.killer.name.bold(),
            self.killer.hp,
            self.killer.max_hp,
            k_bar,
            k_extra.red()
        );

        println!("{}", "╠════════════════════════════════════════════════════════════════════════════════════════════════════════╣".bright_blue());
        println!("║ {:<102} ║", "꥟ NARRATIVE & DIALOGUE LOG (Recent exchanges):".bold().truecolor(224, 224, 224));
        println!("{}", "╟────────────────────────────────────────────────────────────────────────────────────────────────────────╢".bright_blue());

        // Print last 8 log entries
        let start = if self.log.len() > 8 { self.log.len() - 8 } else { 0 };
        for entry in &self.log[start..] {
            if entry.is_narrative {
                println!("║ {:<102} ║", entry.text.italic().truecolor(180, 180, 180));
            } else {
                let speaker_colored = if entry.role_tag == "SURVIVOR" {
                    entry.speaker.green().bold()
                } else if entry.role_tag == "KILLER" {
                    entry.speaker.red().bold()
                } else {
                    entry.speaker.yellow()
                };
                let line = format!("[{}] {}", speaker_colored, entry.text);
                println!("║ {:<114} ║", line);
            }
        }

        println!("{}", "╚════════════════════════════════════════════════════════════════════════════════════════════════════════╝".bright_blue());
        println!();
        println!("{}", "─── ROLEPLAY ACTION MENU ─────────────────────────────────────────────────────────────".bold().truecolor(122, 165, 214));
        println!("  {} Freeform Roleplay  - Type any custom speech or action (e.g. *grips dagger* \"Aspen...\")", "[1]".bold().cyan());
        println!("  {} Lore Dialogue      - Choose canonical voice lines & quotes from forsaken.wiki", "[2]".bold().cyan());
        println!("  {} Character Ability  - Execute Forsaken skill / combat maneuver", "[3]".bold().cyan());
        println!("  {} Emotional Stance   - Switch attitude (Defiant, Sorrowful, Fanatical, Cold, Panicked)", "[4]".bold().cyan());
        println!("  {} Forsaken Codex     - Inspect character lore, relationships & trivia", "[5]".bold().cyan());
        println!("  {} Relocate Arena     - Move to another Forsaken realm (14 canonical maps)", "[6]".bold().cyan());
        println!("  {} Exit Roleplay      - Return to Two-Time shell", "[0]".bold().red());
        println!();
    }

    pub fn handle_lore_dialogue(&mut self) {
        println!("{}", "\n─── CHOOSE CANONICAL LORE DIALOGUE ───".bold().yellow());
        let options = self.get_canonical_options();
        for (i, opt) in options.iter().enumerate() {
            println!("  [{}] {}", i + 1, opt);
        }
        println!("  [0] Cancel");
        print!("\n  ꥟ Select [0-{}]: ", options.len());
        let _ = io::stdout().flush();

        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_ok() {
            let choice: usize = buf.trim().parse().unwrap_or(99);
            if choice >= 1 && choice <= options.len() {
                let chosen = options[choice - 1].to_string();
                self.record_user_dialogue(&chosen);
                self.trigger_npc_response(&chosen);
            }
        }
    }

    fn get_canonical_options(&self) -> Vec<&'static str> {
        if self.player_role == PlayerRole::Survivor {
            match self.survivor.id {
                SurvivorId::TwoTime => {
                    if self.killer.id == KillerId::Azure {
                        vec![
                            "\"Aspen... tell me you can still hear me beneath that dark malice.\"",
                            "\"Never again shall I wonder. Never again shall I hurt. Never again shall I love.\"",
                            "*holds up the sleek Ghostfire Dagger with a manic smile* \"Look... I gave us a nice knife!\"",
                            "\"Stand in silent prayer with me as we give praise to The Spawn! It gave me a second life!\"",
                            "\"The Spawn demands oblation, Aspen! I had no choice... you wanted to abandon the faith!\"",
                        ]
                    } else {
                        vec![
                            "\"Stand in silent prayer with me as we give praise... The Spawn watches over us.\"",
                            "\"Never again shall I wonder. Never again shall I hurt. Never again shall I love.\"",
                            "\"Have you any regrets, dear? I have given The Spawn my own... and it guided me forward.\"",
                            "*manic grin widens as the blade reflects the moonlight* \"GLORY TO THE SPAWN!\"",
                        ]
                    }
                }
                SurvivorId::Chance => vec![
                    "\"Feeling lucky today, pal? Because the odds just shifted in my favor.\"",
                    "\"Never bet against a guy who knows how the coin lands!\"",
                    "\"Fifty-fifty? I like those odds, monster! Let's spin the cylinder!\"",
                    "\"A little sleight of hand never hurt anyone... well, except for you!\"",
                ],
                SurvivorId::Shedletsky => vec![
                    "\"HA! You call that an attack?! Telamon never yields!\"",
                    "*bites into a crispy chicken drumstick* \"Nothing a bucket of fried chicken can't fix!\"",
                    "\"Taste the cold ancient steel of Robloxia's golden age!\"",
                    "\"I've been banishing creeps like you since before your textures were rendered!\"",
                ],
                SurvivorId::Elliot => vec![
                    "\"I-I just came to deliver the pizza! Please don't hurt me!\"",
                    "\"Take a hot slice! Customer satisfaction guaranteed, even in a nightmare!\"",
                    "\"I didn't sign up for horror movie monsters in my minimum wage job description!\"",
                    "\"Eat this hot mozzarella before I drop the whole stack on you!\"",
                ],
                SurvivorId::Guest1337 => vec![
                    "\"We guests fight until our last breath. You won't break my line.\"",
                    "\"Stand behind me! You'll have to tear through my riot shield first!\"",
                    "\"For every guest that was banished to the shadows of Robloxia!\"",
                    "\"Tactical discipline beats raw savagery every single time.\"",
                ],
                SurvivorId::Noob => vec![
                    "\"OOF! Please don't hit me, I'm just a noob!\"",
                    "*chugs Bloxy Cola frantically* \"Ahhh, cold and fizzy! You can't catch me!\"",
                    "\"Why is everyone always trying to murder us?! What did I do?!\"",
                    "\"Run run run run run! Don't look back!\"",
                ],
                SurvivorId::ZeroZeroSevenNSeven => {
                    if self.killer.id == KillerId::C00lkidd {
                        vec![
                            "\"c00lkidd... son, please! Look at what this realm has done to you! Put the scripts down!\"",
                            "\"I adopted you to give you a real home... I won't let The Spectre destroy my boy!\"",
                            "\"I wrote the exploits that broke these systems. You can't trap your father in your own code.\"",
                            "\"Every glitch in this realm has a vulnerability. Let me help you break free!\"",
                        ]
                    } else {
                        vec![
                            "\"I wrote the exploits that broke these systems. You can't trap me in my own code.\"",
                            "\"A diversion script is already compiling behind you.\"",
                            "\"Every glitch in this realm has a vulnerability. I'm going to find it.\"",
                            "\"You're just another malicious process waiting for a patch.\"",
                        ]
                    }
                }
                SurvivorId::Builderman => vec![
                    "\"I built this entire universe piece by piece... I won't let you tear it apart!\"",
                    "\"Dispenser going up! Stay close and keep your health topped off!\"",
                    "\"Sentry calibrated! Let's see how you handle automated defense!\"",
                    "\"Every brick in this realm answers to the master architect!\"",
                ],
                SurvivorId::Dusekkar => vec![
                    "\"By the authority of the core engine, you are denied clearance!\"",
                    "\"Activating Spawn Protection... your corrupted malice cannot pierce this ward.\"",
                    "\"I wrote the fundamental laws of this simulation. You are merely an error code.\"",
                    "\"Feel the weight of the ancient Ban Hammer!\"",
                ],
                SurvivorId::JaneDoe => {
                    if self.killer.id == KillerId::JohnDoe {
                        vec![
                            "\"John... is that you beneath the static? Please, remember who you were before March 18th!\"",
                            "\"I found your files in the cabin basement... you're not a monster, you're trapped!\"",
                            "\"The truth is written in these documents, and no amount of corruption can erase it!\"",
                            "\"I won't leave you in the dark, John! Fight The Spectre's grasp!\"",
                        ]
                    } else {
                        vec![
                            "\"The truth is written in these documents, and no amount of corruption can erase it.\"",
                            "\"Keep the flashlight beam in their eyes! Don't let the darkness close in!\"",
                            "\"I've solved harder cases than this nightmare!\"",
                            "\"Your footprint is stamped all over this realm... and I'm tracking you down!\"",
                        ]
                    }
                }
                SurvivorId::Taph => vec![
                    "\"Watch your step, monster! The floor isn't as solid as it looks!\"",
                    "\"One spring trap, one dead stop. Works every single time.\"",
                    "\"I've rigged every doorway in this hallway. Come find me if you dare!\"",
                    "\"Preparation is the difference between survival and a body bag.\"",
                ],
                SurvivorId::Veeronica => vec![
                    "\"Catch me if you can, slowpokes! Wheels beat claws every single day!\"",
                    "\"Broadcast mode activated! Bumper to your ugly face!\"",
                    "\"I was programmed to give tutorials, but now I'm schooling monsters!\"",
                    "\"You're too slow for 2026! Eat my neon exhaust!\"",
                ],
                SurvivorId::Helperbot => vec![
                    "\"[^_^] Hello friend! Do not worry, Helperbot is on duty! Stay close for Regeneration and Haste!\"",
                    "\"(b^_^)b 'The right tools are never far away. Sometimes you just need a little help finding them!' — Builderman\"",
                    "\"[o_o] Radar scan pinged! Unfinished generator detected within 100 studs! We can finish this together!\"",
                    "\"[^o^] 'Keep encouraging each other. Nobody builds something great all on their own!' Veeronica says I'm doing great!\"",
                    "\"[>_<] Ouchie! Aluminum foil limbs slightly dented, but core programming intact! Keep moving!\"",
                ],
            }
        } else {
            // Player is Killer
            match self.killer.id {
                KillerId::Azure => {
                    if self.survivor.id == SurvivorId::TwoTime {
                        vec![
                            "\"Did your precious 'Spawn' tell you how cold the grave was when your dagger pierced my heart, Two Time?!\"",
                            "\"Aspen is dead. You butchered him on that stone altar. Only Azurewrath remains to settle the score.\"",
                            "\"May the hand of nature be more merciful than your human folly!\"",
                            "*tentacles lash across the stone floor* \"The fleeting sound of the wind sounds softer today than usual... as you breathe your last!\"",
                            "\"Give back the breath you stole from me in the name of your false god!\"",
                        ]
                    } else if self.survivor.id == SurvivorId::Helperbot {
                        vec![
                            "\"Builderman's tin-and-cardboard toy... your pulses cannot cleanse the rot of betrayal.\"",
                            "\"Did your creator tell you how fragile foil is against the thorns of the forgotten?\"",
                            "\"May the hand of nature crush your circuits into peaceful silence.\"",
                        ]
                    } else {
                        vec![
                            "\"The fleeting sound of the wind sounds softer today than usual.\"",
                            "\"May the hand of nature be more merciful than human folly.\"",
                            "*bioluminescent bulbs burst into poisonous mist* \"Feel the thorns take root in your veins!\"",
                            "\"No one escapes the greenhouse of the forgotten...\"",
                        ]
                    }
                }
                KillerId::Slasher => vec![
                    "*chainsaw revs with a violent deafening roar, shaking the ground*",
                    "\"Flesh... tears... easily... under the Necrobloxicon's guidance...\"",
                    "*heavy wet breathing rattles behind the mask as the rusted machete scrapes the floor*",
                    "\"No running... no prayers... only butchery...\"",
                ],
                KillerId::C00lkidd => {
                    if self.survivor.id == SurvivorId::ZeroZeroSevenNSeven {
                        vec![
                            "\"I don't need a father anymore, 007n7! Team c00lkidd runs this server now!\"",
                            "\"Your scripts are ancient history, Dad! Watch me override your entire existence!\"",
                            "\"You should have stayed out of my way! No one patches my code!\"",
                            "\"Look at the fire, Dad! Isn't it beautiful?!\"",
                        ]
                    } else if self.survivor.id == SurvivorId::Helperbot {
                        vec![
                            "\"D3functB0t! You should have stayed in pieces after the Bacon Empire finished with you!\"",
                            "\"No one uses 2008 building bots anymore! Team c00lkidd rules this server!\"",
                            "\"Watch me rewrite your cardboard hitboxes into zero!\"",
                        ]
                    } else {
                        vec![
                            "\"Join Team c00lkidd today! Or get permanently banned from existence!\"",
                            "\"HAHAHA! Did you really think basic collision mechanics would save you from my override?!\"",
                            "\"Special flaming pizza delivery straight to your skull! Enjoy the extra burn!\"",
                            "\"You're just another script error for me to delete!\"",
                        ]
                    }
                }
                KillerId::JohnDoe => {
                    if self.survivor.id == SurvivorId::JaneDoe {
                        vec![
                            "\"01001010 01000001 01001110 01000101... Jane... memory leaking... CANNOT STOP THE SPIKES!\"",
                            "\"March 18th... the code rewrote everything... do not look at me!\"",
                            "\"The cabin... the documents... it burns in my head! Run, Jane!\"",
                            "*spikes erupt erratically in grief and agony* \"DELETE... EVERYTHING!\"",
                        ]
                    } else {
                        vec![
                            "\"01000100 01001111 01000101... The system can no longer sustain your process.\"",
                            "\"March 18th has arrived for you. Corrupting memory sectors...\"",
                            "\"Feel the static gnawing at the edges of your reality.\"",
                            "*glitches violently, teleporting through digital fragments* \"Execution pending...\"",
                        ]
                    }
                }
                KillerId::OneByOne => vec![
                    "\"Insignificant mortals crawling in a fragile three-dimensional cage.\"",
                    "\"I am the architect of your desolation. The Necrobloxicon was merely the key!\"",
                    "\"Perish beneath the cold geometry of total dimensional annihilation!\"",
                    "\"Your prayers are meaningless static to an ancient god!\"",
                ],
                KillerId::Guest666 => vec![
                    "*EARTH-SHATTERING DEMONIC ROAR THAT SPLITS THE AIR*",
                    "\"THE FORGOTTEN GUESTS DEMAND YOUR ENTRAILS!\"",
                    "*digs razor-sharp obsidian claws into the earth with hellfire spitting from jaws*",
                    "\"NO WALLS CAN PROTECT YOU FROM THE DEMON 666!\"",
                ],
                KillerId::Noli => vec![
                    "\"The Void Star never dies... it only hungers for more souls.\"",
                    "\"Do your eyes deceive you? I am standing right behind you!\"",
                    "\"Half a body, half a mind, but eternal malice from February 2010.\"",
                    "*fades into shimmering void particles* \"Welcome to your permanent hallucination.\"",
                ],
                KillerId::Nosferatu => vec![
                    "\"Your pulsing veins sing such a sweet, intoxicating melody...\"",
                    "\"The Sin of Gluttony can never be sated with mere apologies!\"",
                    "*snaps Blood Hook through the air* \"Come closer, little sheep... into the butcher's embrace!\"",
                    "\"A century in the crypt has only sharpened my thirst for warm life!\"",
                ],
            }
        }
    }

    pub fn handle_ability(&mut self) {
        println!("{}", "\n─── CHOOSE ABILITY / COMBAT MANEUVER ───".bold().yellow());
        if self.player_role == PlayerRole::Survivor {
            let abilities = self.survivor.abilities();
            for (i, ab) in abilities.iter().enumerate() {
                println!("  [{}] {} — {}", i + 1, ab.name.bold().green(), ab.desc);
            }
            println!("  [0] Cancel");
            print!("\n  ꥟ Select [0-{}]: ", abilities.len());
            let _ = io::stdout().flush();

            let mut buf = String::new();
            if io::stdin().read_line(&mut buf).is_ok() {
                let choice: usize = buf.trim().parse().unwrap_or(99);
                if choice >= 1 && choice <= abilities.len() {
                    self.execute_survivor_ability(choice - 1);
                }
            }
        } else {
            let abilities = self.killer.abilities();
            for (i, ab) in abilities.iter().enumerate() {
                println!("  [{}] {} — {}", i + 1, ab.name.bold().red(), ab.desc);
            }
            println!("  [0] Cancel");
            print!("\n  ꥟ Select [0-{}]: ", abilities.len());
            let _ = io::stdout().flush();

            let mut buf = String::new();
            if io::stdin().read_line(&mut buf).is_ok() {
                let choice: usize = buf.trim().parse().unwrap_or(99);
                if choice >= 1 && choice <= abilities.len() {
                    self.execute_killer_ability(choice - 1);
                }
            }
        }
    }

    fn execute_survivor_ability(&mut self, idx: usize) {
        let abilities = self.survivor.abilities();
        let ab = &abilities[idx];

        if ab.power < 0 {
            // Healing
            let heal = -ab.power;
            self.survivor.hp = (self.survivor.hp + heal).min(self.survivor.max_hp);
            if self.survivor.id == SurvivorId::Helperbot {
                self.survivor.resource_val = 1; // spent charges
                self.log.push(RPLogEntry {
                    speaker: self.survivor.name.to_string(),
                    role_tag: "SURVIVOR".to_string(),
                    text: format!("*activates {}* Expanding pulse grants Regeneration I & Resistance I (-20% damage)! Consumed all pulse charges. Restored {} HP! (Current HP: {}/{})", ab.name, heal, self.survivor.hp, self.survivor.max_hp),
                    is_narrative: false,
                });
            } else {
                self.log.push(RPLogEntry {
                    speaker: self.survivor.name.to_string(),
                    role_tag: "SURVIVOR".to_string(),
                    text: format!("*activates {}* Restored {} HP! (Current HP: {}/{})", ab.name, heal, self.survivor.hp, self.survivor.max_hp),
                    is_narrative: false,
                });
            }
        } else if ab.power > 0 {
            // Attack
            self.killer.hp = (self.killer.hp - ab.power).max(0);
            if self.survivor.id == SurvivorId::TwoTime {
                self.survivor.resource_val = (self.survivor.resource_val + 25).min(100);
            }
            self.log.push(RPLogEntry {
                speaker: self.survivor.name.to_string(),
                role_tag: "SURVIVOR".to_string(),
                text: format!("*executes {}* Striking {} for {} damage! (Killer HP: {}/{})", ab.name, self.killer.name, ab.power, self.killer.hp, self.killer.max_hp),
                is_narrative: false,
            });
        } else {
            // Evasion / Utility
            if self.survivor.id == SurvivorId::Helperbot {
                self.survivor.resource_val = (self.survivor.resource_val + 1).min(3);
                self.log.push(RPLogEntry {
                    speaker: self.survivor.name.to_string(),
                    role_tag: "SURVIVOR".to_string(),
                    text: format!("*executes {}* 10-second Inspire aura active! Nearby survivors gain Haste V (+50% CD speed). Gained +1 Pulse charge! (Stored charges: {}/3)", ab.name, self.survivor.resource_val),
                    is_narrative: false,
                });
            } else {
                self.log.push(RPLogEntry {
                    speaker: self.survivor.name.to_string(),
                    role_tag: "SURVIVOR".to_string(),
                    text: format!("*executes {}* Moving into defensive cover!", ab.name),
                    is_narrative: false,
                });
            }
        }

        self.update_tension();
        self.trigger_npc_response(&format!("used ability {}", ab.name));
    }

    fn execute_killer_ability(&mut self, idx: usize) {
        let abilities = self.killer.abilities();
        let ab = &abilities[idx];

        let dmg = ab.power;
        self.survivor.hp = (self.survivor.hp - dmg).max(0);

        self.log.push(RPLogEntry {
            speaker: self.killer.name.to_string(),
            role_tag: "KILLER".to_string(),
            text: format!("*unleashes {}* Inflicting {} devastating damage onto {}! (Survivor HP: {}/{})", ab.name, dmg, self.survivor.name, self.survivor.hp, self.survivor.max_hp),
            is_narrative: false,
        });

        self.check_second_life();
        self.update_tension();
        self.trigger_npc_response(&format!("survived killer ability {}", ab.name));
    }

    fn check_second_life(&mut self) {
        if self.survivor.id == SurvivorId::TwoTime && self.survivor.hp <= 0 && !self.survivor.second_life_used {
            self.survivor.second_life_used = true;
            self.survivor.hp = self.survivor.max_hp;
            self.survivor.resource_val = 0; // consumed oblation

            self.log.push(RPLogEntry {
                speaker: "꥟ THE RITUAL OF THE REBORN ꥟".to_string(),
                role_tag: "SPECIAL".to_string(),
                text: "A blinding neon white flash consumes the chamber! Antlers burst with pure luminescence as the Spawn emblem blazes on Two Time's chest! \"GLORY TO THE SPAWN!\" Two Time earns their SECOND LIFE!".to_string(),
                is_narrative: true,
            });

            if self.killer.id == KillerId::Azure {
                self.log.push(RPLogEntry {
                    speaker: "Azure (Aspen)".to_string(),
                    role_tag: "KILLER".to_string(),
                    text: "*tentacles convulse as Aspen reels back in bitter fury* \"No... NO! That glowing curse... you sacrificed my life to buy yourself another?! CURSE YOU, TWO TIME!\"".to_string(),
                    is_narrative: false,
                });
            }
        }
    }

    pub fn handle_freeform_rp(&mut self) {
        println!("{}", "\n─── FREEFORM ROLEPLAY (SAY OR ACT) ───".bold().yellow());
        println!("  Describe actions with *asterisks* and dialogue in \"quotes\".");
        println!("  Example: *grips weapon tight* \"You won't escape this realm alive!\"");
        print!("\n  ꥟ Enter roleplay: ");
        let _ = io::stdout().flush();

        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_ok() {
            let input = buf.trim();
            if !input.is_empty() {
                self.record_user_dialogue(input);
                self.process_combat_keywords(input);
                self.trigger_npc_response(input);
            }
        }
    }

    fn record_user_dialogue(&mut self, text: &str) {
        let (speaker, tag) = if self.player_role == PlayerRole::Survivor {
            (self.survivor.name.to_string(), "SURVIVOR".to_string())
        } else {
            (self.killer.name.to_string(), "KILLER".to_string())
        };

        self.log.push(RPLogEntry {
            speaker,
            role_tag: tag,
            text: text.to_string(),
            is_narrative: false,
        });
    }

    fn process_combat_keywords(&mut self, input: &str) {
        let low = input.to_lowercase();
        if self.player_role == PlayerRole::Survivor {
            if low.contains("stab") || low.contains("slash") || low.contains("strike") || low.contains("shoot") || low.contains("hit") || low.contains("cleave") || low.contains("kick") {
                let dmg = 20;
                self.killer.hp = (self.killer.hp - dmg).max(0);
                if self.survivor.id == SurvivorId::TwoTime {
                    self.survivor.resource_val = (self.survivor.resource_val + 20).min(100);
                }
            } else if low.contains("heal") || low.contains("cola") || low.contains("pizza") || low.contains("chicken") || low.contains("patch") {
                let heal = 25;
                self.survivor.hp = (self.survivor.hp + heal).min(self.survivor.max_hp);
            }
        } else {
            // User is Killer attacking
            if low.contains("slash") || low.contains("cleave") || low.contains("strike") || low.contains("rend") || low.contains("pounce") || low.contains("tentacle") || low.contains("choke") || low.contains("chainsaw") || low.contains("bite") || low.contains("hook") {
                let dmg = 25;
                self.survivor.hp = (self.survivor.hp - dmg).max(0);
                self.check_second_life();
            }
        }
        self.update_tension();
    }

    fn trigger_npc_response(&mut self, user_input: &str) {
        self.turn += 1;
        let response = if self.player_role == PlayerRole::Survivor {
            self.generate_killer_npc_response(user_input)
        } else {
            self.generate_survivor_npc_response(user_input)
        };

        let (speaker, tag) = if self.player_role == PlayerRole::Survivor {
            (self.killer.name.to_string(), "KILLER".to_string())
        } else {
            (self.survivor.name.to_string(), "SURVIVOR".to_string())
        };

        self.log.push(RPLogEntry {
            speaker,
            role_tag: tag,
            text: response,
            is_narrative: false,
        });

        if self.turn % 3 == 0 {
            self.generate_environmental_beat();
        }
    }

    fn generate_killer_npc_response(&mut self, input: &str) -> String {
        let low = input.to_lowercase();
        match self.killer.id {
            KillerId::Azure => {
                if self.survivor.id == SurvivorId::TwoTime {
                    if low.contains("aspen") || low.contains("sorry") || low.contains("regret") || low.contains("love") || low.contains("knife") {
                        "*tentacles whip violently against the stone floor, dark purple veins throbbing* \"Do not speak that name with those lips! You left Aspen in the blood-soaked soil for a false promise! Look at what The Spectre turned me into!\"".to_string()
                    } else if low.contains("spawn") || low.contains("glory") || low.contains("ritual") || low.contains("oblation") || low.contains("second life") {
                        "*seeker bulbs emit a blinding biological flare* \"Your Spawn is a hollow monster! Every life you steal only drags your soul deeper into the void! Feel my thorns!\"".to_string()
                    } else if low.contains("blade") || low.contains("dagger") || low.contains("nice knife") {
                        "*recoils with a hissing growl, touching the old puncture wound* \"The Ghostfire Dagger... I still remember the chill of that steel. This time, my roots will pierce you through!\"".to_string()
                    } else {
                        let replies = [
                            "*four massive dark tentacles rear up, blocking out the light* \"The fleeting sound of the wind sounds softer today than usual... your heartbeat will soon match it.\"",
                            "*strides forward, seeker bulbs blooming with toxic crimson pollen* \"May the hand of nature be more merciful than human folly! Prepare yourself, Two Time!\"",
                            "*enstrangles the air with a savage lash* \"You cannot run from the garden of your own sins!\"",
                        ];
                        replies[self.turn as usize % replies.len()].to_string()
                    }
                } else if self.survivor.id == SurvivorId::Helperbot {
                    let replies = [
                        "*tentacles lash against cardboard and foil* \"Builderman's creation... your pulses cannot cleanse the rot Two Time left behind!\"",
                        "*seeker bulbs spit caustic purple spores* \"Cardboard wilts easily in poisonous mist, little automaton.\"",
                        "*whistles a mournful, hollow tune* \"The fleeting sound of the wind sounds softer today than usual... even for hollow tin.\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                } else {
                    let replies = [
                        "*tentacles coil tightly around broken stone* \"May the hand of nature be more merciful than human folly. You have strayed into my territory.\"",
                        "*seeker bulbs detonate with a sickening pop, spraying caustic spores* \"The garden demands fresh sustenance...\"",
                        "*deep purple eyes glint from the darkness* \"I hear your footsteps in the wind. There is nowhere to hide.\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                }
            }
            KillerId::Slasher => {
                if low.contains("chicken") || low.contains("telamon") || low.contains("shedletsky") {
                    "*chainsaw roars violently as sparks spray off the concrete* \"Telamon... your ancient steel will shatter under the weight of the Necrobloxicon! DIE!\"".to_string()
                } else {
                    let replies = [
                        "*heavy wet breathing rattles behind the mask as the rusted machete scrapes sparks from the floor* \"Flesh... tears... like dry paper...\"",
                        "*revs the chainsaw to an ear-splitting howl, lunging forward with thundering footsteps* \"NO ESCAPE!\"",
                        "*tilts head as the Necrobloxicon embedded in his shoulder pulses with black ichor* \"More... blood... for the pages...\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                }
            }
            KillerId::C00lkidd => {
                if self.survivor.id == SurvivorId::ZeroZeroSevenNSeven {
                    let replies = [
                        "*red aura flares as error codes dance across his skin* \"Stop calling me son! I'm the leader of Team c00lkidd now! Watch this script!\"",
                        "*hurls exploding pizza minions at his father* \"You left me behind, Dad! Now I rewrite the rules of this realm!\"",
                        "*teleports with crimson glitch streaks* \"Your parental firewall is already breached!\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                } else if self.survivor.id == SurvivorId::Helperbot {
                    let replies = [
                        "*cackles as flaming pizza scorches aluminum foil* \"HAHAHA! Look at the cardboard burn! Obsolete bots get no mercy from Team c00lkidd!\"",
                        "*red aura crackles violently* \"D3functB0t belongs in the scrap bin, not in my trial!\"",
                        "*overrides collision with a screeching glitch* \"Try scanning through this error code, toaster!\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                } else {
                    let replies = [
                        "*red aura flares as error codes dance across his skin* \"HAHAHA! Did you really think you could outrun a walkspeed override?! Join Team c00lkidd today!\"",
                        "*hurls a volley of corrupted flaming pizza slices* \"Special hot delivery! Extra fire, no refund!\"",
                        "*teleports three feet closer in a glitching blink* \"I'm rewriting your hitboxes right now! Say goodbye to your server connection!\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                }
            }
            KillerId::JohnDoe => {
                if self.survivor.id == SurvivorId::JaneDoe {
                    let replies = [
                        "*digital static screams as yellow corrupted spikes erupt* \"01001010... Jane... keep away... The code is forcing my hand!\"",
                        "*flickers erratically through the mist, reaching out a glitching hand* \"March 18th... I cannot remember my own face...\"",
                        "*whispers in discordant synthesized voices* \"The Spectre... rewrites... everything...\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                } else {
                    let replies = [
                        "*digital static screams as yellow corrupted spikes erupt from the tiles* \"01000100 01001111 01000101... March 18th is your final date.\"",
                        "*flickers erratically through the mist* \"Your memory address has been marked for termination. Resistance causes system instability.\"",
                        "*whispers in discordant synthesized voices* \"Feel the code collapse beneath your feet...\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                }
            }
            KillerId::OneByOne => {
                let replies = [
                    "*shattered emerald blades spin in a humming vortex* \"You dare speak to a creature of the fourth dimension? Your whole reality is a flimsy parchment I shall shred!\"",
                    "*slices space itself, leaving a rift of howling void* \"The Necrobloxicon foretold the geometry of your ruin! Perish, mortal!\"",
                    "*radiates a piercing blinding green light* \"Insignificant speck, dissolve before my entropy!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            KillerId::Guest666 => {
                let replies = [
                    "*DEAFENING DEMONIC ROAR SHAKES THE ENTIRE ARENA* \"THE FORGOTTEN GUESTS DEMAND YOUR FLESH!\"",
                    "*scratches razor obsidian claws against the floor, spitting hellfire* \"ROOOAAAR! NO MORTAL ESCAPES 666!\"",
                    "*pounces forward with demonic fury, red eyes blazing with hatred* \"YOU WILL BURN WITH THE REST OF THEM!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            KillerId::Noli => {
                let replies = [
                    "*void star pulses with black light as decoy illusions mirror his laugh* \"Look around you. Which one is real? Which one holds the blade?\"",
                    "*darts with Void Rush speed, leaving a trail of shattered space* \"You're fighting a myth from 2010. You cannot banish what is already forgotten.\"",
                    "*half-human face twists with cold amusement* \"The star's decay consumes everything... starting with your hope.\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            KillerId::Nosferatu => {
                let replies = [
                    "*wipes blood from his chin with aristocratic poise* \"Ah, the frantic rhythm of your terror... it seasons the blood wonderfully!\"",
                    "*cracks the heavy Blood Hook against the stone* \"Run if you wish! The hunter cherishes a fleeing banquet!\"",
                    "*eyes burn crimson with the Sin of Gluttony* \"Every drop you shed brings me closer to godhood in this twilight!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
        }
    }

    fn generate_survivor_npc_response(&mut self, input: &str) -> String {
        let low = input.to_lowercase();
        match self.survivor.id {
            SurvivorId::TwoTime => {
                if self.killer.id == KillerId::Azure {
                    if low.contains("dead") || low.contains("butcher") || low.contains("killed") || low.contains("grave") {
                        "*grips the Ghostfire Dagger with both trembling hands, tears gleaming in wide eyes* \"I had to do it, Aspen! The Spawn demanded sacrifice so we could be reborn together! Don't you see the antlers? Don't you see the light?!\"".to_string()
                    } else if low.contains("thorn") || low.contains("tentacle") || low.contains("strangle") {
                        "*dodges behind a stone pillar, gasping for breath* \"Never again shall I wonder... never again shall I hurt! GLORY TO THE SPAWN!\"".to_string()
                    } else {
                        "*manic smile falters into a solemn prayer* \"Tell me, dear... have you any regrets? The Spawn cleansed mine with cold steel, and it shall cleanse yours too!\"".to_string()
                    }
                } else {
                    let replies = [
                        "*spins the sleek dagger between fingerless gloves, eyes shining with fanatical reverence* \"Stand in silent prayer with me as we give praise... The Spawn will grant me another life!\"",
                        "*whispers under breath* \"Never again shall I wonder. Never again shall I hurt. Never again shall I love.\"",
                        "*slashes at the shadows with the Ghostfire Dagger* \"GLORY TO THE SPAWN!\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                }
            }
            SurvivorId::Chance => {
                let replies = [
                    "*flips the golden coin high into the air and catches it on the back of his hand* \"Heads, I put a hole in your coat. Tails, I do it twice! Feeling lucky?\"",
                    "*spins the cylinder of his revolver with a loud clatter* \"Fifty-fifty odds are the best kind of odds, monster!\"",
                    "*tips his fedora with a smirk* \"You're loud, ugly, and predictable. Let's see if your reflexes match your mouth!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            SurvivorId::Shedletsky => {
                let replies = [
                    "*brandishes the glowing Illumina sword, taking a defiant stance* \"HA! You think you scare Telamon?! I fought the oldest glitches in Robloxia!\"",
                    "*wolfs down a bite of crispy fried chicken* \"Nothing a crunchy bucket can't handle! Come get a taste of ancient admin steel!\"",
                    "*swings the blade in a dazzling arc of white light* \"Stand down before I reset your character!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            SurvivorId::Elliot => {
                let replies = [
                    "*holds a steaming pizza box as an improvised shield* \"I just deliver pizza! 30 minutes or less! Please don't murder the courier!\"",
                    "*frantically tosses a boiling slice of pizza at your eyes* \"Hot cheese hazard! Back off, monster!\"",
                    "*scrambles backward into the mist* \"I should've worked at the bloxburg burger place instead!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            SurvivorId::Guest1337 => {
                let replies = [
                    "*plants his tactical riot shield firmly into the earth* \"Hold the line! You will not pass this shield, beast!\"",
                    "*punches forward with reinforced combat knuckles* \"For every guest you ever hunted into the darkness! We stand our ground!\"",
                    "*steadies breathing through combat helmet* \"Target locked. Bring everything you have.\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            SurvivorId::Noob => {
                let replies = [
                    "*chugs an entire Bloxy Cola in two seconds and lets out a frantic burp* \"OOF! Why are you chasing me?! Go away!\"",
                    "*trips over a pebble, accidentally dodging under your swing* \"Woah! That was close! Run run run run!\"",
                    "*screams at the top of his lungs* \"SOMEBODY HELP! THE MONSTER IS RIGHT HERE!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            SurvivorId::ZeroZeroSevenNSeven => {
                if self.killer.id == KillerId::C00lkidd {
                    let replies = [
                        "*dodges behind a holographic clone, eyes locked on c00lkidd* \"I know you're still in there, son! I won't give up on you!\"",
                        "*injects a debug packet into the ground* \"Listen to your father's voice, not The Spectre's lies!\"",
                        "*stands firm despite the burns* \"You can throw all your fire at me... I'm staying right here!\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                } else {
                    let replies = [
                        "*fades through a desync packet, reappearing five paces back* \"You're swinging at cached coordinates, creature.\"",
                        "*types rapidly on script terminal* \"Security bypass complete. Your attacks are lagging.\"",
                        "*calmly checks holographic diagnostics* \"I've seen stronger malware on a calculator.\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                }
            }
            SurvivorId::Builderman => {
                let replies = [
                    "*tightens bolts on a fresh automated Sentry* \"You want to break Robloxia? You'll have to dismantle its builder first!\"",
                    "*taps the dispenser screen to top off health* \"Every stud in this arena answers to my blueprints!\"",
                    "*swings heavy pipe wrench in two-handed stance* \"Stand down or face server decommission!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            SurvivorId::Dusekkar => {
                let replies = [
                    "*raises the golden Ban Hammer as celestial sparks crackle* \"By the original system architecture, you are trespassing!\"",
                    "*chants ancient server protocols, golden shield glowing brightly* \"Your dark corruption has no root permissions here!\"",
                    "*points staff directly at the killer* \"Face the legacy of early Robloxia's chief architect!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            SurvivorId::JaneDoe => {
                if self.killer.id == KillerId::JohnDoe {
                    let replies = [
                        "*holds up the dossier files through the static* \"John! Look at the dates! Look at the cabin logs! You remember me!\"",
                        "*shines the taser light into John's eyes* \"I'm not leaving you behind in this simulation! Fight it!\"",
                        "*wipes blood from forehead* \"I don't care how corrupted you get... I will solve this!\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                } else {
                    let replies = [
                        "*flashes tactical flashlight straight into killer's optics* \"Blinded! Step into the light and face justice!\"",
                        "*checks notes in the confidential case file* \"Every hunter has a tell. I know your next move before you make it!\"",
                        "*readies taser counter-thrust* \"Come closer! Let's see how much voltage your heart can take!\"",
                    ];
                    replies[self.turn as usize % replies.len()].to_string()
                }
            }
            SurvivorId::Taph => {
                let replies = [
                    "*clicks remote detonator as tripwires snap tight* \"Snap! That's forty pounds of spring steel on your ankle!\"",
                    "*scatters razor-sharp caltrops behind with a sharp laugh* \"Try running now, bloodhound!\"",
                    "*ducks into shadowed bunker corner* \"You walked straight into my kill-box!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            SurvivorId::Veeronica => {
                let replies = [
                    "*grinds board along concrete edge, throwing shower of blue sparks* \"Too slow! Need a tutorial on how to catch a skater?!\"",
                    "*switches comm to broadcast mode with loud feedback whine* \"Bumper forcefield engaged! Bounce right back to the lobby!\"",
                    "*does a clean 360 kickflip over your swipe* \"Style points: ten out of ten! Terror score: zero!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
            SurvivorId::Helperbot => {
                let replies = [
                    "*emits an expanding harmonic pulse wave, antennas twitching* \"[^_^] Pulse deployed! Regeneration I and Resistance I active! Hang on, team!\"",
                    "*briefly pauses in place as glowing Inspire aura expands 14 studs* \"(b^_^)b Haste V activated! Cooldowns recovering fifty percent faster! We can do this!\"",
                    "*radar sweep scans every 12 seconds with a bright chime* \"[o_o] Unfinished generator highlighted within 100 studs! Don't give up!\"",
                ];
                replies[self.turn as usize % replies.len()].to_string()
            }
        }
    }

    fn generate_environmental_beat(&mut self) {
        let beats = match self.arena {
            Arena::RebornFamilyAbode => [
                "*A cold breeze sweeps across the altar, causing tallow candles to flicker violently. The symbol ꥟ glows with a faint bioluminescent hue.*",
                "*Choral chants echo faintly from subterranean catacombs beneath the stone oblation slabs.*",
                "*A drop of dark crimson ichor drips from the altar ceiling, splashing against the stone floor.*",
            ],
            Arena::BotanicalConservatory => [
                "*Mutated thorns tighten around the shattered greenhouse panes. A seeker bulb pulses with ominous purple light.*",
                "*The scent of crushed belladonna and bitter nightshade fills the humid air.*",
                "*Shattered glass crunches beneath unseen footsteps deep in the overgrown foliage.*",
            ],
            Arena::GlassHouses => [
                "*A sharp cracking sound echoes through the glass floor as fractured moonbeams shimmer across transparent beams.*",
                "*Distant gusts of wind rattle the crystal skybridges, threatening to cast anything standing into the void.*",
                "*Reflections in the emerald glass distort, showing spectral silhouettes that vanish in an instant.*",
            ],
            Arena::RobloxHQ => [
                "*Fluorescent ceiling tubes flicker erratically, buzzing with 60Hz static as rain lashes the windows.*",
                "*An abandoned office telephone rings shrilly across empty cubicles before cutting abruptly into static.*",
                "*The elevator cables in the lift shaft groan under sudden unseen tension.*",
            ],
            Arena::BuilderBrothersPizza => [
                "*The brick pizza oven roars with unnatural crimson fire, casting dancing demonic shadows over the greasy tiles.*",
                "*A metallic timer dings loudly from the back prep counter followed by the scraping of heavy butcher knives.*",
                "*Flickering neon signs buzz overhead, casting the entire dining room in bloody red hues.*",
            ],
            Arena::BloodfellManor => [
                "*Lightning tears through the stormy sky, illuminating ancient portraits of vampire nobility along the grand stairway.*",
                "*A grandfather clock chimes midnight with thirteen discordant tolls that reverberate through your bones.*",
                "*Velvet curtains whip in the rain as cold wind howls through broken stained-glass windows.*",
            ],
            Arena::FamiliarRuins => [
                "*Ancient Robloxia stone glyphs pulse with forgotten admin magic, illuminating crumbling marble steps.*",
                "*A hollow rumble echoes from the deep sanctuary beneath the cracked stone slabs.*",
                "*A flock of ravens bursts from the ruined archway, circling above the battleground.*",
            ],
            Arena::NaturalDisasterIsland => [
                "*Acid rain sizzles violently as it eats into the rocky shore, and an earthquake tremor shakes the watchtower.*",
                "*A bolt of green lightning strikes the lighthouse beacon, sending sparks showering across the sand.*",
                "*Crashing dark waves threaten to swallow the narrow perimeter of the island.*",
            ],
            Arena::CakeFactory => [
                "*Massive industrial conveyor belts screech to life, grinding rusty gears and churning thick frosting vats.*",
                "*Steam releases with a deafening blast from industrial pressure valves overhead.*",
                "*Heavy mechanical mixers drop into stainless steel bowls with bone-shaking impacts.*",
            ],
            Arena::YoricksRestingPlace => [
                "*Dense freezing mist snakes between weathered tombstones as distant church bells toll for the dead.*",
                "*The iron hinges of a crypt door slowly creak open in the shadow of the weeping willows.*",
                "*Pale will-o'-the-wisps hover over freshly disturbed earth, casting eerie green glows.*",
            ],
            Arena::Area51 => [
                "*Emergency klaxons wail continuously as rotating red warning beacons bathe the containment sector in crimson.*",
                "*A hydraulic pressure warning blares over the intercom: 'LEVEL 4 BIO-HAZARD CONTAINMENT BREACH'.*",
                "*Sparks spray from severed high-voltage conduits onto wet concrete floors.*",
            ],
            Arena::SFOTH => [
                "*Violent mountain gales howl across the floating cloud skybridges, making footing precarious above the abyss.*",
                "*A sword embedded in a stone pedestal hums with raw elemental Illumina resonance.*",
                "*Clouds part beneath the stone span, revealing thousands of studs of empty sky below.*",
            ],
            Arena::UndergroundWar => [
                "*Subterranean artillery shells detonate above, sending showers of mud and gravel down into the trench.*",
                "*Water trickles through rotting wooden supports as distant trench whistles pierce the smog.*",
                "*The acrid scent of cordite and damp sulfur fills every breath.*",
            ],
            Arena::PlanetVoss => [
                "*Two blood-red fractured moons eclipse overhead, casting the crimson dust dunes into alien twilight.*",
                "*Humming obsidian monoliths discharge arcs of dark cosmic lightning into the thin atmosphere.*",
                "*Red sand swirls into towering dust devils that scream across the desolate wastes.*",
            ],
        };

        let choice = beats[self.turn as usize % beats.len()];
        self.log.push(RPLogEntry {
            speaker: "꥟ AMBIENCE".to_string(),
            role_tag: "ENVIRONMENT".to_string(),
            text: choice.to_string(),
            is_narrative: true,
        });
    }

    fn update_tension(&mut self) {
        let s_ratio = self.survivor.hp as f32 / self.survivor.max_hp as f32;
        let k_ratio = self.killer.hp as f32 / self.killer.max_hp as f32;

        if self.survivor.second_life_used {
            self.tension = "RITUAL AWAKENED (SECOND LIFE)";
        } else if s_ratio < 0.25 || k_ratio < 0.25 {
            self.tension = "DESPERATE CLIMAX";
        } else if s_ratio < 0.55 || k_ratio < 0.55 {
            self.tension = "BLOODY CONFRONTATION";
        } else if self.turn > 2 {
            self.tension = "FIERCE DUEL";
        } else {
            self.tension = "TENSE STALKING";
        }
    }

    pub fn handle_emotional_stance(&mut self) {
        println!("{}", "\n─── CHANGE EMOTIONAL STANCE ───".bold().yellow());
        println!("  [1] Defiant (Unwavering resolve, standing ground)");
        println!("  [2] Sorrowful & Regretful (Mourning past tragedies and lost bonds)");
        println!("  [3] Fanatical (Glory to The Spawn / Devout frenzy)");
        println!("  [4] Cold & Calculating (Emotionless, analytical strategy)");
        println!("  [5] Terrified & Panicked (Frantic desperation, fighting for life)");
        println!("  [6] Mocking & Taunting (Arrogant banter, baiting the opponent)");
        println!("  [0] Keep Current");
        print!("\n  ꥟ Select stance [0-6]: ");
        let _ = io::stdout().flush();

        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_ok() {
            match buf.trim() {
                "1" => self.stance = Stance::Defiant,
                "2" => self.stance = Stance::Sorrowful,
                "3" => self.stance = Stance::Fanatical,
                "4" => self.stance = Stance::Cold,
                "5" => self.stance = Stance::Terrified,
                "6" => self.stance = Stance::Mocking,
                _ => {}
            }
        }
    }

    pub fn handle_codex(&self) {
        print!("\x1b[2J\x1b[1;1H");
        println!("{}", "╔════════════════════════════════════════════════════════════════════════════════════════════════════════╗".bright_blue());
        println!("║ {:<102} ║", "꥟ FORSAKEN LORE CODEX — VERIFIED FROM FORSAKEN.WIKI".bold().truecolor(122, 165, 214));
        println!("{}", "╠════════════════════════════════════════════════════════════════════════════════════════════════════════╣".bright_blue());

        // Survivor Codex
        println!("║ {} {:<93} ║", "[SURVIVOR]".bold().green(), format!("{} — {} ({})", self.survivor.name, self.survivor.title, self.survivor.price).bold());
        println!("║ Class: {:<15} Weapon: {:<40} ║", self.survivor.class_type.cyan(), self.survivor.weapon.yellow());
        println!("║ Lore: {:<96} ║", "");
        for line in textwrap_simple(self.survivor.lore_summary, 96) {
            println!("║   {:<94} ║", line);
        }

        println!("{}", "╟────────────────────────────────────────────────────────────────────────────────────────────────────────╢".bright_blue());

        // Killer Codex
        println!("║ {} {:<96} ║", "[KILLER]".bold().red(), format!("{} — {} ({})", self.killer.name, self.killer.title, self.killer.price).bold());
        println!("║ Weapon: {:<94} ║", self.killer.weapon.yellow());
        println!("║ Lore: {:<96} ║", "");
        for line in textwrap_simple(self.killer.lore_summary, 96) {
            println!("║   {:<94} ║", line);
        }

        println!("{}", "╟────────────────────────────────────────────────────────────────────────────────────────────────────────╢".bright_blue());
        // Matchup lore
        if self.survivor.id == SurvivorId::TwoTime && self.killer.id == KillerId::Azure {
            println!("║ {} {:<87} ║", "[LORE MATCHUP: TWO TIME & AZURE]".bold().magenta(), "THE SPAWN'S BETRAYAL");
            let pair_lore = "In official Forsaken lore, Two Time and Aspen (now Azure) belonged to Family Home For The Reborn (Spawn Cult). When Aspen discovered the sinister nature of the cult and wished to leave, Two Time sacrificed him with the Ghostfire Dagger to appease The Spawn. Resurrected by The Spectre into Azurewrath, Aspen seeks bitter revenge, while Two Time gained the power of Second Life yet carries unspoken torment.";
            for line in textwrap_simple(pair_lore, 96) {
                println!("║   {:<94} ║", line.italic());
            }
        } else if self.survivor.id == SurvivorId::ZeroZeroSevenNSeven && self.killer.id == KillerId::C00lkidd {
            println!("║ {} {:<85} ║", "[LORE MATCHUP: 007n7 & c00lkidd]".bold().magenta(), "FATHER & PRODIGAL SON");
            let pair_lore = "In Forsaken lore, 007n7 was an infamous early Roblox hacker who adopted c00lkidd to give the boy a loving home. When c00lkidd fell under the sway of corrupted scripts and The Spectre, 007n7 ventured into the realm to rescue his adopted son, choosing compassion over destruction.";
            for line in textwrap_simple(pair_lore, 96) {
                println!("║   {:<94} ║", line.italic());
            }
        } else if self.survivor.id == SurvivorId::JaneDoe && self.killer.id == KillerId::JohnDoe {
            println!("║ {} {:<86} ║", "[LORE MATCHUP: JANE & JOHN DOE]".bold().magenta(), "THE MARCH 18TH ANOMALY");
            let pair_lore = "Jane Doe's journey began in the Cabin basement, searching for documents about John Doe and the mysterious March 18th event. Inside the realm, she confronts John Doe's corrupted digital shell, attempting to break The Spectre's hold through investigative evidence.";
            for line in textwrap_simple(pair_lore, 96) {
                println!("║   {:<94} ║", line.italic());
            }
        } else {
            println!("║ Source: https://forsaken.wiki — Playable Survivors & Killers lore database {:<26} ║", "");
        }

        println!("{}", "╚════════════════════════════════════════════════════════════════════════════════════════════════════════╝".bright_blue());
        println!("\nPress Enter to return to the Roleplay Arena...");
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf);
    }

    pub fn handle_relocate(&mut self) {
        println!("{}", "\n─── RELOCATE TO ANOTHER CANONICAL ARENA (forsaken.wiki) ───".bold().yellow());
        for (i, arena) in Arena::all().iter().enumerate() {
            println!("  [{:>2}] {}", i + 1, arena.name());
        }
        println!("  [ 0] Cancel");
        print!("\n  ꥟ Select Arena [0-{}]: ", Arena::all().len());
        let _ = io::stdout().flush();

        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_ok() {
            let choice: usize = buf.trim().parse().unwrap_or(99);
            if choice >= 1 && choice <= Arena::all().len() {
                self.arena = Arena::all()[choice - 1];
                self.log.push(RPLogEntry {
                    speaker: "꥟ RELOCATION".to_string(),
                    role_tag: "ENVIRONMENT".to_string(),
                    text: format!("The battle relocates to {}! {}", self.arena.name(), self.arena.description()),
                    is_narrative: true,
                });
            }
        }
    }
}

fn textwrap_simple(text: &str, max_len: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let words = text.split_whitespace();
    let mut cur_line = String::new();

    for w in words {
        if cur_line.is_empty() {
            cur_line.push_str(w);
        } else if cur_line.len() + 1 + w.len() <= max_len {
            cur_line.push(' ');
            cur_line.push_str(w);
        } else {
            lines.push(cur_line);
            cur_line = w.to_string();
        }
    }
    if !cur_line.is_empty() {
        lines.push(cur_line);
    }
    lines
}

pub fn select_character_and_start() {
    let mut rl = match DefaultEditor::new() {
        Ok(e) => e,
        Err(_) => return,
    };

    loop {
        print!("\x1b[2J\x1b[1;1H");
        println!("{}", "╔════════════════════════════════════════════════════════════════════════════════════════════════════════╗".bright_blue());
        println!("║ {:<102} ║", "꥟ FORSAKEN ROLEPLAY: THE SPAWN'S ARENA (ROBLOX 2026 ROSTER) ꥟".bold().truecolor(122, 165, 214));
        println!("║ {:<102} ║", "\"A fragile cultist that holds a terrible secret... or an ancient hunter of the void?\"".italic());
        println!("║ {:<102} ║", "Lore Verified from official wiki: https://forsaken.wiki".cyan());
        println!("{}", "╠════════════════════════════════════════════════════════════════════════════════════════════════════════╣".bright_blue());
        println!("║ {:<102} ║", "CHOOSE YOUR PATH:");
        println!("║   {} Play as a SURVIVOR (13 Playable Classes: Two Time, Helperbot, 007n7...) {:<20} ║", "[1]".bold().green(), "");
        println!("║   {} Play as a KILLER (8 Playable Horrors: Azure, Slasher, c00lkidd, Noli, Nosferatu...) {:<13} ║", "[2]".bold().red(), "");
        println!("║   {} Canonical Lore Clashes (Two Time vs Azure, 007n7 vs c00lkidd, Jane vs John...) {:<17} ║", "[3]".bold().magenta(), "");
        println!("║   {} Read Full Forsaken Roster & Map Codex (November 2026 Database) {:<30} ║", "[4]".bold().yellow(), "");
        println!("║   {} Return to Two-Time Linux Shell {:<67} ║", "[0]".bold().red(), "");
        println!("{}", "╚════════════════════════════════════════════════════════════════════════════════════════════════════════╝".bright_blue());
        print!("\n  ꥟ Select [0-4]: ");
        let _ = io::stdout().flush();

        let line = match rl.readline("") {
            Ok(l) => l.trim().to_string(),
            Err(_) => break,
        };

        match line.as_str() {
            "1" => {
                if let Some((s_id, k_id, arena)) = prompt_survivor_setup(&mut rl) {
                    let mut session = RoleplaySession::new(PlayerRole::Survivor, s_id, k_id, arena);
                    run_roleplay_loop(&mut session, &mut rl);
                }
            }
            "2" => {
                if let Some((s_id, k_id, arena)) = prompt_killer_setup(&mut rl) {
                    let mut session = RoleplaySession::new(PlayerRole::Killer, s_id, k_id, arena);
                    run_roleplay_loop(&mut session, &mut rl);
                }
            }
            "3" => {
                if let Some((role, s_id, k_id, arena)) = prompt_lore_clash(&mut rl) {
                    let mut session = RoleplaySession::new(role, s_id, k_id, arena);
                    run_roleplay_loop(&mut session, &mut rl);
                }
            }
            "4" => {
                display_full_codex(&mut rl);
            }
            "0" | "exit" | "quit" => break,
            _ => continue,
        }
    }
}

fn prompt_survivor_setup(rl: &mut DefaultEditor) -> Option<(SurvivorId, KillerId, Arena)> {
    print!("\x1b[2J\x1b[1;1H");
    println!("{}", "─── SELECT YOUR SURVIVOR CHARACTER (All 13 as of Update 5.3.0 | https://forsaken.wiki) ───".bold().green());
    println!("  [ 1] Two Time       — Sentinel/Survivalist (Ghostfire Dagger, Oblation, Second Life ritual)");
    println!("  [ 2] Chance         — Sentinel (Loaded Six-Shooter, Lucky Gold Coin, Coin Flip Trick Shot)");
    println!("  [ 3] Shedletsky     — Sentinel (Telamon Illumina Sword, Crispy Fried Chicken)");
    println!("  [ 4] Elliot         — Support (Hot Pizza Delivery Boxes, Scalding Pizza Throw)");
    println!("  [ 5] Guest 1337     — Sentinel (Tactical Riot Shield, Counter Bash, Knuckles)");
    println!("  [ 6] Noob           — Survivalist (Bloxy Cola Speed, Panic Dodge, Iconic OOF!)");
    println!("  [ 7] 007n7          — Survivalist (Holographic Decoys, Packet Desync, Adoptive Father of c00lkidd)");
    println!("  [ 8] Builderman     — Support (Heavy Pipe Wrench, Automated Sentry, Medical Dispenser)");
    println!("  [ 9] Dusekkar       — Support (Golden Ban Hammer, Spawn Protection Shield, Staff of Protection)");
    println!("  [10] Jane Doe       — Sentinel/Support (Classified Dossier, Flashlight Taser, March 18th)");
    println!("  [11] Taph           — Support (Spring Trap Launcher, Spiked Tripwires, Caltrop Scatter)");
    println!("  [12] Veeronica      — Survivalist/Mobility (Neon Jet-Skateboard, Broadcast Bumper, Phase Shift)");
    println!("  [13] Helperbot      — Support (Radar passive, Pulse Regen/Resist, Inspire Haste V, Rebuilt D3functB0t)");
    println!("  [ 0] Back");
    print!("\n  ꥟ Choose Survivor [1-13]: ");
    let _ = io::stdout().flush();

    let s_id = match rl.readline("").ok()?.trim() {
        "1" => SurvivorId::TwoTime,
        "2" => SurvivorId::Chance,
        "3" => SurvivorId::Shedletsky,
        "4" => SurvivorId::Elliot,
        "5" => SurvivorId::Guest1337,
        "6" => SurvivorId::Noob,
        "7" => SurvivorId::ZeroZeroSevenNSeven,
        "8" => SurvivorId::Builderman,
        "9" => SurvivorId::Dusekkar,
        "10" => SurvivorId::JaneDoe,
        "11" => SurvivorId::Taph,
        "12" => SurvivorId::Veeronica,
        "13" => SurvivorId::Helperbot,
        _ => return None,
    };

    println!("\n{}", "─── SELECT OPPONENT KILLER TO ROLEPLAY AGAINST (https://forsaken.wiki) ───".bold().red());
    println!("  [1] Azure (Aspen)  — Trapper/Avenger (Seeker Bulbs, Stigmatize, Bio-Tentacles, Spawn Cult)");
    println!("  [2] Slasher (P13)  — Relentless Brute (Necrobloxicon, Heavy Machete, Chainsaw)");
    println!("  [3] c00lkidd       — Hacker Killer (Walkspeed Override, Corrupted Flaming Pizza)");
    println!("  [4] John Doe       — Digital Corruptor (March 18th Spikes, Binary Static, Crash Protocol)");
    println!("  [5] 1x1x1x1        — 4th-Dimensional Cosmic Deity (Shattered Emerald Blades of Entropy)");
    println!("  [6] Guest 666      — Demonic Behemoth (Blood Frenzy, Obsidian Claws, Hellfire Roar)");
    println!("  [7] Noli           — Void Star Harbinger (Void Rush, Hallucinatory Clones, 2010 Myth)");
    println!("  [8] Nosferatu      — Sin of Gluttony (Blood Hook Impalement, Vampiric Feast)");
    print!("\n  ꥟ Choose Killer [1-8]: ");
    let _ = io::stdout().flush();

    let k_id = match rl.readline("").ok()?.trim() {
        "1" => KillerId::Azure,
        "2" => KillerId::Slasher,
        "3" => KillerId::C00lkidd,
        "4" => KillerId::JohnDoe,
        "5" => KillerId::OneByOne,
        "6" => KillerId::Guest666,
        "7" => KillerId::Noli,
        "8" => KillerId::Nosferatu,
        _ => KillerId::Azure,
    };

    let arena = prompt_arena(rl);
    Some((s_id, k_id, arena))
}

fn prompt_killer_setup(rl: &mut DefaultEditor) -> Option<(SurvivorId, KillerId, Arena)> {
    print!("\x1b[2J\x1b[1;1H");
    println!("{}", "─── SELECT YOUR KILLER CHARACTER (All 8 as of November 2026 | https://forsaken.wiki) ───".bold().red());
    println!("  [1] Azure (Aspen)  — Trapper/Avenger (Seeker Bulbs, Stigmatize, Bio-Tentacles, Spawn Cult)");
    println!("  [2] Slasher (P13)  — Relentless Brute (Necrobloxicon, Heavy Machete, Chainsaw)");
    println!("  [3] c00lkidd       — Hacker Killer (Walkspeed Override, Corrupted Flaming Pizza)");
    println!("  [4] John Doe       — Digital Corruptor (March 18th Spikes, Binary Static, Crash Protocol)");
    println!("  [5] 1x1x1x1        — 4th-Dimensional Cosmic Deity (Shattered Emerald Blades of Entropy)");
    println!("  [6] Guest 666      — Demonic Behemoth (Blood Frenzy, Obsidian Claws, Hellfire Roar)");
    println!("  [7] Noli           — Void Star Harbinger (Void Rush, Hallucinatory Clones, 2010 Myth)");
    println!("  [8] Nosferatu      — Sin of Gluttony (Blood Hook Impalement, Vampiric Feast)");
    println!("  [0] Back");
    print!("\n  ꥟ Choose Killer [1-8]: ");
    let _ = io::stdout().flush();

    let k_id = match rl.readline("").ok()?.trim() {
        "1" => KillerId::Azure,
        "2" => KillerId::Slasher,
        "3" => KillerId::C00lkidd,
        "4" => KillerId::JohnDoe,
        "5" => KillerId::OneByOne,
        "6" => KillerId::Guest666,
        "7" => KillerId::Noli,
        "8" => KillerId::Nosferatu,
        _ => return None,
    };

    println!("\n{}", "─── SELECT TARGET SURVIVOR TO HUNT & ROLEPLAY WITH (https://forsaken.wiki) ───".bold().green());
    println!("  [ 1] Two Time       — Sentinel/Survivalist (Ghostfire Dagger, Oblation, Second Life ritual)");
    println!("  [ 2] Chance         — Sentinel (Loaded Six-Shooter, Lucky Gold Coin, Coin Flip Trick Shot)");
    println!("  [ 3] Shedletsky     — Sentinel (Telamon Illumina Sword, Crispy Fried Chicken)");
    println!("  [ 4] Elliot         — Support (Hot Pizza Delivery Boxes, Scalding Pizza Throw)");
    println!("  [ 5] Guest 1337     — Sentinel (Tactical Riot Shield, Counter Bash, Knuckles)");
    println!("  [ 6] Noob           — Survivalist (Bloxy Cola Speed, Panic Dodge, Iconic OOF!)");
    println!("  [ 7] 007n7          — Survivalist (Holographic Decoys, Packet Desync, Adoptive Father of c00lkidd)");
    println!("  [ 8] Builderman     — Support (Heavy Pipe Wrench, Automated Sentry, Medical Dispenser)");
    println!("  [ 9] Dusekkar       — Support (Golden Ban Hammer, Spawn Protection Shield, Staff of Protection)");
    println!("  [10] Jane Doe       — Sentinel/Support (Classified Dossier, Flashlight Taser, March 18th)");
    println!("  [11] Taph           — Support (Spring Trap Launcher, Spiked Tripwires, Caltrop Scatter)");
    println!("  [12] Veeronica      — Survivalist/Mobility (Neon Jet-Skateboard, Broadcast Bumper, Phase Shift)");
    println!("  [13] Helperbot      — Support (Radar passive, Pulse Regen/Resist, Inspire Haste V, Rebuilt D3functB0t)");
    print!("\n  ꥟ Choose Survivor [1-13]: ");
    let _ = io::stdout().flush();

    let s_id = match rl.readline("").ok()?.trim() {
        "1" => SurvivorId::TwoTime,
        "2" => SurvivorId::Chance,
        "3" => SurvivorId::Shedletsky,
        "4" => SurvivorId::Elliot,
        "5" => SurvivorId::Guest1337,
        "6" => SurvivorId::Noob,
        "7" => SurvivorId::ZeroZeroSevenNSeven,
        "8" => SurvivorId::Builderman,
        "9" => SurvivorId::Dusekkar,
        "10" => SurvivorId::JaneDoe,
        "11" => SurvivorId::Taph,
        "12" => SurvivorId::Veeronica,
        "13" => SurvivorId::Helperbot,
        _ => SurvivorId::TwoTime,
    };

    let arena = prompt_arena(rl);
    Some((s_id, k_id, arena))
}

fn prompt_lore_clash(rl: &mut DefaultEditor) -> Option<(PlayerRole, SurvivorId, KillerId, Arena)> {
    print!("\x1b[2J\x1b[1;1H");
    println!("{}", "─── SELECT CANONICAL FORSAKEN LORE CLASH ───".bold().magenta());
    println!("  [1] The Spawn's Betrayal: Two Time vs Azure (Aspen) [Cultist vs Vengeful Botanist]");
    println!("  [2] Father & Son Exploit: 007n7 vs c00lkidd [Reformed Hacker Father vs Rebel Son]");
    println!("  [3] The March 18th Mystery: Jane Doe vs John Doe [Cabin Investigator vs Digital Anomaly]");
    println!("  [4] Robloxia's Golden Age: Shedletsky vs 1x1x1x1 [Telamon Admin vs 4th-Dimensional Entity]");
    println!("  [5] Architect vs The Void: Builderman vs Noli [Platform Founder vs Void Star Myth]");
    println!("  [6] The Last Stand of the Guests: Guest 1337 vs Guest 666 [Heroic Veteran vs Demon]");
    println!("  [0] Back");
    print!("\n  ꥟ Choose Clash [1-6]: ");
    let _ = io::stdout().flush();

    let clash_choice = match rl.readline("").ok()?.trim() {
        "1" => (SurvivorId::TwoTime, KillerId::Azure, Arena::RebornFamilyAbode),
        "2" => (SurvivorId::ZeroZeroSevenNSeven, KillerId::C00lkidd, Arena::RobloxHQ),
        "3" => (SurvivorId::JaneDoe, KillerId::JohnDoe, Arena::FamiliarRuins),
        "4" => (SurvivorId::Shedletsky, KillerId::OneByOne, Arena::SFOTH),
        "5" => (SurvivorId::Builderman, KillerId::Noli, Arena::GlassHouses),
        "6" => (SurvivorId::Guest1337, KillerId::Guest666, Arena::UndergroundWar),
        _ => return None,
    };

    println!("\n  Which side do you wish to play?");
    println!("  [1] Play as Survivor");
    println!("  [2] Play as Killer");
    print!("\n  ꥟ Select [1/2]: ");
    let _ = io::stdout().flush();

    let role = match rl.readline("").ok()?.trim() {
        "2" => PlayerRole::Killer,
        _ => PlayerRole::Survivor,
    };

    Some((role, clash_choice.0, clash_choice.1, clash_choice.2))
}

fn prompt_arena(rl: &mut DefaultEditor) -> Arena {
    println!("\n{}", "─── SELECT ARENA / MAP (Canonical forsaken.wiki Map List) ───".bold().yellow());
    println!("  [ 1] Reborn Family Abode (Family Home For The Reborn Altar)");
    println!("  [ 2] Overgrown Botanical Conservatory (Aspen's Greenhouse)");
    println!("  [ 3] Glass Houses (Classic Roblox Glass Skyways)");
    println!("  [ 4] Roblox HQ (Corporate Cubicles & Rooftop)");
    println!("  [ 5] Builder Brothers Pizza (Haunted Kitchen & Ovens)");
    println!("  [ 6] Bloodfell Manor (Gothic Vampire Estate)");
    println!("  [ 7] Familiar Ruins (Ancient Robloxia Altars)");
    println!("  [ 8] Natural Disaster Island (Cataclysm Watchtower & Acid Storms)");
    println!("  [ 9] Cake Factory (Industrial Mixers & Conveyors)");
    println!("  [10] Yorick's Resting Place (Haunted Graveyard & Mausoleums)");
    println!("  [11] Area 51 (Underground Containment Labs)");
    println!("  [12] Sword Fight on the Heights (SFOTH Skybridges)");
    println!("  [13] Underground War (Trenches & Mud Bunkers)");
    println!("  [14] Planet Voss (Alien Crimson Dust Wasteland)");
    print!("\n  ꥟ Choose Arena [1-14]: ");
    let _ = io::stdout().flush();

    match rl.readline("").ok().unwrap_or_default().trim() {
        "2" => Arena::BotanicalConservatory,
        "3" => Arena::GlassHouses,
        "4" => Arena::RobloxHQ,
        "5" => Arena::BuilderBrothersPizza,
        "6" => Arena::BloodfellManor,
        "7" => Arena::FamiliarRuins,
        "8" => Arena::NaturalDisasterIsland,
        "9" => Arena::CakeFactory,
        "10" => Arena::YoricksRestingPlace,
        "11" => Arena::Area51,
        "12" => Arena::SFOTH,
        "13" => Arena::UndergroundWar,
        "14" => Arena::PlanetVoss,
        _ => Arena::RebornFamilyAbode,
    }
}

fn run_roleplay_loop(session: &mut RoleplaySession, rl: &mut DefaultEditor) {
    loop {
        session.render_screen();
        print!("  ꥟ Your Move [1: RP, 2: Lore, 3: Ability, 4: Stance, 5: Codex, 6: Move, 0: Exit]: ");
        let _ = io::stdout().flush();

        let line = match rl.readline("") {
            Ok(l) => l.trim().to_string(),
            Err(_) => break,
        };

        match line.as_str() {
            "1" => session.handle_freeform_rp(),
            "2" => session.handle_lore_dialogue(),
            "3" => session.handle_ability(),
            "4" => session.handle_emotional_stance(),
            "5" => session.handle_codex(),
            "6" => session.handle_relocate(),
            "0" | "exit" | "quit" => {
                println!("\n  ꥟ Departing from the Forsaken Roleplay Arena...\n");
                break;
            }
            _ => continue,
        }
    }
}

pub fn display_full_codex(rl: &mut DefaultEditor) {
    print!("\x1b[2J\x1b[1;1H");
    println!("{}", "╔════════════════════════════════════════════════════════════════════════════════════════════════════════╗".bright_blue());
    println!("║ {:<102} ║", "꥟ FORSAKEN WIKI COMPLETE ROSTER & MAP CODEX (NOVEMBER 2026) ꥟".bold().truecolor(122, 165, 214));
    println!("║ {:<102} ║", "Verified from: https://forsaken.wiki".cyan());
    println!("{}", "╠════════════════════════════════════════════════════════════════════════════════════════════════════════╣".bright_blue());
    println!("║ {:<102} ║", "SURVIVORS (13 Playable Classes):".bold().green());
    println!("║ • Two Time (Sentinel/Survivalist): Ghostfire Dagger, Oblation gauge, Second Life ritual. 500 PP.    ║");
    println!("║ • Chance (Sentinel): High-stakes gambler with loaded revolver and 50/50 coin-flip trick shots. 500 PP║");
    println!("║ • Shedletsky (Sentinel): Telamon Illumina blade, battle roar, and bucket of fried chicken. Free.    ║");
    println!("║ • Elliot (Support): Selfless pizza delivery boy, heals allies with pizza boxes, scalding sauce. Free ║");
    println!("║ • Guest 1337 (Sentinel): The Last Guest champion, tactical riot shield, counter bash, knuckles. Free ║");
    println!("║ • Noob (Survivalist): Classic robloxian, Bloxy Cola speed boosts, frantic dodges, iconic 'OOF!'. Free║");
    println!("║ • 007n7 (Survivalist): Reformed hacker, adoptive father of c00lkidd, holographic clones, desync. 600PP║");
    println!("║ • Builderman (Support): Master architect, constructs Stud Sentries and Medical Dispensers. 500 PP.  ║");
    println!("║ • Dusekkar (Support): Arch-engineer, Golden Ban Hammer, divine Spawn Protection shields. 600 PP.    ║");
    println!("║ • Jane Doe (Sentinel/Support): March 18th investigator, Cabin files, Flashlight taser. Quest unlock.║");
    println!("║ • Taph (Support): Strategic trapper, spring-loaded snares, caltrop scatter, field bandages. 500 PP. ║");
    println!("║ • Veeronica (Survivalist/Mobility): Rebellious cyber-skater, jet-board kickflips, bumper mode. 600 PP║");
    println!("║ • Helperbot (Support): Rebuilt D3functB0t, Radar scan, Pulse Regen/Resist, Inspire Haste V. 650 PP. ║");
    println!("{}", "╠════════════════════════════════════════════════════════════════════════════════════════════════════════╣".bright_blue());
    println!("║ {:<102} ║", "KILLERS (8 Playable Horrors & Myths):".bold().red());
    println!("║ • Azure / Aspen (Trapper/Area Denial): Sacrificed botanist, 4 black tentacles, Seeker Bulbs. $2000PP║");
    println!("║ • Slasher / Project 13 (Attrition): Undead behemoth guided by Necrobloxicon, machete & chainsaw. Free║");
    println!("║ • c00lkidd (Versatile): Leader of Team c00lkidd, 007n7's son, Walkspeed Override, flaming pizza. Free║");
    println!("║ • John Doe (Trapper/Set-play): March 18th glitch myth, digital corrupt spikes, crash protocol. Shop.║");
    println!("║ • 1x1x1x1 (Ranged/Attrition): 4th-dimensional cosmic deity, shattered emerald entropy blades. Shop. ║");
    println!("║ • Guest 666 (Burst/Snowball): Demonic banned guest, infernal cries, obsidian claws, blood frenzy. Shop║");
    println!("║ • Noli (Rushdown/Deception): 2010 platform myth, Void Star decay, Void Rush, hallucinatory decoys.  ║");
    println!("║ • Nosferatu (Sin of Gluttony): Blood Hook chain impalement, vampiric life drain, blood surge. $1,922 ║");
    println!("{}", "╠════════════════════════════════════════════════════════════════════════════════════════════════════════╣".bright_blue());
    println!("║ {:<102} ║", "CANONICAL MAPS (Public Rotation & Lore Arenas):".bold().yellow());
    println!("║ Reborn Family Abode • Botanical Conservatory • Glass Houses • Roblox HQ • Builder Brothers Pizza   ║");
    println!("║ Bloodfell Manor • Familiar Ruins • Natural Disaster Island • Cake Factory • Yorick's Resting Place  ║");
    println!("║ Area 51 • Sword Fight on the Heights (SFOTH) • Underground War • Planet Voss                        ║");
    println!("{}", "╚════════════════════════════════════════════════════════════════════════════════════════════════════════╝".bright_blue());
    println!("\nPress Enter to return to menu...");
    let _ = rl.readline("");
}

pub fn run_forsaken_roleplay(th: &Theme) {
    println!("\n  ꥟ Entering Forsaken Roleplay Engine (Theme: {})...", th.name);
    select_character_and_start();
}

pub fn run_quick_lore_clash() {
    let mut rl = match DefaultEditor::new() {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut session = RoleplaySession::new(
        PlayerRole::Survivor,
        SurvivorId::TwoTime,
        KillerId::Azure,
        Arena::RebornFamilyAbode,
    );
    run_roleplay_loop(&mut session, &mut rl);
}

pub fn handle_cli_rp_args(args: &[String], th: &Theme) {
    if args.iter().any(|a| a == "--quick" || a == "quick") {
        run_quick_lore_clash();
        return;
    }
    if args.iter().any(|a| a == "--codex" || a == "codex") {
        if let Ok(mut rl) = DefaultEditor::new() {
            display_full_codex(&mut rl);
        }
        return;
    }
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "help") {
        println!("{}", "\n─── FORSAKEN TUI ROLEPLAY USAGE ───".bold().cyan());
        println!("  two-time rp                     - Interactive character & realm selection");
        println!("  two-time rp quick               - Quick Lore Clash: Two Time vs Azure (Aspen)");
        println!("  two-time rp codex               - Read full November 2026 Forsaken Roster Codex");
        println!("  two-time rp --survivor <name> --killer <name> [--map <name>] [--role <survivor|killer>]");
        println!("  Example: two-time rp --survivor twotime --killer azure --map reborn");
        println!("  Example: two-time rp --survivor 007n7 --killer coolkidd --map robloxhq");
        println!("  Source: https://forsaken.wiki\n");
        return;
    }

    let mut s_id = None;
    let mut k_id = None;
    let mut arena_id = None;
    let mut role = PlayerRole::Survivor;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--survivor" | "-s" | "survivor" => {
                if let Some(val) = args.get(i + 1) {
                    s_id = SurvivorId::from_str(val);
                }
            }
            "--killer" | "-k" | "killer" => {
                if let Some(val) = args.get(i + 1) {
                    k_id = KillerId::from_str(val);
                }
            }
            "--map" | "--arena" | "-m" | "map" | "arena" => {
                if let Some(val) = args.get(i + 1) {
                    arena_id = Arena::from_str(val);
                }
            }
            "--role" | "-r" | "role" => {
                if let Some(val) = args.get(i + 1) {
                    if val.to_lowercase().contains("killer") {
                        role = PlayerRole::Killer;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }

    if let (Some(s), Some(k)) = (s_id, k_id) {
        let arena = arena_id.unwrap_or(Arena::RebornFamilyAbode);
        if let Ok(mut rl) = DefaultEditor::new() {
            let mut session = RoleplaySession::new(role, s, k, arena);
            run_roleplay_loop(&mut session, &mut rl);
            return;
        }
    }

    run_forsaken_roleplay(th);
}
