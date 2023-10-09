use std::fs::File;
use std::io::BufReader;

use rodio::decoder::LoopedDecoder;
use rodio::Decoder;

use crate::sound2::{Music as MusicTrait, Sfx as SfxTrait};

type MusicDecoder = LoopedDecoder<BufReader<File>>;

fn open_music(name: &str) -> MusicDecoder {
    let root = std::env::current_dir().unwrap();
    let file = File::open(root.join("music").join(name)).unwrap();

    Decoder::new_looped(BufReader::new(file)).unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Music {
    Title,
    Route1,
    Route3,
    Route12,
    MagnetTrain,
    KantoGymLeaderBattle,
    KantoTrainerBattle,
    KantoWildBattle,
    PokemonCenter,
    HikerEncounter,
    LassEncounter,
    OfficerEncounter,
    Heal,
    LavenderTown,
    Route2,
    MtMoon,
    ShowMeAround,
    GameCorner,
    Bicycle,
    HallOfFame,
    ViridianCity,
    CeladonCity,
    TrainerVictory,
    WildVictory,
    GymVictory,
    MtMoonSquare,
    Gym,
    PalletTown,
    PokemonTalk,
    ProfOak,
    RivalEncounter,
    RivalAfter,
    Surf,
    Evolution,
    NationalPark,
    Credits,
    AzaleaTown,
    CherrygroveCity,
    KimonoEncounter,
    UnionCave,
    JohtoWildBattle,
    JohtoTrainerBattle,
    Route30,
    EcruteakCity,
    VioletCity,
    JohtoGymLeaderBattle,
    ChampionBattle,
    RivalBattle,
    RocketBattle,
    ProfElm,
    DarkCave,
    Route29,
    Route36,
    SsAqua,
    YoungsterEncounter,
    BeautyEncounter,
    RocketEncounter,
    PokemaniacEncounter,
    SageEncounter,
    NewBarkTown,
    GoldenrodCity,
    VermilionCity,
    PokemonChannel,
    PokeFluteChannel,
    TinTower,
    SproutTower,
    BurnedTower,
    Lighthouse,
    LakeOfRage,
    IndigoPlateau,
    Route37,
    RocketHideout,
    DragonsDen,
    JohtoWildBattleNight,
    RuinsOfAlphRadio,
    Capture,
    Route26,
    Mom,
    VictoryRoad,
    PokemonLullaby,
    PokemonMarch,
    GsOpening,
    GsOpening2,
    MainMenu,
    RuinsOfAlphInterior,
    RocketOverture,
    DancingHall,
    BugCatchingContestRanking,
    BugCatchingContest,
    LakeOfRageRocketRadio,
    Printer,
    PostCredits,
    // New to Crystal
    Clair,
    MobileAdapterMenu,
    MobileAdapter,
    BuenasPassword,
    MysticalmanEncounter,
    CrystalOpening,
    BattleTowerTheme,
    SuicuneBattle,
    BattleTowerLobby,
    MobileCenter,
}

impl Music {
    pub fn from_id(id: u8) -> Option<Music> {
        match id {
            0x01 => Some(Music::Title),
            0x02 => Some(Music::Route1),
            0x03 => Some(Music::Route3),
            0x04 => Some(Music::Route12),
            0x05 => Some(Music::MagnetTrain),
            0x06 => Some(Music::KantoGymLeaderBattle),
            0x07 => Some(Music::KantoTrainerBattle),
            0x08 => Some(Music::KantoWildBattle),
            0x09 => Some(Music::PokemonCenter),
            0x0a => Some(Music::HikerEncounter),
            0x0b => Some(Music::LassEncounter),
            0x0c => Some(Music::OfficerEncounter),
            0x0d => Some(Music::Heal),
            0x0e => Some(Music::LavenderTown),
            0x0f => Some(Music::Route2),
            0x10 => Some(Music::MtMoon),
            0x11 => Some(Music::ShowMeAround),
            0x12 => Some(Music::GameCorner),
            0x13 => Some(Music::Bicycle),
            0x14 => Some(Music::HallOfFame),
            0x15 => Some(Music::ViridianCity),
            0x16 => Some(Music::CeladonCity),
            0x17 => Some(Music::TrainerVictory),
            0x18 => Some(Music::WildVictory),
            0x19 => Some(Music::GymVictory),
            0x1a => Some(Music::MtMoonSquare),
            0x1b => Some(Music::Gym),
            0x1c => Some(Music::PalletTown),
            0x1d => Some(Music::PokemonTalk),
            0x1e => Some(Music::ProfOak),
            0x1f => Some(Music::RivalEncounter),
            0x20 => Some(Music::RivalAfter),
            0x21 => Some(Music::Surf),
            0x22 => Some(Music::Evolution),
            0x23 => Some(Music::NationalPark),
            0x24 => Some(Music::Credits),
            0x25 => Some(Music::AzaleaTown),
            0x26 => Some(Music::CherrygroveCity),
            0x27 => Some(Music::KimonoEncounter),
            0x28 => Some(Music::UnionCave),
            0x29 => Some(Music::JohtoWildBattle),
            0x2a => Some(Music::JohtoTrainerBattle),
            0x2b => Some(Music::Route30),
            0x2c => Some(Music::EcruteakCity),
            0x2d => Some(Music::VioletCity),
            0x2e => Some(Music::JohtoGymLeaderBattle),
            0x2f => Some(Music::ChampionBattle),
            0x30 => Some(Music::RivalBattle),
            0x31 => Some(Music::RocketBattle),
            0x32 => Some(Music::ProfElm),
            0x33 => Some(Music::DarkCave),
            0x34 => Some(Music::Route29),
            0x35 => Some(Music::Route36),
            0x36 => Some(Music::SsAqua),
            0x37 => Some(Music::YoungsterEncounter),
            0x38 => Some(Music::BeautyEncounter),
            0x39 => Some(Music::RocketEncounter),
            0x3a => Some(Music::PokemaniacEncounter),
            0x3b => Some(Music::SageEncounter),
            0x3c => Some(Music::NewBarkTown),
            0x3d => Some(Music::GoldenrodCity),
            0x3e => Some(Music::VermilionCity),
            0x3f => Some(Music::PokemonChannel),
            0x40 => Some(Music::PokeFluteChannel),
            0x41 => Some(Music::TinTower),
            0x42 => Some(Music::SproutTower),
            0x43 => Some(Music::BurnedTower),
            0x44 => Some(Music::Lighthouse),
            0x45 => Some(Music::LakeOfRage),
            0x46 => Some(Music::IndigoPlateau),
            0x47 => Some(Music::Route37),
            0x48 => Some(Music::RocketHideout),
            0x49 => Some(Music::DragonsDen),
            0x4a => Some(Music::JohtoWildBattleNight),
            0x4b => Some(Music::RuinsOfAlphRadio),
            0x4c => Some(Music::Capture),
            0x4d => Some(Music::Route26),
            0x4e => Some(Music::Mom),
            0x4f => Some(Music::VictoryRoad),
            0x50 => Some(Music::PokemonLullaby),
            0x51 => Some(Music::PokemonMarch),
            0x52 => Some(Music::GsOpening),
            0x53 => Some(Music::GsOpening2),
            0x54 => Some(Music::MainMenu),
            0x55 => Some(Music::RuinsOfAlphInterior),
            0x56 => Some(Music::RocketOverture),
            0x57 => Some(Music::DancingHall),
            0x58 => Some(Music::BugCatchingContestRanking),
            0x59 => Some(Music::BugCatchingContest),
            0x5a => Some(Music::LakeOfRageRocketRadio),
            0x5b => Some(Music::Printer),
            0x5c => Some(Music::PostCredits),

            // New to Crystal
            0x5d => Some(Music::Clair),
            0x5e => Some(Music::MobileAdapterMenu),
            0x5f => Some(Music::MobileAdapter),
            0x60 => Some(Music::BuenasPassword),
            0x61 => Some(Music::MysticalmanEncounter),
            0x62 => Some(Music::CrystalOpening),
            0x63 => Some(Music::BattleTowerTheme),
            0x64 => Some(Music::SuicuneBattle),
            0x65 => Some(Music::BattleTowerLobby),
            0x66 => Some(Music::MobileCenter),

            _ => None,
        }
    }
}

impl SfxTrait<MusicDecoder> for Music {
    #[rustfmt::skip]
    fn open(self) -> MusicDecoder {
        match self {
            Music::Title => open_music("2-02. Title Screen (Crystal).flac"),
            Music::Route1 => open_music("1-85. Route 1.flac"),
            Music::Route3 => open_music("1-76. Route 3.flac"),
            Music::Route12 => open_music("1-72. Route 11.flac"),
            Music::MagnetTrain => open_music("1-75. Magnet Train.flac"),
            Music::KantoGymLeaderBattle => open_music("1-79. Battle! (Gym Leader - Kanto).flac"),
            Music::KantoTrainerBattle => open_music("1-77. Battle! (Trainer - Kanto).flac"),
            Music::KantoWildBattle => open_music("1-73. Battle! (Wild Pokémon - Kanto).flac"),
            Music::PokemonCenter => open_music("1-14. Pokémon Center.flac"),
            Music::HikerEncounter => open_music("1-31. Trainers' Eyes Meet (Hiker).flac"),
            Music::LassEncounter => open_music("1-27. Trainers' Eyes Meet (Lass).flac"),
            Music::OfficerEncounter => open_music("1-39. Trainers' Eyes Meet (Officer).flac"),
            Music::Heal => open_music("1-15. Pokémon Healed.flac"),
            Music::LavenderTown => open_music("1-74. Lavender Town.flac"),
            Music::Route2 => open_music("1-84. Viridian Forest.flac"),
            Music::MtMoon => open_music("1-82. Mt. Moon.flac"),
            Music::ShowMeAround => open_music("1-13. Hurry Along 2.flac"),
            Music::GameCorner => open_music("1-42. Game Corner.flac"),
            Music::Bicycle => open_music("1-43. Cycling.flac"),
            Music::HallOfFame => open_music("1-90. Hall of Fame.flac"),
            Music::ViridianCity => open_music("1-78. Pewter City.flac"),
            Music::CeladonCity => open_music("1-80. Celadon City.flac"),
            Music::TrainerVictory => open_music("1-21. Victory! (Trainer).flac"),
            Music::WildVictory => open_music("1-11. Victory! (Wild Pokémon).flac"),
            Music::GymVictory => open_music("1-35. Victory! (Gym Leader).flac"),
            Music::MtMoonSquare => open_music("1-83. Mt. Moon Square.flac"),
            Music::Gym => open_music("1-33. Pokémon Gym.flac"),
            Music::PalletTown => open_music("1-86. Pallet Town.flac"),
            Music::PokemonTalk => open_music("1-67. Radio Prof. Oak's Pokémon Talk.flac"),
            Music::ProfOak => open_music("1-18. Professor Oak.flac"),
            Music::RivalEncounter => open_music("1-37. Rival.flac"),
            Music::RivalAfter => open_music("1-65. Rival 2.flac"),
            Music::Surf => open_music("1-57. Surf.flac"),
            Music::Evolution => open_music("1-36. Evolution.flac"),
            Music::NationalPark => open_music("1-45. National Park.flac"),
            Music::Credits => open_music("1-91. Ending.flac"),
            Music::AzaleaTown => open_music("1-32. Azalea Town.flac"),
            Music::CherrygroveCity => open_music("1-12. Cherrygrove City.flac"),
            Music::KimonoEncounter => open_music("1-50. Trainers' Eyes Meet (Kimono Girl).flac"),
            Music::UnionCave => open_music("1-28. Union Cave.flac"),
            Music::JohtoWildBattle => open_music("1-09. Battle! (Wild Pokémon - Johto) (Day).flac"),
            Music::JohtoTrainerBattle => open_music("1-20. Battle! (Trainer - Johto).flac"),
            Music::Route30 => open_music("1-16. Route 30.flac"),
            Music::EcruteakCity => open_music("1-48. Ecruteak City.flac"),
            Music::VioletCity => open_music("1-23. Violet City.flac"),
            Music::JohtoGymLeaderBattle => open_music("1-34. Battle! (Gym Leader - Johto).flac"),
            Music::ChampionBattle => open_music("1-89. Battle! (Champion).flac"),
            Music::RivalBattle => open_music("1-38. Battle! (Rival).flac"),
            Music::RocketBattle => open_music("1-64. Battle! (Team Rocket).flac"),
            Music::ProfElm => open_music("1-07. Elm Pokémon Lab.flac"),
            Music::DarkCave => open_music("1-22. Dark Cave.flac"),
            Music::Route29 => open_music("1-08. Route 29.flac"),
            Music::Route36 => open_music("1-26. Route 32.flac"),
            Music::SsAqua => open_music("1-70. S.S. Aqua.flac"),
            Music::YoungsterEncounter => open_music("1-19. Trainers' Eyes Meet (Youngster).flac"),
            Music::BeautyEncounter => open_music("1-54. Trainers' Eyes Meet (Beauty).flac"),
            Music::RocketEncounter => open_music("1-63. Trainers' Eyes Meet (Team Rocket).flac"),
            Music::PokemaniacEncounter => open_music("1-60. Trainers' Eyes Meet (PokéManiac).flac"),
            Music::SageEncounter => open_music("1-25. Trainers' Eyes Meet (Sage).flac"),
            Music::NewBarkTown => open_music("1-05. New Bark Town.flac"),
            Music::GoldenrodCity => open_music("1-40. Goldenrod City.flac"),
            Music::VermilionCity => open_music("1-71. Vermilion City.flac"),
            Music::PokemonChannel => open_music("1-41. Radio Pokémon Channel.flac"),
            Music::PokeFluteChannel => open_music("1-81. Radio Poké Flute.flac"),
            Music::TinTower => open_music("1-52. Tin Tower.flac"),
            Music::SproutTower => open_music("1-24. Sprout Tower.flac"),
            Music::BurnedTower => open_music("1-51. Burned Tower.flac"),
            Music::Lighthouse => open_music("1-56. Olivine Lighthouse.flac"),
            Music::LakeOfRage => open_music("1-59. Route 42.flac"),
            Music::IndigoPlateau => open_music("1-88. Pokémon League.flac"),
            Music::Route37 => open_music("1-53. Route 38.flac"),
            Music::RocketHideout => open_music("1-62. Team Rocket HQ.flac"),
            Music::DragonsDen => open_music("1-68. Dragon's Den.flac"),
            Music::JohtoWildBattleNight => open_music("1-10. Battle! (Wild Pokémon - Johto) (Night).flac"),
            Music::RuinsOfAlphRadio => open_music("1-30. Radio Unown.flac"),
            Music::Capture => open_music("1-17. Pokémon Caught.flac"),
            Music::Route26 => open_music("1-69. Route 26.flac"),
            Music::Mom => open_music("1-06. Hurry Along.flac"),
            Music::VictoryRoad => open_music("1-87. Victory Road.flac"),
            Music::PokemonLullaby => open_music("1-55. Pokémon Lullaby.flac"),
            Music::PokemonMarch => open_music("1-44. Radio Pokémon March.flac"),
            Music::GsOpening => open_music("1-01. Opening Movie .flac"),
            Music::GsOpening2 => open_music("1-02. Opening Movie 2.flac"),
            Music::MainMenu => open_music("1-04. An Adventure Begins!.flac"),
            Music::RuinsOfAlphInterior => open_music("1-29. Ruins of Alph.flac"),
            Music::RocketOverture => open_music("1-66. Radio Tower Occupied!.flac"),
            Music::DancingHall => open_music("1-49. Ecruteak Dance Theater.flac"),
            Music::BugCatchingContestRanking => open_music("1-47. Bug-Catching Contest.flac"),
            Music::BugCatchingContest => open_music("1-46. Bug-Catching Contest (Preparing).flac"),
            Music::LakeOfRageRocketRadio => open_music("1-61. Radio Transmission.flac"),
            Music::Printer => open_music("1-58. Game Boy Printer.flac"),
            Music::PostCredits => open_music("1-92. The End.flac"),

            // New to Crystal
            Music::Clair => open_music("2-06. Clair.flac"),
            Music::MobileAdapterMenu => open_music("2-11. Mobile Adapter Menu (JP).flac"),
            Music::MobileAdapter => open_music("2-10. Mobile Adapter Screen (JP).flac"),
            Music::BuenasPassword => open_music("2-03. Radio Buena's Password.flac"),
            Music::MysticalmanEncounter => open_music("2-04. Eusine.flac"),
            Music::CrystalOpening => open_music("2-01. Opening Movie (Crystal).flac"),
            Music::BattleTowerTheme => open_music("2-07. Battle Tower.flac"),
            Music::SuicuneBattle => open_music("2-05. Battle! (RaikouEnteiSuicune).flac"),
            Music::BattleTowerLobby => open_music("2-08. Battle Tower Reception Desk.flac"),
            Music::MobileCenter => open_music("2-09. Pokémon Communication Center (JP).flac"),
        }
    }
}

impl MusicTrait<MusicDecoder> for Music {
    fn id(&self) -> u32 {
        *self as u32
    }
}
