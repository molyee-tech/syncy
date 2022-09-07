use libp2p::transport::Boxed;
use libp2p::identify::Keypair;
use libp2p::PeerId;
use libp2p::muxing::StreamMuxerBox;

pub fn build_transport(
    key_pair: Keypair,
    psk: Option<PreSharedKey>,
) -> Boxed<(PeerId, StreamMuxerBox)> {
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&key_pair)
        .unwrap();
    let noise_config = noise::NoiseConfig::xx(noise_keys).into_authenticated();
    let yamux_config = YamuxConfig::default();

    let base_transport = TcpTransport::new(GenTcpConfig::default().nodelay(true));
    let maybe_encrypted = match psk {
        Some(psk) => EitherTransport::Left(
            base_transport.and_then(move |socket, _| PnetConfig::new(psk).handshake(socket)),
        ),
        None => EitherTransport::Right(base_transport),
    };
    maybe_encrypted
        .upgrade(Version::V1)
        .authenticate(noise_config)
        .multiplex(yamux_config)
        .timeout(Duration::from_secs(20))
        .boxed()
    /*
    let (tx, rx) = mpsc::unbounded_channel();
    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&KEYS)
        .expect("can create auth keys");

    let upversion = upgrade::Version::V1;
    let creds = NoiseConfig::xx(auth_keys).into_authenticated();
    let mplex = mplex::MplexConfig::new();

    // todo build transport from config
    let transport = TokioTcpConfig::new()
        .upgrade(upversion)
        .authenticate(creds)
        .multiplex(mplex)
        .boxed();
     */
}