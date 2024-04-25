use std::rc::Rc;

use super::L2rUser;
use super::{ConstructParams, PeerConstructor, Specifier};
use super::spec;

#[derive(Debug, Clone)]
pub struct PeerCtl (pub String);
impl Specifier for PeerCtl {
    fn construct(&self, cp: ConstructParams) -> PeerConstructor {
        let env = match &cp.left_to_right {
            L2rUser::ReadFrom(ref env) => &**env,
            _ => panic!("PeerCtl: unexpected L2rUser"),
        };

        let rargs = match &env.uri {
            Some(uri) => uri.trim_start_matches('/'),
            None => panic!("PeerCtl: no URI in env"),
        };

        let rspec = match self.0.len() {
            0 => rargs.to_string(),
            _ => format!("{}:{}", self.0, rargs),
        };

        spec(&rspec).unwrap().construct(cp.clone())
    }
    specifier_boilerplate!(noglobalstate singleconnect no_subspec );
}
specifier_class!(
    name = PeerCtlClass,
    target = PeerCtl,
    prefixes = ["peer-ctl:", "from-peer:", "from-left:"],
    arg_handling = into,
    overlay = false,
    StreamOriented,
    SingleConnect,
    help = r#"
Connect to specified left-peer-specified peer. Argument is a overridable specifier

Example: connect to tcp host specified by the left peer's URI

    websocat ws-l:0.0.0.0:8081 from-left:tcp 

Example: dangerously allow left peer to specify peer protocol

    websocat ws-l:0.0.0.0:8081 from-left:
"#
);