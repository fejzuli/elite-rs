use serde::{de, Deserialize, Deserializer};
use time::OffsetDateTime;

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        invalid => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(invalid as u64),
            &"Must be zero or one",
        )),
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event", rename_all_fields = "PascalCase")]
pub enum Event {
    /// First event in every journal file
    Fileheader {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        /// Part number of this journal.
        /// When a journal reaches 500k lines it will contain an Event::Continue
        /// pointing to the next part.
        #[serde(rename = "part")]
        part: u32,
        odyssey: bool,
        /// Language code in the form of "German/DE", "French/FR" etc.
        #[serde(rename = "language")]
        language: String,
        #[serde(rename = "gameversion")]
        gameversion: String,
        #[serde(rename = "build")]
        build: String,
    },

    // [[Startup]]

    /// Cargo information, written at startup.
    /// After startup other cargo events will be empty and signify that the
    /// Cargo.json file was updated.
    Cargo {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        vessel: Vessel,
        count: u32,
        #[serde(default)]
        inventory: Vec<CargoEntry>,
    },
    /// Save was cleared (who would ever do that?)
    ClearSavedGame {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        /// Player name
        name: String,
        /// Player ID
        #[serde(rename = "FID")]
        fid: String,
    },
    /// Written at start of load game process
    Commander {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        /// Player name
        name: String,
        /// Player ID
        #[serde(rename = "FID")]
        fid: String,
    },
    /// Written when loading from main menu, switching ship,
    /// changing ship in outfitting, or docking SRV back in ship.
    Loadout {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        /// Current ship type
        ship: String,
        #[serde(rename = "ShipID")]
        ship_id: u32,
        /// User defined ship name
        ship_name: String,
        /// User defined ship ID
        ship_ident: String,
        hull_value: u32,
        #[serde(default)]
        modules_value: u32,
        hull_health: f32,
        /// Mass of hull and modules excluding cargo and fuel
        unladen_mass: f32,
        fuel_capacity: FuelCapacity,
        cargo_capacity: u32,
        max_jump_range: f32,
        rebuy: u32,
        #[serde(default)]
        hot: bool,
        modules: Vec<Module>,
    },
    Materials {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        raw: Vec<Material>,
        manufactured: Vec<Material>,
        encoded: Vec<Material>,
    },
    Missions {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        active: Vec<Mission>,
        failed: Vec<Mission>,
        complete: Vec<Mission>,
    },
    /// Created a new commander
    NewCommander {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        /// Player name
        name: String,
        /// Player ID
        #[serde(rename = "FID")]
        fid: String,
        /// Selected starter package
        package: String,
    },
    LoadGame {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        /// Player name
        commander: String,
        /// Player ID
        #[serde(rename = "FID")]
        fid: String,
        horizons: bool,
        odyssey: bool,
    },
    Passengers {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        manifest: Vec<PassengerRecord>,
    },
    Powerplay {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        power: String,
        rank: u32,
        merits: u32,
        votes: u32,
        time_pledged: u32,
    },
    Progress {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        combat: u32,
        trade: u32,
        explore: u32,
        soldier: u32,
        exobiologist: u32,
        empire: u32,
        federation: u32,
        #[serde(rename = "CQC")]
        cqc: u32,
    },
    Rank {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        combat: u32,
        trade: u32,
        explore: u32,
        soldier: u32,
        exobiologist: u32,
        empire: u32,
        federation: u32,
        #[serde(rename = "CQC")]
        cqc: u32,
    },
    /// Players reputation with superpowers,
    /// written at startup and after rank and progress
    /// 
    /// Thresholds:
    /// hostile: -100..-90
    /// unfriendly: -90..-35
    /// neutral: -35..4
    /// cordial: 4..35
    /// friendly: 35..90
    /// allied: 90..100
    Reputation {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        empire: f32,
        federation: f32,
        independent: f32,
        alliance: f32,
    },
    Statistics {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        #[serde(rename = "Bank_Account")]
        bank_account: BankAccountStatistics,
        combat: CombatStatistics,
        crime: CrimeStatistics,
        smuggling: SmugglingStatistics,
        trading: TradingStatistics,
        mining: MiningStatistics,
        exploration: ExplorationStatistics,
        passengers: PassengersStatistics,
        #[serde(rename = "Search_And_Rescue")]
        search_and_rescue: SearchAndRescueStatistics,
        crafting: CraftingStatistics,
        crew: CrewStatistics,
        multicrew: MulticrewStatistics,
    },

