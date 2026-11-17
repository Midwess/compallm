use std::{net::IpAddr, path::Path};

use maxminddb::{Mmap, Reader, path};

/// One read-only mapping shared by login requests, without a full-file heap copy.
pub struct CityDatabase {
    reader: Reader<Mmap>,
}

impl CityDatabase {
    /// The mapped database must stay immutable. Replace updates by atomic rename,
    /// then restart the process; never truncate or overwrite a mapped file.
    pub fn open(database: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        // SAFETY: deployment owns this read-only data file and must replace it
        // atomically, as required by this constructor's documented contract.
        let reader = unsafe { Reader::open_mmap(database)? };
        if !reader
            .metadata()
            .database_type
            .to_ascii_lowercase()
            .contains("city")
        {
            return Err("a City database is required".into());
        }
        Ok(Self { reader })
    }

    pub fn location(&self, ip: IpAddr) -> Option<String> {
        let ip = match ip {
            IpAddr::V6(ip) => ip
                .to_ipv4_mapped()
                .map(IpAddr::V4)
                .unwrap_or(IpAddr::V6(ip)),
            ip => ip,
        };
        match ip {
            IpAddr::V4(ip)
                if ip.is_private()
                    || ip.is_loopback()
                    || ip.is_link_local()
                    || ip.is_unspecified()
                    || ip.is_multicast()
                    || ip.is_broadcast()
                    || ip.is_documentation()
                    || ip.octets()[0] == 0
                    || ip.octets()[0] >= 240
                    || (ip.octets()[0] == 100 && ip.octets()[1] & 0xc0 == 0x40)
                    || (ip.octets()[0] == 198 && matches!(ip.octets()[1], 18 | 19)) =>
            {
                return None;
            }
            IpAddr::V6(ip)
                if ip.is_loopback()
                    || ip.is_unspecified()
                    || ip.is_multicast()
                    || ip.is_unique_local()
                    || ip.is_unicast_link_local()
                    || ip.segments()[0..2] == [0x2001, 0x0db8] =>
            {
                return None;
            }
            _ => {}
        }
        let result = self.reader.lookup(ip).ok()?;
        let country: &str = result
            .decode_path(&path!["country", "names", "en"])
            .ok()??;
        let city: Option<&str> = result.decode_path(&path!["city", "names", "en"]).ok()?;
        let label = match city.filter(|city| !city.is_empty()) {
            Some(city) => format!("{city}, {country}"),
            None => country.to_owned(),
        };
        (label.len() <= 256).then_some(label)
    }
}
