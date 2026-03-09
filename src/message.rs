#[derive(Debug)]
pub struct Message {
    pub protocol: String,           // Protocol
    pub command: String,            // Com
    pub piste: String,              // Piste
    pub competition: String,        // Compe
    pub phase: String,              // Phase
    pub pool_tab: String,           // PoulTab
    pub match_number: String,       // Match
    pub round: String,              // Round
    pub time: String,               // Time
    pub stopwatch: String,          // Stopwatch
    pub competition_type: String,   // Type
    pub weapon: String,             // Weapon
    pub priority: String,           // Priority
    pub state: String,              // State
    pub referee_id: String,         // RefId
    pub referee_name: String,       // RefName
    pub referee_nation: String,     // RefNat
    pub right_id: String,           // R1 RightId
    pub right_name: String,         // R2 RightName
    pub right_nation: String,       // R3 RightNat
    pub right_score: String,        // R4 Rscore
    pub right_status: String,       // R5 Rstatus
    pub right_yellow_cards: String, // R6 RYcard
    pub right_red_cards: String,    // R7 RRcard
    pub right_light: String,        // R8 RLight
    pub right_white_light: String,  // R9 RWlight
    pub right_medical: String,      // R10 RMedical
    pub right_reserve: String,      // R11 RReserve
    pub right_p_card: String,       // R12 RP-card
    pub left_id: String,            // L1 LeftId
    pub left_name: String,          // L2 LeftName
    pub left_nation: String,        // L3 LeftNat
    pub left_score: String,         // L4 Lscore
    pub left_status: String,        // L5 Lstatus
    pub left_yellow_cards: String,  // L6 LYcard
    pub left_red_cards: String,     // L7 LRcard
    pub left_light: String,         // L8 LLight
    pub left_white_light: String,   // L9 LWlight
    pub left_medical: String,       // L10 RMedical (left fencer)
    pub left_reserve: String,       // L11 LReserve
    pub left_p_card: String,        // L12 LP-card
}

impl Default for Message {
    fn default() -> Self {
        Self {
            protocol: "0".to_string(),
            command: "0".to_string(),
            piste: "0".to_string(),
            competition: "0".to_string(),
            phase: "0".to_string(),
            pool_tab: "0".to_string(),
            match_number: "0".to_string(),
            round: "0".to_string(),
            time: "0".to_string(),
            stopwatch: "0".to_string(),
            competition_type: "0".to_string(),
            weapon: "0".to_string(),
            priority: "0".to_string(),
            state: "0".to_string(),
            referee_id: "0".to_string(),
            referee_name: "0".to_string(),
            referee_nation: "0".to_string(),
            right_id: "0".to_string(),
            right_name: "0".to_string(),
            right_nation: "0".to_string(),
            right_score: "0".to_string(),
            right_status: "0".to_string(),
            right_yellow_cards: "0".to_string(),
            right_red_cards: "0".to_string(),
            right_light: "0".to_string(),
            right_white_light: "0".to_string(),
            right_medical: "0".to_string(),
            right_reserve: "0".to_string(),
            right_p_card: "0".to_string(),
            left_id: "0".to_string(),
            left_name: "0".to_string(),
            left_nation: "0".to_string(),
            left_score: "0".to_string(),
            left_status: "0".to_string(),
            left_yellow_cards: "0".to_string(),
            left_red_cards: "0".to_string(),
            left_light: "0".to_string(),
            left_white_light: "0".to_string(),
            left_medical: "0".to_string(),
            left_reserve: "0".to_string(),
            left_p_card: "0".to_string(),
        }
    }
}

impl Message {
    pub fn new(
        command: impl Into<String>,
        piste: impl Into<String>,
        competition: impl Into<String>,
    ) -> Self {
        Self {
            protocol: "EFP1.1".to_string(),
            command: command.into(),
            piste: piste.into(),
            competition: competition.into(),
            ..Default::default()
        }
    }

    pub fn display(piste: impl Into<String>, competition: impl Into<String>) -> Self {
        Self::new("DISP", piste, competition)
    }
}
