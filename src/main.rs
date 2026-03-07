use std::{io, net::UdpSocket};
#[derive(Debug)]
struct Message {
    protocol: String,          // Protocol
    command: String,           // Com
    piste: String,             // Piste
    competition: String,       // Compe
    phase: String,             // Phase
    pool_tab: String,          // PoulTab
    match_number: String,      // Match
    round: String,             // Round
    time: String,              // Time
    stopwatch: String,         // Stopwatch
    competition_type: String,  // Type
    weapon: String,            // Weapon
    priority: String,          // Priority
    state: String,             // State
    referee_id: String,        // RefId
    referee_name: String,      // RefName
    referee_nation: String,    // RefNat
    right_id: String,          // R1 RightId
    right_name: String,        // R2 RightName
    right_nation: String,      // R3 RightNat
    right_score: String,       // R4 Rscore
    right_status: String,      // R5 Rstatus
    right_yellow_cards: String,// R6 RYcard
    right_red_cards: String,   // R7 RRcard
    right_light: String,       // R8 RLight
    right_white_light: String, // R9 RWlight
    right_medical: String,     // R10 RMedical
    right_reserve: String,     // R11 RReserve
    right_p_card: String,      // R12 RP-card
    left_id: String,           // L1 LeftId
    left_name: String,         // L2 LeftName
    left_nation: String,       // L3 LeftNat
    left_score: String,        // L4 Lscore
    left_status: String,       // L5 Lstatus
    left_yellow_cards: String, // L6 LYcard
    left_red_cards: String,    // L7 LRcard
    left_light: String,        // L8 LLight
    left_white_light: String,  // L9 LWlight
    left_medical: String,      // L10 RMedical (left fencer)
    left_reserve: String,      // L11 LReserve
    left_p_card: String,       // L12 LP-card
}