    // [[Travel]]
    ApproachBody,
    Docked,
    DockingCancelled,
    DockingDenied,
    DockingGranted,
    DockingRequested,
    DockingTimeout,
    FSDJump,
    FSDTarget,
    LeaveBody,
    Liftoff,
    Location,
    StartJump,
    SupercruiseEntry,
    SupercruiseExit,
    Touchdown,
    Undocked,
    NavRoute,
    NavRouteClear,

    // [[Combat]]
    Bounty,
    CapShipBond,
    Died,
    EscapeInterdiction,
    FactionKillBond,
    FighterDestroyed,
    HeatDamage,
    HeatWarning,
    HullDamage,
    Interdicted,
    Interdiction,
    PVPKill,
    ShieldState,
    ShipTargeted,
    SRVDestroyed,
    UnderAttack,

    // [[Exploration]]
    CodexEntry,
    DiscoveryScan,
    Scan,
    FSSAllBodiesFound,
    FSSBodySignals,
    FSSDiscoveryScan,
    FSSSignalDiscovered,
    MaterialCollected,
    MaterialDiscarded,
    MaterialDiscovered,
    MultiSellExplorationData,
    NavBeaconScan,
    BuyExplorationData,
    SAAScanComplete,
    SAASignalsFound,
    ScanBaryCentre,
    SellExplorationData,
    Screenshot,

    // [[Trade]]
    AsteroidCracked,
    BuyTradeData,
    CollectCargo,
    EjectCargo,
    MarketBuy,
    MarketSell,
    MiningRefined,

    // [[Station Services]]
    BuyAmmo,
    BuyDrones,
    CargoDepot,
    CommunityGoal,
    CommunityGoalDiscard,
    CommunityGoalJoin,
    CommunityGoalReward,
    CrewAssign,
    CrewFire,
    CrewHire,
    EngineerApply,
    EngineerContribution,
    EngineerCraft,
    EngineerLegacyConvert,
    EngineerProgress,
    FetchRemoteModule,
    Market,
    MassModuleStore,
    MaterialTrade,
    MissionAbandoned,
    MissionAccepted,
    MissionCompleted,
    MissionFailed,
    MissionRedirected,
    ModuleBuy,
    ModuleRetrieve,
    ModuleSell,
    ModuleSellRemote,
    ModuleStore,
    ModuleSwap,
    Outfitting,
    PayBounties,
    PayFines,
    PayLegacyFines,
    RedeemVoucher,
    RefuelAll,
    RefuelPartial,
    Repair,
    RepairAll,
    RestockVehicle,
    ScientificResearch,
    SearchAndRescue,
    SellDrones,
    SellShipOnRebuy,
    SetUserShipName,
    Shipyard,
    ShipyardBuy,
    ShipyardNew,
    ShipyardSell,
    ShipyardTransfer,
    ShipyardSwap,
    StoredModules,
    StoredShips,
    TechnologyBroker,
    ClearImpound,

    // [[Powerplay]]
    PowerplayCollect,
    PowerplayDefect,
    PowerplayDeliver,
    PowerplayFastTrack,
    PowerplayJoin,
    PowerplayLeave,
    PowerplaySalary,
    PowerplayVote,
    PowerplayVoucher,

    // [[Squadrons]]
    AppliedToSquadron,
    DisbandedSquadron,
    InvitedToSquadron,
    JoinedSquadron,
    KickedFromSquadron,
    LeftSquadron,
    SharedBookmarkToSquadron,
    SquadronCreated,
    SquadronDemotion,
    SquadronPromotion,
    SquadronStartup,
    WonATrophyForSquadron,

