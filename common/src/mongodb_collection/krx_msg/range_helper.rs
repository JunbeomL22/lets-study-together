use std::ops::Range;

/// If the payload starts with
/// A3, G7, B6 => Some(Range{start: 17, end: 29}) [quote, quote+trade, trade]
/// B7 => Some(Range{start: 9, end: 21})  [quote with MM/LP together]
/// OA => Some(Range{start: 15, end: 27}) [remaining orders]
/// A6 => Some(Range{start: 15, end: 27})  [market close]
/// C4 => Some(Range{start: 7, end: 19})  [market open]
/// H2 => Some(Range{start: 5, end: 17})  [open interest]
/// H1 => Some(Range{start: 21, end: 33})  [derivative investors]
/// H6 => Some(Range{start: 24, end: 36})  [underlying bond info of KTBF]
/// A0 => Some(Range{start: 27, end: 39})  [inst info excluding ELW/ETN]
/// J9077 => Some(Range{start: 13, end: 25})  [bond issue info]
pub fn krx_messages_instcode_range(payload: &[u8]) -> Option<Range<usize>> {
    if payload.len() > 5 {
        if &payload[..5] == b"B6054" || &payload[..5] == b"B6044" {
            return None;
        } else if &payload[..5] == b"J9077" {
            // [bond issue info]
            return Some(13..25);
        }
    }
    match payload.get(0..2) {
        // [quote & trade]
        Some(b"A3") | Some(b"G7") | Some(b"B6") => Some(17..29),
        // [quote with MM/LP together]
        Some(b"B7") => Some(9..21),
        // [remaining orders]
        Some(b"OA") => Some(15..27),
        // [market close]
        Some(b"A6") => Some(15..27),
        // [market open]
        Some(b"C4") => Some(7..19),
        // [open interest]
        Some(b"H2") => Some(5..17),
        // [derivative investors]
        Some(b"H1") => Some(21..33),
        // [inst info excluding ELW/ETN]
        Some(b"A0") => Some(27..39),
        // [ELW/ETN info]
        Some(b"A1") => Some(13..25),
        // [underlying bond info of KTBF]
        Some(b"H6") => Some(24..36),
        _ => None,
    }
}

/// If the payload starts with
/// A3 | G7 | B6 => qote and trade
/// H2 => open interest
/// C1 => investor statistics after market close
/// A0 => inst info excluding ELW/ETN
/// H6 => unserlying bond info of KTBF
/// B7 => quote with MM/LP together
pub fn krx_message_dist_index_range(payload: &[u8]) -> Option<Range<usize>> {    
    if payload.len() < 5 {
        return None;
    }
    let trcode: &[u8; 5] = payload[..5].try_into().unwrap();
    if is_a0(trcode) || is_b6(trcode) || is_a3(trcode) || is_g7(trcode) {
        return Some(5..13);
    }
    match payload.get(0..2) {
        //Some(b"A3") | Some(b"G7") | Some(b"B6") => Some(5..13),
        Some(b"H2") | Some(b"C1") => Some(17..23),
        //Some(b"A0") => Some(5..13),
        Some(b"H6") => Some(5..13),
        Some(b"B7") => Some(5..13),
        _ => None,
    }
}

/// (증권A) STK : A001S
/// (증권C) STK : A002S, A003S, A004S
/// (증권B) KSQ : A001Q
/// (증권B) KNX : A001X
/// (채권A) BND : A001B
/// (채권A) RPO : A001R
/// (파생A) DRV : A001F, A002F, A003F, A004F, A005F, A006F, A007F, A008F, A009F, A010F, A011F, A012F, A013F, A015F, A016F
/// (파생B) DRV : A014F
/// (일반A) CMD : A001G
/// (일반A) ETS : A001E
fn is_a0(trcode: &[u8; 5]) -> bool {
    let res = matches!(
        trcode, 
        b"A001S" | b"A002S" | b"A003S" | b"A004S" | b"A001Q" | b"A001X" | b"A001B" | 
        b"A001R" | b"A001F" | b"A002F" | b"A003F" | b"A004F" | b"A005F" | b"A006F" | 
        b"A007F" | b"A008F" | b"A009F" | b"A010F" | b"A011F" | b"A012F" | b"A013F" | 
        b"A015F" | b"A016F" | b"A014F" | b"A001G" | b"A001E",
    );
    res
}

/// (채권A) BND : B601B
/// (채권A) KTS : B601K
/// (채권A) SMB : B601M
/// (채권A) RPO : B601R
/// (파생A) DRV : B601F, B602F, B603F, B606F, B607F, B608F, B609F, B610F, B611F, B612F, B613F, B615F, B616F
/// (파생B) DRV : B614F
/// (파생A) DRV : B604F, B605F
/// (일반A) CMD : B601G
/// (일반A) ETS : B601E
fn is_b6(trcode: &[u8; 5]) -> bool {
    let res = matches!(
        trcode, 
        b"B601B" | b"B601K" | b"B601M" | b"B601R" | b"B601F" | b"B602F" | 
        b"B603F" | b"B604F" | b"B605F" | b"B606F" | b"B607F" | b"B608F" | 
        b"B609F" | b"B610F" | b"B611F" | b"B612F" | b"B613F" | b"B614F" | 
        b"B615F" | b"B616F" | b"B601G" | b"B601E"
    );
    res
 }
/// (채권A) BND : G701B
/// (채권A) KTS : G701K
/// (채권A) SMB : G701M
/// (채권A) RPO : G701R
/// (파생A) DRV : G701F, G702F, G703F, G706F, G707F, G708F, G709F, G710F, G711F, G712F, G713F, G715F, G716F
/// (파생B) DRV : G714F
/// (파생A) DRV : G704F, G705F
/// (일반A) CMD : G701G
/// (일반A) ETS : G701E
fn is_g7(trcode: &[u8; 5]) -> bool {
    let res = matches!(
        trcode, 
        b"G701B" | b"G701K" | b"G701M" | b"G701R" | b"G701F" | b"G702F" | b"G703F" | 
        b"G704F" | b"G705F" | b"G706F" | b"G707F" | b"G708F" | b"G709F" | b"G710F" | 
        b"G711F" | b"G712F" | b"G713F" | b"G714F" | b"G715F" | b"G716F" | b"G701G" | b"G701E"
    );
    res
 }
 
/// (증권A) STK : A301S
/// (증권C) STK : A302S, A303S, A304S
/// (증권B) KSQ : A301Q
/// (증권B) KNX : A301X
/// (채권A) BND : A301B
/// (채권A) SMB : A301M
/// (채권A) KTS : A301K
/// (파생A) DRV : A301F, A302F, A303F, A304F, A305F, A306F, A307F, A308F, A309F, A310F, A311F, A312F, A313F, A315F, A316F
/// (파생B) DRV : A314F
/// (일반A) CMD : A301G
/// (일반A) ETS : A301E
fn is_a3(trcode: &[u8; 5]) -> bool {
    let res = matches!(trcode, 
        b"A301S" | b"A302S" | b"A303S" | b"A304S" | b"A301Q" | 
        b"A301X" | b"A301B" | b"A301M" | b"A301K" | b"A301F" | 
        b"A302F" | b"A303F" | b"A304F" | b"A305F" | b"A306F" | 
        b"A307F" | b"A308F" | b"A309F" | b"A310F" | b"A311F" | 
        b"A312F" | b"A313F" | b"A314F" | b"A315F" | b"A316F" | 
        b"A301G" | b"A301E"
    );
    res
 }