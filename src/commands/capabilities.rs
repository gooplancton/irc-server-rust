use irc_parser::FromIRCString;

#[derive(FromIRCString)]
pub struct CapabilitiesArgs;

pub fn handle_capabilities(_args: CapabilitiesArgs) -> anyhow::Result<Vec<&'static str>> {
    Ok(vec!["CAP * LS :\r\n"])
}