    // [[Fleet Carriers]]
    CarrierJump,
    CarrierBuy,
    CarrierStats,
    CarrierJumpRequest,
    CarrierDecommission,
    CarrierCancelDecommission,
    CarrierBankTransfer,
    CarrierDepositFuel,
    CarrierCrewServices,
    CarrierFinance,
    CarrierShipPack,
    CarrierModulePack,
    CarrierTradeOrder,
    CarrierDockingPermission,
    CarrierNameChanged,
    CarrierJumpCancelled,

    // [[New in Odyssey]]
    Backpack,
    BackpackChange,
    BackpackMaterials,
    BookDropship,
    BookTaxi,
    BuyMicroResources,
    BuySuit,
    BuyWeapon,
    CancelDropship,
    CancelTaxi,
    CollectItems,
    CreateSuitLoadout,
    DeleteSuitLoadout,
    Disembark,
    DropItems,
    DropShipDeploy,
    Embark,
    FCMaterials,
    LoadoutEquipModule,
    LoadoutRemoveModule,
    RenameSuitLoadout,
    Resupply,
    ScanOrganic,
    SellMicroResources,
    SellOrganicData,
    SellSuit,
    SellWeapon,
    ShipLocker,
    SuitLoadout,
    SwitchSuitLoadout,
    TransferMicroResources,
    TradeMicroResources,
    UpgradeSuit,
    UpgradeWeapon,
    UseConsumable,

    // [[Other Events]]
    AfmuRepairs,
    ApproachSettlement,
    ChangeCrewRole,
    CockpitBreached,
    CommitCrime,
    Continued,
    CrewLaunchFighter,
    CrewMemberJoins,
    CrewMemberQuits,
    CrewMemberRoleChange,
    CrimeVictim,
    DatalinkScan,
    DatalinkVoucher,
    DataScanned,
    DockFighter,
    DockSRV,
    EndCrewSession,
    FighterRebuilt,
    FuelScoop,
    Friends,
    JetConeBoost,
    JetConeDamage,
    JoinACrew,
    KickCrewMember,
    LaunchDrone,
    LaunchFighter,
    LaunchSRV,
    ModuleInfo,
    Music,
    NpcCrewPaidWage,
    NpcCrewRank,
    Promotion,
    ProspectedAsteroid,
    QuitACrew,
    RebootRepair,
    /// Text message was received from another player or npc
    ReceiveText {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        #[serde(rename = "From")]
        from: String,
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "Channel")]
        channel: Channel,
    },
    RepairDrone,
    ReservoirReplenished,
    Resurrect,
    Scanned,
    SelfDestruct,
    /// Text message was sent to another player
    SendText {
        #[serde(with = "time::serde::iso8601", rename = "timestamp")]
        timestamp: OffsetDateTime,
        #[serde(rename = "To")]
        to: String,
        #[serde(rename = "Message")]
        message: String,
    },
    Shutdown,
    Synthesis,
    SystemsShutdown,
    USSDrop,
    VehicleSwitch,
    WingAdd,
    WingInvite,
    WingJoin,
    WingLeave,
    CargoTransfer,
    SupercruiseDestinationDrop,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    Wing,
    Local,
    VoiceChat,
    Friend,
    Player,
    Npc,
    Squadron,
    StarSystem,
}

