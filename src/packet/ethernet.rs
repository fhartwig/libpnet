// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Ethernet packet abstraction

use packet::{Packet, PrimitiveValues};

use util::MacAddr;

/// Represents an Ethernet packet
#[packet]
pub struct Ethernet {
    #[construct_with(u8, u8, u8, u8, u8, u8)]
    destination: MacAddr,
    #[construct_with(u8, u8, u8, u8, u8, u8)]
    source: MacAddr,
    #[construct_with(u16)]
    ethertype: EtherType,
    #[payload]
    payload: Vec<u8>,
}

#[test]
fn ethernet_header_test() {
    let mut packet = [0u8; 14];
    {
        let mut ethernet_header = MutableEthernetPacket::new(&mut packet[..]).unwrap();

        let source = MacAddr(0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc);
        ethernet_header.set_source(source);
        assert_eq!(ethernet_header.get_source(), source);

        let dest = MacAddr(0xde, 0xf0, 0x12, 0x34, 0x45, 0x67);
        ethernet_header.set_destination(dest);
        assert_eq!(ethernet_header.get_destination(), dest);

        ethernet_header.set_ethertype(EtherTypes::Ipv6);
        assert_eq!(ethernet_header.get_ethertype(), EtherTypes::Ipv6);
    }

    let ref_packet = [0xde, 0xf0, 0x12, 0x34, 0x45, 0x67, /* destination */
                      0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, /* source */
                      0x86, 0xdd /* ethertype */];
    assert_eq!(&ref_packet[..], &packet[..]);
}

/// EtherTypes defined at:
/// http://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml
/// These values should be used in the Ethernet EtherType field
///
/// FIXME Should include all
/// A handful of these have been selected since most are archaic and unused.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod EtherTypes {
    use packet::ethernet::EtherType;

    /// Internet Protocol version 4 (IPv4) [RFC7042]
    pub const Ipv4: EtherType = EtherType(0x0800);
    /// Address Resolution Protocol (ARP) [RFC7042]
    pub const Arp: EtherType = EtherType(0x0806);
    /// Wake on Lan
    pub const WakeOnLan: EtherType = EtherType(0x0842);
    /// Reverse Address Resolution Protocol (RARP) [RFC903]
    pub const Rarp: EtherType = EtherType(0x8035);
    /// Internet Protocol version 6 (IPv6) [RFC7042]
    pub const Ipv6: EtherType = EtherType(0x86DD);
}

/// Represents the Ethernet ethertype field.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EtherType(pub u16);

impl EtherType {
    /// Construct a new EtherType
    pub fn new(val: u16) -> EtherType {
        EtherType(val)
    }
}

impl PrimitiveValues for EtherType {
    type T = (u16,);
    fn to_primitive_values(&self) -> (u16,) {
        (self.0,)
    }
}
