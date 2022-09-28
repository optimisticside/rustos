bitflags::bitflags! {
    /// Specifies the type of hardware used for the local network. Used in the exact same way as
    /// the equivalent field (HRD) in the Address Resolution Protocal (ARP).
    pub struct HardwareType: u8 {
        const ETHERNET = 1;
        const IEEE_802 = 6;
        const ARC_NET = 7;
        const LOCAL_TALK = 11;
        const LOCAL_NET = 12;
        const SDMS = 14;
        const FRAME_DELAY = 15;
        const ASYNC_TRANSFER = 16;
        const HDLC = 17;
        const FIBER_CHANNEL = 18;
        const SERIAL_LINE = 20;
    }
}

bitflags::bitflags! {
    /// Corresponds to the formerly 2-byte field in the BOOTP message format defined by RFC 951.
    /// Presently contains just 1 flag subfield, as follows.
    pub struct PacketFlags: u16 {
        /// A client that does not know its IP address at the time it sends the request sets this
        /// flag, to serve as an immediate indicator that the DCHP server or relay agent that
        /// recieves the request should send its reply back by broadcast.
        const BROADCAST = 1 << 0;
    }
}

/// A packet is a unit of data that can be sent or recieved as part of a larger piece of data,
/// through the network.
#[derive(C, packed)]
pub struct Packet {
    /// Specifies a general type of message. A value of 1 indicates this is a request message and a
    /// value of 2 indicates that it is a response/reply message.
    oper_code: u8,
    /// See [`HardwareType`]
    hw_type: u8,
    /// How long hardware addresses are in this range.
    /// For ethernet or other networks using IEEE 802 MAC addresses, the value is 6.
    hw_addr_len: u8,
    /// Set to 0 by a client before transmitting a request and used by relay agents to control the
    /// forwarding of BOOTP and/or DCHP messages.
    hops: u8,
    /// See [`PacketFlags`]
    flags: u16,
    /// IP Address of the client.
    /// The client sets this field only if it has a valid IP address while in the BOUND, RENEWING,
    /// or REBINDING states. It is otherwise set to 0.
    client_ip: u32,
    /// IP Address the server is assigning to the client.
    your_ip: u32,
    /// IP address of the server that the client should use for the next step in the bootstrap
    /// process, which may not address of the server sending the reply.
    server_ip: u32,
    /// The server sending the DCHPOFFER or DCHPACK message might put its name in this field. It
    /// can be a nickname or a full DNS domain name.
    server_name: [u8; 8],
    /// Can be used by the client to request a particular type of boot file in a DCHPDISCOVER
    /// message. Used by the server in a DCHPOFFER to fully specify a boot file directory-path and
    /// filename.
    boot_filename: [u8; 16],
}
