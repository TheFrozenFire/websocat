use futures::future::ok;
use http_bytes::http::header::HeaderName;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use super::{ L2rUser, LeftSpecToRightSpec };
use super::{box_up_err, peer_err_s, wouldblock, BoxedNewPeerFuture, BoxedNewPeerStream, Peer};
use super::{multi, once, ConstructParams, Options, PeerConstructor, Specifier};
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

        let rspec = format!("{}:{}", self.0, rargs);

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

Example: connect to tcp host specified by websocket header X-Target-Host:X-Target-Port

    websocat ws-l:0.0.0.0:8084 from-left:tcp 

Example: dangerously allow left peer to specify peer protocol

    websocat ws-l:0.0.0.0:8084 from-left:ws:
"#
);