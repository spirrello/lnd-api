use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let dir = match std::env::var_os("LND_REPO") {
        Some(lnd_protos_path) => {
            let mut lnd_protos_dir = PathBuf::from(lnd_protos_path);
            lnd_protos_dir.push("lnrpc");
            lnd_protos_dir
        }
        None => PathBuf::from("protos"),
    };

    let protos = vec![
        "autopilotrpc/autopilot.proto",
        "chainrpc/chainnotifier.proto",
        "devrpc/dev.proto",
        "invoicesrpc/invoices.proto",
        "lightning.proto",
        "stateservice.proto",
        "walletunlocker.proto",
        "lnclipb/lncli.proto",
        "neutrinorpc/neutrino.proto",
        "peersrpc/peers.proto",
        "routerrpc/router.proto",
        "signrpc/signer.proto",
        "verrpc/verrpc.proto",
        "walletrpc/walletkit.proto",
        "watchtowerrpc/watchtower.proto",
        "wtclientrpc/wtclient.proto",
    ];

    let proto_paths: Vec<_> = protos
        .iter()
        .map(|proto| {
            let mut path = PathBuf::from(&dir);
            path.push(proto);
            path.display().to_string()
        })
        .collect();

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(&proto_paths, &[dir])?;
    Ok(())
}
