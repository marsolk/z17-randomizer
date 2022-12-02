#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Location {

    // Hyrule -------------------

    HyruleBellTravel,
    HyruleField,
    EasternRuinsUpper,
    EasternRuinsEastLedge,
    WitchCave,
    WitchHouse,
    RavioShop,
    ZoraDomain,
    ZoraDomainArea,
    WaterfallCave,
    WaterfallCaveShallowWater,
    MergeDungeon,
    EastRuinsBombCaveUpper,
    EastRuinsBombCaveLower,
    HouseOfGalesIsland,
    RossoHouse,
    RossoCave,
    TornadoRodDungeon,
    GraveyardLedgeHyrule,
    GraveyardLedgeLorule,
    GraveyardLedgeCave,
    BlacksmithHouse,
    BlacksmithCave,
    HyruleCastleCourtyard,
    HyruleCastleLeftRoom,
    HyruleCastleRightRoom,
    HyruleCastleInterior,
    HyruleCastleRoof,
    HyruleCastleDungeon,
    LostWoods,
    MasterSwordArea,
    FortuneTeller,
    KakarikoJailCell,
    WellUpper,
    WellLower,
    StylishWomanHouse,
    MilkBar,
    BeeGuyHouse,
    KakarikoItemShop,
    LakesideItemShop,
    ItemSellerCave,
    FlippersDungeon,
    SouthernRuinsBombCave,
    SouthernRuinsPillars,
    LakeDarkCave,
    IceRodCave,
    Sanctuary,
    SanctuaryChurch,
    CuccoDungeonLedge,
    CuccoDungeon,
    WaterfallLedge,
    CuccoHouse,
    CuccoHouseRear,

    MoldormCave,
    MoldormCaveTop,
    MoldormLedge,

    DeathMountainBase,
    DeathWestLedge,
    DeathSecondFloor,
    DeathBombCave,
    DeathWeatherVaneCaveLeft,
    DeathFairyCave,
    DonkeyCaveLower,
    DonkeyCaveUpper,
    DeathThirdFloor,
    AmidaCaveLower,
    AmidaCaveUpper,
    DeathTopLeftLedge,
    DeathMountainWestTop,
    DeathMountainEastTop,
    SpectacleRock,
    SpectacleRockCaveLeft,
    SpectacleRockCaveRight,
    HookshotDungeon,
    FireCaveTop,
    FireCaveCenter,
    FireCaveMiddle,
    FireCaveBottom,
    BoulderingLedgeLeft,
    BoulderingLedgeBottom,
    BoulderingLedgeRight,
    RossosOreMine,
    FloatingIslandHyrule,


    // Lorule -------------------

    LoruleBellTravel,
    LoruleCastleField,
    BigBombFlowerShop,
    BigBombFlowerField,
    LoruleGraveyard,
    LoruleSanctuary,
    LoruleSanctuaryCaveLower,
    LoruleSanctuaryCaveUpper,
    KusDomainSouth,
    KusDomain,
    GreatRupeeFairyCave,
    LoruleBlacksmith,
    BootsDungeon,
    VacantHouseBottom,
    VacantHouseTop,
    ThiefGirlCave,
    SwampCave,
    BigBombCave,
    HauntedGroveLedge,

    Desert,
    DesertSouthWestLedge,
    DesertPalaceWeatherVane,
    DesertPalaceMidwayLedge,
    DesertZaganagaLedge,

    MiseryMire,
    SandRodDungeon,
    MiseryMireLedge,
    MiseryMireBridge,
    MiseryMireOoB,

    LoruleLakeWater,
    LoruleLakeEast,
    LoruleLakeNorthWest,
    LoruleLakeSouthWest,
    LakesideItemShopLorule,

    DarkRuins,
    DarkRuinsWeatherVane,
    DarkRuinsShallowWater,
    HinoxCaveWater,
    HinoxCaveShallowWater,
    HinoxCave,
    SkullWoodsOverworld,
    MysteriousManCave,

    RossosOreMineLorule,
    LoruleDeathWest,
    LoruleDeathEastTop,
    LoruleDeathEastLedgeUpper,
    LoruleDeathEastLedgeLower,

    IceCaveEast,
    IceCaveCenter,
    IceCaveWest,
    IceCaveNorthWest,
    IceCaveSouthWest,
    IceCaveSouth,

    FloatingIslandLorule,

    // Dungeons -----------------

    EasternPalaceFoyer,
    EasternPalace1F,
    EasternPalaceMiniboss,
    EasternPalace2F,
    EasternPalaceBoss,
    EasternPalacePostYuga,
    EasternPalaceEscape,

    HouseOfGalesFoyer,
    HouseOfGalesEast1F,
    HouseOfGalesWest1F,
    HouseOfGales2F,
    HouseOfGales3F,
    HouseOfGalesBoss,

    TowerOfHeraFoyer,
    TowerOfHeraBottom,
    TowerOfHeraMiddle,
    TowerOfHeraTop,
    TowerOfHeraBoss,

    DarkPalaceFoyer,
    DarkPalaceSecondRoom,
    DarkPalaceMain,
    DarkPalaceLockedDoors,
    DarkPalaceBoss,
    DarkPalaceAfterBoss,

    SwampPalaceOutside,
    SwampPalaceAntechamber,
    SwampPalaceFoyer,
    SwampPalaceMain,

    SkullWoodsFoyer,
    SkullWoodsMain,
    SkullWoodsB2,
    SkullWoodsElevatorHallway,
    SkullWoodsBossHallway,
    SkullWoodsEastB1NorthFoyer,
    SkullWoodsEastB1North,
    SkullWoodsEastB1SouthFoyer,
    SkullWoodsEastB1South,
    SkullWoodsEastB1SouthLedges,
    SkullWoodsOutdoor3,
    SkullWoodsBossRoom,
    SkullWoodsSeresGrove,

    ThievesHideoutB1,
    ThievesHideoutB2,
    ThievesHideoutEscape,
    ThievesBoss,
    ThievesPostBoss,

    TurtleRockWeatherVane,
    TurtleRockFrontDoor,
    TurtleRockFoyer,
    TurtleRockMain,
    TurtleRockLeftBalcony,
    TurtleRockLeftBalconyPath,
    TurtleRockRightBalcony,
    TurtleRockRightBalconyPath,
    TurtleRockBoss,
    TurtleRockPostBoss,

    DesertPalaceFoyer,
    DesertPalace1F,
    DesertPalace2F,
    DesertPalace3F,
    DesertPalaceExit3F,
    ZaganagasArena,
    MiseryMireRewardBasket,

    IceRuinsFoyer,
    IceRuins,
    IceRuinsBoss,
    IceRuinsPostBoss,

    ChamberOfSages,

    LoruleCastle1F,
    LoruleCastleEastLedge1F,
    LoruleCastleCenter1F,
    LoruleCastle2F3F,
    LoruleCastle4F5F,
    HildasStudy,
    ZeldasStudy,
    ThroneRoom,

    SacredRealm,
}