#[derive(Debug, Deserialize)]
pub enum Vessel {
    Ship,
    SRV,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CargoEntry {
    pub name: String,
    pub count: u32,
    pub stolen: u32,
    #[serde(rename = "MissionID", default)]
    pub mission_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuelCapacity {
    pub main: f32,
    pub reserve: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    pub slot: String,
    pub item: String,
    pub on: bool,
    /// Power priority
    pub priority: u32,
    pub health: f32,
    #[serde(default)]
    pub value: u32,
    /// For passenger cabins this holds the number seats in the cabin
    #[serde(default)]
    pub ammo_in_clip: Option<u32>,
    #[serde(default)]
    pub ammo_in_hopper: Option<u32>,
    #[serde(default)]
    pub engineering: Option<Engineering>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Engineering {
    #[serde(rename = "EngineerID")]
    pub engineer_id: u32,
    /// Engineers name
    /// TODO: find out why this can be empty
    #[serde(default)]
    pub engineer: String,
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u32,
    pub blueprint_name: String,
    pub level: u32,
    pub quality: f32,
    #[serde(default)]
    pub experimental_effect: Option<String>,
    pub modifiers: Vec<EngineeringModifiers>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineeringModifiers {
    pub label: String,
    #[serde(default)]
    pub value: Option<f32>,
    pub original_value: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub less_is_good: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Material {
    pub name: String,
    pub count: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mission {
    #[serde(rename = "MissionID")]
    pub mission_id: u32,
    pub name: String,
    pub passenger_mission: bool,
    /// Time until mission expires in seconds
    pub expires: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengerRecord {
    #[serde(rename = "MissionID")]
    pub mission_id: u32,
    #[serde(rename = "Type")]
    pub passenger_type: String,
    #[serde(rename = "VIP")]
    pub vip: bool,
    pub wanted: bool,
    pub count: u32,
}

#[derive(Debug, Deserialize)]
pub struct BankAccountStatistics {
    #[serde(rename = "Current_Wealth")]
    pub current_wealth: u64,
    #[serde(rename = "Spent_On_Ships")]
    pub spent_on_ships: u64,
    #[serde(rename = "Spent_On_Outfitting")]
    pub spent_on_outfitting: u64,
    #[serde(rename = "Spent_On_Repairs")]
    pub spent_on_repairs: u64,
    #[serde(rename = "Spent_On_Fuel")]
    pub spent_on_fuel: u64,
    #[serde(rename = "Spent_On_Ammo_Consumables")]
    pub spent_on_ammo_consumables: u64,
    #[serde(rename = "Insurance_Claims")]
    pub insurance_claims: u32,
    #[serde(rename = "Spent_On_Insurance")]
    pub spent_on_insurance: u64,
    #[serde(rename = "Owned_Ship_Count")]
    pub owned_ship_count: u32,
    #[serde(rename = "Spent_On_Suits", default)]
    pub spent_on_suits: u64,
    #[serde(rename = "Spent_On_Weapons", default)]
    pub spent_on_weapons: u64,
    #[serde(rename = "Spent_On_Suit_Consumables", default)]
    pub spent_on_suit_consumables: u64,
    #[serde(rename = "Suits_Owned", default)]
    pub suits_owned: u32,
    #[serde(rename = "Weapons_Owned", default)]
    pub weapons_owned: u32,
    #[serde(rename = "Spent_On_Premium_Stock", default)]
    pub spent_on_premium_stock: u64,
    #[serde(rename = "Premium_Stock_Bought", default)]
    pub premium_stock_bought: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CombatStatistics {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrimeStatistics {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SmugglingStatistics {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TradingStatistics {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MiningStatistics {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExplorationStatistics {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersStatistics {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchAndRescueStatistics {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CraftingStatistics {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrewStatistics {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MulticrewStatistics {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct BoolFromInt {
        #[serde(deserialize_with = "bool_from_int")]
        val: bool,
    }

    #[test]
    fn test_bool_to_int_deserialization() {
        let data_true = r#"
            {
                "val": 1
            }
        "#;
        let res_true: Result<BoolFromInt, serde_json::Error> = serde_json::from_str(data_true);
        assert!(res_true.is_ok_and(|bfi| bfi.val));

        let data_false = r#"
            {
                "val": 0
            }
        "#;
        let res_false: Result<BoolFromInt, serde_json::Error> = serde_json::from_str(data_false);
        assert!(res_false.is_ok_and(|bfi| !bfi.val));
    }
}
