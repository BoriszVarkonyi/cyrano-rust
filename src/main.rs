//Modules import
mod message;

use std::thread::JoinHandle;
use std::thread::spawn;
use std::{io, net::UdpSocket};

//Saját importok
use message::Message;

fn decompose_msg(mut raw: &str) -> Result<Message, String> {
    const GENERAL_LEN: usize = 17;
    const FENCER_FIELD_LENGTH: usize = 12;

    raw = &raw[1..]; //remove head 1 char
    raw = &raw[..raw.len() - 3]; //remove tail 3 char

    let mut sections: Vec<&str> = raw.split('%').collect();

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

    sections[1] = &sections[1][1..sections[1].len() - 1];

    let mut rfencerdata: Vec<&str> = sections[1]
        .split('|')
        .map(|value| if value.is_empty() { "0" } else { value })
        .collect();

    println!("Right fencer data - {:?}", rfencerdata);
    println!("Right fencer data length - {}", rfencerdata.len());

    sections[2] = &sections[2][1..sections[2].len() - 1];

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
    println!("asd{:?}", general);
    if rfencerdata.len() != FENCER_FIELD_LENGTH {
        let missing = FENCER_FIELD_LENGTH - rfencerdata.len();

        rfencerdata.resize(rfencerdata.len() + missing, "0");
    }
    println!("dsa{:?}", rfencerdata);
    if lfencerdata.len() != FENCER_FIELD_LENGTH {
        let missing = FENCER_FIELD_LENGTH - lfencerdata.len();

        lfencerdata.resize(lfencerdata.len() + missing, "0");
    }
    println!("sda{:?}", lfencerdata);

    let mut sp_vec = Vec::new();

    sp_vec.extend(general);
    sp_vec.extend(rfencerdata);
    sp_vec.extend(lfencerdata);

    println!("{:?}", sp_vec);

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

fn compose_hello(piste_number: String, competition_id: String) -> String {
    //|EFP1.1|HELLO|17|fm-eq|%| - example HELLO message

    let msg: String = format!("|EFP1.1|HELLO|{}|{}|%|", piste_number, competition_id);

    msg
}

fn compose_display(msg: Message) -> String {
    //example display - |EFP1.1|DISP|8|fm-eq|2|B64| … |%|28|P. Martin|FRA||U|…|%|32| B. Panini|ITA||U|…|%|

    let general = [
        msg.protocol.as_str(),
        msg.command.as_str(), // use command from Message
        msg.piste.as_str(),
        msg.competition.as_str(),
        msg.phase.as_str(),
        msg.pool_tab.as_str(),
        msg.match_number.as_str(),
        msg.round.as_str(),
        msg.time.as_str(),
        msg.stopwatch.as_str(),
        msg.competition_type.as_str(),
        msg.weapon.as_str(),
        msg.priority.as_str(),
        msg.state.as_str(),
        msg.referee_id.as_str(),
        msg.referee_name.as_str(),
        msg.referee_nation.as_str(),
    ];

    let right = [
        msg.right_id.as_str(),
        msg.right_name.as_str(),
        msg.right_nation.as_str(),
        msg.right_score.as_str(),
        msg.right_status.as_str(),
        msg.right_yellow_cards.as_str(),
        msg.right_red_cards.as_str(),
        msg.right_light.as_str(),
        msg.right_white_light.as_str(),
        msg.right_medical.as_str(),
        msg.right_reserve.as_str(),
        msg.right_p_card.as_str(),
    ];

    let left = [
        msg.left_id.as_str(),
        msg.left_name.as_str(),
        msg.left_nation.as_str(),
        msg.left_score.as_str(),
        msg.left_status.as_str(),
        msg.left_yellow_cards.as_str(),
        msg.left_red_cards.as_str(),
        msg.left_light.as_str(),
        msg.left_white_light.as_str(),
        msg.left_medical.as_str(),
        msg.left_reserve.as_str(),
        msg.left_p_card.as_str(),
    ];

    format!(
        "|{}|%|{}|%|{}|%|",
        general.join("|"),
        right.join("|"),
        left.join("|")
    )
}

fn send_message(
    socket: &UdpSocket,
    ip_address: String,
    port: String,
    buffer: String,
) -> io::Result<()> {
    {
        let recv_addr: String = format!("{}:{}", ip_address, port);

        let buf = buffer.as_bytes();

        socket
            .send_to(buf, &recv_addr)
            .map_err(|e| io::Error::new(e.kind(), format!("send_to({recv_addr}) failed: {e}")))?;
    }
    Ok(())
}

fn start_listener(socket: UdpSocket) -> JoinHandle<()> {
    spawn(move || {
        let mut buf = [0u8; 2048];

        loop {
            match socket.recv_from(&mut buf) {
                Ok((size, src)) => {
                    let msg = String::from_utf8_lossy(&buf[..size]);
                    println!("[RECV from {}] {}", src, msg);
                }
                Err(e) => {
                    eprintln!("recv_from error: {}", e);
                    break;
                }
            }
        }
    })
}

fn main() -> io::Result<()> {
    {
        //UDP socket creation
        let socket = UdpSocket::bind("0.0.0.0:50200")?;

        let listener_socket = socket.try_clone()?;
        let _listener = start_listener(listener_socket);

        //Example message initiation
        let msg = Message {
            left_id: "2".to_string(),
            left_name: "Valaki".to_string(),
            right_id: "1".to_string(),
            right_name: "Masik".to_string(),
            ..Message::new("DISP", "1", "fjm-eq")
        };

        let to_send = compose_display(msg);

        let example_hello = compose_hello("1".to_string(), "fjm-eq".to_string());

        match send_message(
            &socket,
            "192.168.1.103".to_string(),
            "50100".to_string(),
            example_hello,
        ) {
            Ok(v) => {
                println!("{:?}", v)
            }
            Err(e) => eprintln!("failed: {e}"),
        }
        //println!("{}", to_send);
        //let _ = send_message("192.168.1.103".to_string(), "50100".to_string(), to_send);
    }

    //Keep main alive so the listener thread is alive.
    loop {
        std::thread::park();
    }
    Ok(())
}
