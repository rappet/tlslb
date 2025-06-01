use std::{
    io::{Read, Write},
    marker::PhantomData,
    net::IpAddr,
};

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
///  let ip_ref = db.lookup_ip(IpAddr::V4(Ipv4Addr::new(9, 9, 9, 9))).expect("entry exists in example data");
///  assert_eq!(ip_ref.asn(), 19281);
/// ```
pub struct IpDatabase {
    // TBD....
}

impl IpDatabase {
    /// Creates a new empty database.
    ///
    /// You might want to do bulk load operations like [`Self::load_routing_table_txt`] after creation.
    ///
    /// # TODO
    /// Maybe we want to bind this to the AS database?
    pub fn new() -> Self {
        IpDatabase {}
    }

    /// Lookup metadata of an IP address
    ///
    /// # Example
    ///
    /// ```
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// # use ip_database::IpDatabase;
    /// # let mut db = IpDatabase::new();
    /// # db.load_routing_table_txt(&include_bytes!("./test-route-table.txt")[..]).expect("test data works");
    /// let ip_ref = db.lookup_ip(IpAddr::V4(Ipv4Addr::new(9, 9, 9, 9))).expect("entry exists in example data");
    /// assert_eq!(ip_ref.asn(), 19281);
    /// ```
    pub fn lookup_ip(&self, addr: IpAddr) -> Option<IpRef> {
        todo!()
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
    pub fn load_routing_table_txt(
        &mut self,
        mut reader: impl Read,
    ) -> Result<Self, LoadIpDatabaseError> {
        todo!()
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
