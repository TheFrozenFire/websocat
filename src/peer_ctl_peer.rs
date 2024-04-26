use std::rc::Rc;

use super::L2rUser;
use super::{ConstructParams, PeerConstructor, Specifier};
use super::spec;
use url::Url;

#[derive(Debug, Clone)]
pub struct PeerCtl (pub String);
impl Specifier for PeerCtl {
    fn construct(&self, cp: ConstructParams) -> PeerConstructor {
        let env = match &cp.left_to_right {
            L2rUser::ReadFrom(ref env) => &**env,
            _ => panic!("PeerCtl: unexpected L2rUser"),
        };

        println!("{}", env.headers.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join("\n"));

        let rargs = match (&cp.program_options.peer_ctl_query_param, &cp.program_options.peer_ctl_header, &env.uri) {
            (Some(query_param), None, _) => {
                Url::parse(format!("https://example.com{}", &env.uri.as_ref().unwrap()).as_str())
                    .unwrap()
                    .query_pairs().find(|(k, _)| k == query_param)
                    .unwrap_or_default().1.to_string()
            },
            (None, Some(header_name), _) => {
                env.headers.iter().find(|(k, _)| k == header_name)
                    .unwrap_or(&(String::new(), String::new())).1.to_string()
            },
            (None, None, Some(uri)) => uri.trim_start_matches('/').to_string(),
            _ => panic!("PeerCtl: no peer_ctl_query_param in program_options"),
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