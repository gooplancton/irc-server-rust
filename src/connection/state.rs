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

pub enum RegistrationState {
    Unregistered,
    ReadyToBeRegistered,
    AlreadyRegistered,
    ReadyToBeUnregistered
}

pub struct ConnectionState {
    pub registration_state: RegistrationState,
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub modes: UserModes,
}

impl ConnectionState {
    pub fn new() -> Self {
        Self {
            registration_state: RegistrationState::Unregistered,
            nickname: None,
            username: None,
            modes: UserModes(0),
        }
    }
}