fn decompose_msg(mut raw: &str) -> Result<Message, String> {

    const GENERAL_LEN : usize = 17;
    const FENCER_FIELD_LENGTH: usize = 12;


    raw = &raw[1..]; //remove head 1 char
    raw = &raw[..raw.len()- 3]; //remove tail 3 char

    let mut sections: Vec<&str> = raw
        .split('%')
        .collect();

    println!("Section general: {}", sections[0].to_string());
    println!("Section Rfencer: {}", sections[1].to_string());
    println!("Section Lfencer: {}", sections[2].to_string());

    let mut general: Vec<&str> = sections[0]
        .trim_matches('|')
        .split('|')
        .map(|value| if value.is_empty() { "0" } else { value })
        .collect();

    println!("General data - {:?}", general);
    println!("General data length - {}", general.len());

    sections[1] = &sections[1][1..sections[1].len()-1];

    let mut rfencerdata: Vec<&str> = sections[1]
        .split('|')
        .map(|value| if value.is_empty() { "0" } else { value })
        .collect();


    println!("Right fencer data - {:?}", rfencerdata);
    println!("Right fencer data length - {}", rfencerdata.len());

    sections[2] = &sections[2][1..sections[2].len()-1];

    let mut lfencerdata: Vec<&str> = sections[2]
        .split('|')
        .map(|value| if value.is_empty() { "0" } else { value })
        .collect();

    println!("Left fencer data - {:?}", lfencerdata);
    println!("Left fencer data length - {}", lfencerdata.len());


    if general.len() != GENERAL_LEN {
        let missing = GENERAL_LEN - general.len();

        general.resize(general.len() + missing, "0");
    }
        println!("asd{:?}",general);
    if rfencerdata.len() != FENCER_FIELD_LENGTH {
        let missing = FENCER_FIELD_LENGTH - rfencerdata.len();

        rfencerdata.resize(rfencerdata.len() + missing, "0");
    }
        println!("dsa{:?}",rfencerdata);
    if lfencerdata.len() != FENCER_FIELD_LENGTH {
        let missing = FENCER_FIELD_LENGTH - lfencerdata.len();

        lfencerdata.resize(lfencerdata.len() + missing, "0");
    }
        println!("sda{:?}",lfencerdata);

    let mut sp_vec = Vec::new();

    sp_vec.extend(general);
    sp_vec.extend(rfencerdata);
    sp_vec.extend(lfencerdata);

    println!("{:?}",sp_vec);

    const EXPECTED_FIELDS: usize = 41;
    if sp_vec.len() < EXPECTED_FIELDS {
        return Err(format!(
            "not enough fields: got {}, expected {}",
            sp_vec.len(),
            EXPECTED_FIELDS
        ));
    }

    let msg: Message = Message {
        protocol: sp_vec[0].to_string(),
        command: sp_vec[1].to_string(),
        piste: sp_vec[2].to_string(),
        competition: sp_vec[3].to_string(),
        phase: sp_vec[4].to_string(),
        pool_tab: sp_vec[5].to_string(),
        match_number: sp_vec[6].to_string(),
        round: sp_vec[7].to_string(),
        time: sp_vec[8].to_string(),
        stopwatch: sp_vec[9].to_string(),
        competition_type: sp_vec[10].to_string(),
        weapon: sp_vec[11].to_string(),
        priority: sp_vec[12].to_string(),
        state: sp_vec[13].to_string(),
        referee_id: sp_vec[14].to_string(),
        referee_name: sp_vec[15].to_string(),
        referee_nation: sp_vec[16].to_string(),
        right_id: sp_vec[17].to_string(),
        right_name: sp_vec[18].to_string(),
        right_nation: sp_vec[19].to_string(),
        right_score: sp_vec[20].to_string(),
        right_status: sp_vec[21].to_string(),
        right_yellow_cards: sp_vec[22].to_string(),
        right_red_cards: sp_vec[23].to_string(),
        right_light: sp_vec[24].to_string(),
        right_white_light: sp_vec[25].to_string(),
        right_medical: sp_vec[26].to_string(),
        right_reserve: sp_vec[27].to_string(),
        right_p_card: sp_vec[28].to_string(),
        left_id: sp_vec[29].to_string(),
        left_name: sp_vec[30].to_string(),
        left_nation: sp_vec[31].to_string(),
        left_score: sp_vec[32].to_string(),
        left_status: sp_vec[33].to_string(),
        left_yellow_cards: sp_vec[34].to_string(),
        left_red_cards: sp_vec[35].to_string(),
        left_light: sp_vec[36].to_string(),
        left_white_light: sp_vec[37].to_string(),
        left_medical: sp_vec[38].to_string(),
        left_reserve: sp_vec[39].to_string(),
        left_p_card: sp_vec[40].to_string(),
    };

    Ok(msg)
}

fn send_hello(
    ip_address: String,
    port: String,
    piste_number: String,
    competition_id: String,
) -> io::Result<()> {
//|EFP1.1|HELLO|17|fm-eq|%| - example HELLO message
{
let addr = "0.0.0.0:0";
let socket = UdpSocket::bind(addr)?;


let msg: String = format!("|EFP1.1|HELLO|{}|{}|%|", piste_number, competition_id);
let buf = msg.as_bytes();

println!("{:?}", buf);


let recv_addr: String = format!("{}:{}", ip_address, port);

socket
    .send_to(buf, &recv_addr)
    .map_err(|e| io::Error::new(e.kind(), format!("send_to({recv_addr}) failed: {e}")))?;

}
Ok(())
}

fn recv_msg(){
    
}

fn main() /*-> std::io::Result<()>*/{
    /*{
         let mut addr = "127.0.0.1:50100";
        let socket = UdpSocket::bind(addr)?;
        let mut buf = [0;210];
        let (amt, src) = socket.recv_from(&mut buf)?;

        println!("Number of received bytes: {}", amt);

        let buf = &mut buf[..amt];

        println!("Raw bytes: {:?}", buf);

        let msg = String::from_utf8_lossy(buf);

        println!("as text: {}", msg);

        println!("number of characters: {}", msg.len());

        let my_obj = decompose_msg(&msg);

        println!("{:?}", my_obj);

        buf.reverse();

        addr = "127.0.0.1:50101";

        socket.send_to(buf, addr)?;

    }
    Ok(())*/

    match send_hello("192.168.1.103".to_string(), "50100".to_string(), "1".to_string(), "fj-emq".to_string()) {
        Ok(v) => {println!("{:?}", v)}
        Err(e) => eprintln!("failed: {e}"),
    }
}
