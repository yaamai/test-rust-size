use netlink_sys::{protocols::NETLINK_GENERIC, AsyncSocket, AsyncSocketExt, SmolSocket, SocketAddr};
use netlink_packet_core::{NetlinkPayload, NetlinkPayload::InnerMessage, NetlinkMessage, NetlinkHeader, NLM_F_DUMP, NLM_F_REQUEST};
use netlink_packet_generic::{GenlMessage, ctrl::{nlas::GenlCtrlAttrs, GenlCtrl, GenlCtrlCmd}};
use netlink_packet_wireguard::{
    nlas::{WgAllowedIpAttrs, WgDeviceAttrs, WgPeerAttrs},
    Wireguard,
    WireguardCmd,
};

use smol::{io};

async fn get_family_id(family_name: String) {
    let mut msg = NetlinkMessage {
        header: NetlinkHeader {
            flags: NLM_F_REQUEST | NLM_F_DUMP,
            ..Default::default()
        },
        payload: GenlMessage::from_payload(GenlCtrl {
            cmd: GenlCtrlCmd::GetFamily,
            nlas: vec![GenlCtrlAttrs::FamilyName(family_name.to_owned())],
        })
        .into(),
    };

    msg.finalize();
    let mut buf = vec![0; msg.buffer_len() as usize];
    msg.serialize(&mut buf[..msg.buffer_len()]);

    let mut sock = SmolSocket::new(NETLINK_GENERIC).unwrap();
    let kernel_addr = SocketAddr::new(0, 0);

    let result = sock.send_to(&buf, &kernel_addr).await;
    println!("{:?}", result);

    let (buf2, _) = sock.recv_from_full().await.unwrap();
    println!("{:?}, {}", buf2, buf2.len());
    let msg2: NetlinkMessage<GenlMessage<GenlCtrl>> = NetlinkMessage::deserialize(&buf2).unwrap();
    println!("{:?}", msg2.buffer_len());
    match msg2.payload {
        InnerMessage(innmsg) => {
            println!("{:?}", innmsg.payload.nlas);
        },
        _ => (),
    }
}

async fn a() {
    let genlmsg: GenlMessage<Wireguard> = GenlMessage::from_payload(Wireguard {
        cmd: WireguardCmd::GetDevice,
        nlas: vec![WgDeviceAttrs::IfName("wg0".into())],
    });
    let mut nlmsg = NetlinkMessage::from(genlmsg);
    nlmsg.header.flags = NLM_F_REQUEST | NLM_F_DUMP;

    // let mut nlmsg = NetlinkMessage {
    //     header: NetlinkHeader {
    //         message_type: 2,
    //         flags: NLM_F_REQUEST | NLM_F_DUMP,
    //         ..Default::default()
    //     },
    //     payload: GenlMessage::from_payload(Wireguard {
    //         cmd: WireguardCmd::GetDevice,
    //         nlas: vec![WgDeviceAttrs::IfName("wg0".into())],
    //     })
    //     .into(),
    // };

    nlmsg.finalize();
    let mut buf = vec![0; nlmsg.buffer_len() as usize];
    nlmsg.serialize(&mut buf[..nlmsg.buffer_len()]);
    println!("{:?}", buf);


    // sendto(3, [{nlmsg_len=28, nlmsg_type=0 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|0x300, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x01\x00\x00\x08\x00\x02\x00\x77\x67\x30\x00"], 28, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 28
    // sendto(3, [{nlmsg_len=28, nlmsg_type=wireguard, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x300, nlmsg_seq=1648836879, nlmsg_pid=0}, "\x00\x01\x00\x00\x08\x00\x02\x00\x77\x67\x30\x00"], 28, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 28


    let mut sock = SmolSocket::new(NETLINK_GENERIC).unwrap();
    let kernel_addr = SocketAddr::new(0, 0);

    let result = sock.send_to(&buf, &kernel_addr).await;
    println!("{:?}", result);

    let mut buf2 = vec![0; 4096];
    sock.recv_from(&mut buf2).await.unwrap();
    /*
    let (conn, mut handle, _) = new_connection_with_socket::<SmolSocket>().unwrap();
    let mut responses = handle.request(nlmsg).await.unwrap();
        println!("a");
    while let Some(result) = responses.next().await {
        println!("a");
        let rx_packet = result.unwrap();
        match rx_packet.payload {
            NetlinkPayload::InnerMessage(genlmsg) => {
                print_wg_payload(genlmsg.payload);
            }
            NetlinkPayload::Error(e) => {
                eprintln!("Error: {:?}", e.to_io());
            }
            _ => (),
        };
    }
    */
}

fn main() -> io::Result<()> {
    smol::block_on(async {
        get_family_id("wireguard".into()).await;
        a().await;
        // let mut stream = net::TcpStream::connect("example.com:80").await?;
        // let req = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
        // stream.write_all(req).await?;

        // let mut stdout = Unblock::new(std::io::stdout());
        // io::copy(stream, &mut stdout).await?;
        Ok(())
    })
}

fn print_wg_payload(wg: Wireguard) {
    for nla in &wg.nlas {
        match nla {
            WgDeviceAttrs::IfIndex(v) => println!("IfIndex: {}", v),
            WgDeviceAttrs::IfName(v) => println!("IfName: {}", v),
            WgDeviceAttrs::PrivateKey(_) => println!("PrivateKey: (hidden)"),
            WgDeviceAttrs::PublicKey(v) => println!("PublicKey: {}", base64::encode(v)),
            WgDeviceAttrs::ListenPort(v) => println!("ListenPort: {}", v),
            WgDeviceAttrs::Fwmark(v) => println!("Fwmark: {}", v),
            WgDeviceAttrs::Peers(nlas) => {
                for peer in nlas {
                    println!("Peer: ");
                    print_wg_peer(peer);
                }
            }
            _ => (),
        }
    }
}

fn print_wg_peer(nlas: &[WgPeerAttrs]) {
    for nla in nlas {
        match nla {
            WgPeerAttrs::PublicKey(v) => println!("  PublicKey: {}", base64::encode(v)),
            WgPeerAttrs::PresharedKey(_) => println!("  PresharedKey: (hidden)"),
            WgPeerAttrs::Endpoint(v) => println!("  Endpoint: {}", v),
            WgPeerAttrs::PersistentKeepalive(v) => println!("  PersistentKeepalive: {}", v),
            WgPeerAttrs::LastHandshake(v) => println!("  LastHandshake: {:?}", v),
            WgPeerAttrs::RxBytes(v) => println!("  RxBytes: {}", v),
            WgPeerAttrs::TxBytes(v) => println!("  TxBytes: {}", v),
            WgPeerAttrs::AllowedIps(nlas) => {
                for ip in nlas {
                    print_wg_allowedip(ip);
                }
            }
            _ => (),
        }
    }
}

fn print_wg_allowedip(nlas: &[WgAllowedIpAttrs]) -> Option<()> {
    let ipaddr = nlas.iter().find_map(|nla| {
        if let WgAllowedIpAttrs::IpAddr(addr) = nla {
            Some(*addr)
        } else {
            None
        }
    })?;
    let cidr = nlas.iter().find_map(|nla| {
        if let WgAllowedIpAttrs::Cidr(cidr) = nla {
            Some(*cidr)
        } else {
            None
        }
    })?;
    println!("  AllowedIp: {}/{}", ipaddr, cidr);
    Some(())
}
