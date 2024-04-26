use std::rc::Rc;

use crate::Peer;

use super::L2rUser;
use super::{ConstructParams, PeerConstructor, Specifier};
use super::spec;
use url::Url;

#[derive(Debug, Clone)]
pub struct PeerCtl (pub String);
impl Specifier for PeerCtl {
    fn construct(&self, cp: ConstructParams) -> PeerConstructor {
        // if self.0 doesn't have a colon, add one
        let default_rspec = match (self.0.len(), self.0.find(':')) {
            (0, _) => self.0.clone(),
            (_, Some(_)) => self.0.clone(),
            (_, None) => format!("{}:", self.0),
        };

        debug!("PeerCtl: default_rspec: {:?}", default_rspec);

        let env = match &cp.left_to_right {
            L2rUser::ReadFrom(ref env) => &**env,
            _ => panic!("PeerCtl: unexpected L2rUser"),
        };

        let rargs = match (&cp.program_options.peer_ctl_query_param, &cp.program_options.peer_ctl_header, &env.uri) {
            (Some(query_param), None, _) => {
                debug!("PeerCtl: rargs from query param: {:?}", query_param);

                Url::parse(format!("https://example.com{}", &env.uri.as_ref().unwrap()).as_str())
                    .unwrap()
                    .query_pairs().find(|(k, _)| k == query_param)
                    .unwrap_or_default().1.to_string()
            },
            (None, Some(header_name), _) => {
                debug!("PeerCtl: rargs from header: {:?}", header_name);

                env.headers.iter().find(|(k, _)| k == header_name)
                    .unwrap_or(&(String::new(), String::new())).1.to_string()
            },
            (None, None, Some(uri)) => {
                debug!("PeerCtl: rargs from uri: {:?}", uri);

                uri.trim_start_matches('/').to_string()
            },
            _ => panic!("PeerCtl: no peer_ctl_query_param in program_options"),
        };

        debug!("PeerCtl: rargs: {:?}", rargs);

        let rspec = match default_rspec.len() {
            0 => rargs.to_string(),
            _ => {
                // Split default_rspec and rargs by colon, and then map the rargs over the default_rspec
                // default_rspec = tcp:example.com:80, rargs = example.net:443, result = tcp:example.net:443
                // default_rspec = tcp:example.com, rargs = example.net:443, result = tcp:example.net:443
                // default_rspec = tcp:, rargs = example.net:443, result = tcp:example.net:443
                let default_spec_type = default_rspec.split(':').next().unwrap();
                let mut rspec = default_rspec.split(':').skip(1).collect::<Vec<&str>>();
                let rargs = rargs.split(':').collect::<Vec<&str>>();
                rspec.iter_mut().zip(rargs.iter()).for_each(|(a, b)| *a = *b);
                if rspec.len() < rargs.len() {
                    rspec.extend_from_slice(&rargs[rspec.len()..]);
                }
                format!("{}:{}", default_spec_type, rspec.join(":"))
            },
        };

        debug!("PeerCtl: rspec: {:?}", rspec);

        match spec(&rspec) {
            Ok(spec) => spec.construct(cp),
            Err(e) => PeerConstructor::Error(e),
        }
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