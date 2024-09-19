use bitfield::bitfield;

bitfield! {
    pub struct UserModes(u8);
    pub away, set_away: 1;
    pub invisible, set_invisible: 1;
    pub wallops, set_wallops: 1;
    pub restricted, set_restricted: 1;
    pub operator, set_operator: 1;
    pub local_operator, set_local_operator: 1;
    pub notice_recipient, set_notice_recipient: 1;
}


pub struct ConnectionState {
    capabilities_negotiation_ended: bool,
    nickname: Option<String>,
    modes: UserModes,
}
