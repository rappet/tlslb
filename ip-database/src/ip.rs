use std::{
    io::{BufRead, BufReader, Read, Write},
    marker::PhantomData,
    net::IpAddr,
};

use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use prefix_trie::PrefixMap;

use crate::LoadIpDatabaseError;

/// An IP to ASN route lookup table, that is optimized for read access.
///
/// Note that it is not meant to be modified instead of initial loading steps,
/// as the internal structure might be optimized and routes might be deaggregated to save space
/// and increase performance.
///
/// If you need something that is updated live, you are wrong here.
///
/// To update the database recreate and exchange it.
///
/// # Example
///
/// ```
/// # use std::net::{IpAddr, Ipv4Addr};
/// use std::sync::Arc;
/// # use ip_database::IpDatabase;
/// #
/// # let mut reader = &include_bytes!("./test-route-table.txt")[..];
///  let mut db = IpDatabase::new();
///  db.load_routing_table_txt(reader).expect("test data works");
///
///  // Normally, you would keep this in some shared state
///  let db = Arc::new(db);
///
///  let ip_ref = db.lookup_ip(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 23))).expect("entry exists in example data");
///  assert_eq!(ip_ref.asn(), 64497);
/// ```
pub struct IpDatabase {
    ipv4_to_asn: PrefixMap<Ipv4Net, u32>,
    ipv6_to_asn: PrefixMap<Ipv6Net, u32>,
}

impl IpDatabase {
    /// Creates a new empty database.
    ///
    /// You might want to do bulk load operations like [`Self::load_routing_table_txt`] after creation.
    ///
    /// # TODO
    /// Maybe we want to bind this to the AS database?
    pub fn new() -> Self {
        IpDatabase {
            ipv4_to_asn: PrefixMap::new(),
            ipv6_to_asn: PrefixMap::new(),
        }
    }

    /// Lookup metadata of an IP address
    ///
    /// This function supports IPv4-compatible and IPv4-mapped IP addresses and interprets
    /// them as IPv4 addresses.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// # use ip_database::IpDatabase;
    /// # let mut db = IpDatabase::new();
    /// # db.load_routing_table_txt(&include_bytes!("./test-route-table.txt")[..]).expect("test data works");
    /// let ip_ref = db.lookup_ip(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 23))).expect("entry exists in example data");
    /// assert_eq!(ip_ref.asn(), 64497);
    /// ```
    pub fn lookup_ip(&self, addr: IpAddr) -> Option<IpRef> {
        // we throw away the IP-network here as this will be aggregated and might not correspond
        // to an actual BGP-route in the DFZ anymore
        let asn = match normalize_ip_address(addr) {
            IpAddr::V4(ipv4) => self
                .ipv4_to_asn
                .get_lpm(&Ipv4Net::from(ipv4))
                .map(|(_net, &addr)| addr),
            IpAddr::V6(ipv6) => self
                .ipv6_to_asn
                .get_lpm(&Ipv6Net::from(ipv6))
                .map(|(_net, &addr)| addr),
        };
        asn.map(|asn| IpRef {
            asn: asn,
            _dummy: Default::default(),
        })
    }

    /// Load an IP route to AS mapping from a txt stream
    ///
    /// The data in that stream might look like this:
    ///
    /// ```txt
    /// 1.1.1.0/24 13335
    /// 8.8.8.0/24 15169
    /// 9.9.9.0/24 19281
    /// 2a0e:46c6::/40 207968
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// # use ip_database::IpDatabase;
    /// # let mut reader = &include_bytes!("./test-route-table.txt")[..];
    /// let mut db = IpDatabase::new();
    /// db.load_routing_table_txt(reader).expect("the data from the reader is correct");
    /// ```
    pub fn load_routing_table_txt(&mut self, reader: impl Read) -> Result<(), LoadIpDatabaseError> {
        for line_res in BufReader::new(reader).lines() {
            let line = line_res?;
            if line.trim().is_empty() || line.trim_start().starts_with('#') {
                continue;
            }
            let (route_raw, asn_raw) = line.split_once(' ').ok_or_else(|| {
                LoadIpDatabaseError::MalformedInputFile("line is not split by route and ASN".into())
            })?;
            let net: IpNet = route_raw.parse().map_err(|_err| {
                LoadIpDatabaseError::MalformedInputFile(format!(
                    "ip network {route_raw:?} is malformed in line {line:?}"
                ))
            })?;
            let asn: u32 = asn_raw.parse().map_err(|_err| {
                LoadIpDatabaseError::MalformedInputFile(format!(
                    "asn {asn_raw:?} is malformed in line {line:?}"
                ))
            })?;
            match net {
                IpNet::V4(net_v4) => self.ipv4_to_asn.insert(Ipv4Net::from(net_v4), asn),
                IpNet::V6(net_v6) => self.ipv6_to_asn.insert(Ipv6Net::from(net_v6), asn),
            };
        }

        Ok(())
    }

    /// Store the IP route to ASN mapping to a file
    ///
    /// It uses the same file format as described in [`Self::load_routing_table_txt`]
    pub fn store_routing_table_txt(
        &self,
        mut writer: impl Write,
    ) -> Result<(), LoadIpDatabaseError> {
        todo!()
    }
}

fn normalize_ip_address(ip_addr: IpAddr) -> IpAddr {
    match ip_addr {
        IpAddr::V4(ip_addr) => IpAddr::V4(ip_addr),
        IpAddr::V6(ip_addr) => {
            if let Some(ipv4) = ip_addr.to_ipv4() {
                IpAddr::V4(ipv4)
            } else if let Some(ipv4) = ip_addr.to_ipv4_mapped() {
                IpAddr::V4(ipv4)
            } else {
                IpAddr::V6(ip_addr)
            }
        }
    }
}

/// A reference to a specific IP address in the routing table
pub struct IpRef<'a> {
    asn: u32,
    _dummy: PhantomData<&'a ()>,
}

impl<'a> IpRef<'a> {
    /// The number of the autonomous system
    pub fn asn(&self) -> u32 {
        self.asn
    }
}
