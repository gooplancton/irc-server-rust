#[derive(FromIRCString)]
pub enum Command {
    #[commmand_name="JOIN"]
    Join(JoinArgs),

    #[command_name="CAP"]
    Capabilities(CapabilitiesArgs),

    #[command_name="NICK"]
    Nickname(NicknameArgs)
}
