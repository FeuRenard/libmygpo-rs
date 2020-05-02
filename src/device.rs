//! [Device API](https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html)

use crate::client::{AuthenticatedClient, DeviceClient};
use crate::episode::EpisodeActionType;
use crate::error::Error;
use crate::subscription::Podcast;
use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use url::Url;

/// Type of the [`Device`](./struct.Device.html)
#[serde(rename_all = "lowercase")]
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum DeviceType {
    /// desktop computer
    Desktop,
    /// portable computer
    Laptop,
    /// smartphone/tablet
    Mobile,
    /// server
    Server,
    /// any type of device, which doesn't fit another variant
    Other,
}

/// Devices are used throughout the API to identify a device / a client application.
#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct Device {
    /// A device ID can be any string matching the regular expression `[\w.-]+`. The client application MUST generate a string to be used as its device ID, and SHOULD ensure that it is unique within the user account. A good approach is to combine the application name and the name of the host it is running on.
    ///
    /// If two applications share a device ID, this might cause subscriptions to be overwritten on the server side. While it is possible to retrieve a list of devices and their IDs from the server, this SHOULD NOT be used to let a user select an existing device ID.
    pub id: String,
    /// Human readable label for the device
    pub caption: String,
    /// Type of the device
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub device_type: DeviceType,
    /// number of subscriptions for this device
    pub subscriptions: u16,
}

#[derive(Serialize)]
pub(crate) struct DeviceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) device_type: Option<DeviceType>,
}

/// episode update information as used in [DeviceUpdates](./struct.DeviceUpdates.html)
#[derive(Serialize, Deserialize)]
pub struct EpisodeUpdate {
    /// episode title
    pub title: String,
    /// episode URL
    pub url: Url,
    /// podcast title
    pub podcast_title: String,
    /// podcast URL
    pub podcast_url: Url,
    /// episode description
    pub description: String,
    /// episode website
    pub website: Url,
    /// gpodder.net internal URL
    pub mygpo_link: Url,
    /// episode release date
    pub released: NaiveDateTime,
    /// latest episode action reported for this episode
    pub status: Option<EpisodeActionType>,
}

/// updated information for a device as returned by [`get_device_updates`](./trait.GetDeviceUpdates.html#tymethod.get_device_updates)
#[derive(Serialize, Deserialize)]
pub struct DeviceUpdates {
    /// list of subscriptions to be added
    pub add: Vec<Podcast>,
    /// list of URLs to be unsubscribed
    pub rem: Vec<Url>,
    /// list of updated episodes
    pub updates: Vec<EpisodeUpdate>,
    /// current timestamp; for retrieving changes since the last query
    pub timestamp: u64,
}

/// see [`update_device_data`](./trait.UpdateDeviceData.html#tymethod.update_device_data)
pub trait UpdateDeviceData {
    /// Update Device Data
    ///
    /// # Parameters
    ///
    /// - `caption`: The new human readable label for the device
    /// - `device_type`: see [`DeviceType`](./enum.DeviceType.html)
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::DeviceClient;
    /// use mygpoclient::device::{DeviceType,UpdateDeviceData};
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// # let deviceid = std::env::var("GPODDER_NET_DEVICEID").unwrap();
    /// #
    /// let client = DeviceClient::new(&username, &password, &deviceid);
    ///
    /// client.update_device_data("My Phone".to_owned(), DeviceType::Mobile)?;
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html#update-device-data)
    fn update_device_data<T: Into<Option<String>>, U: Into<Option<DeviceType>>>(
        &self,
        caption: T,
        device_type: U,
    ) -> Result<(), Error>;
}

/// see [`list_devices`](./trait.ListDevices.html#tymethod.list_devices)
pub trait ListDevices {
    /// List Devices
    ///
    /// Returns the list of devices that belong to a user. This can be used by the client to let the user select a device from which to retrieve subscriptions, etc..
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::device::ListDevices;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let devices = client.list_devices()?;
    ///
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html#list-devices)
    fn list_devices(&self) -> Result<Vec<Device>, Error>;
}

/// see [`get_device_updates`](./trait.GetDeviceUpdates.html#tymethod.get_device_updates)
pub trait GetDeviceUpdates {
    /// Get Device Updates
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::DeviceClient;
    /// use mygpoclient::device::GetDeviceUpdates;
    /// # use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// # let deviceid = std::env::var("GPODDER_NET_DEVICEID").unwrap();
    /// # let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - 86400;
    /// #
    /// let client = DeviceClient::new(&username, &password, &deviceid);
    ///
    /// let device_updates = client.get_device_updates(timestamp, true)?;
    ///
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [gpodder.net API Documentation](https://gpoddernet.readthedocs.io/en/latest/api/reference/devices.html#get-device-updates)
    fn get_device_updates(&self, since: u64, include_actions: bool)
        -> Result<DeviceUpdates, Error>;
}

impl UpdateDeviceData for DeviceClient {
    fn update_device_data<T: Into<Option<String>>, U: Into<Option<DeviceType>>>(
        &self,
        caption: T,
        device_type: U,
    ) -> Result<(), Error> {
        let input = DeviceData {
            caption: caption.into(),
            device_type: device_type.into(),
        };
        self.post(
            &format!(
                "https://gpodder.net/api/2/devices/{}/{}.json",
                self.authenticated_client.username, self.device_id
            ),
            &input,
        )?;
        Ok(())
    }
}

impl ListDevices for AuthenticatedClient {
    fn list_devices(&self) -> Result<Vec<Device>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/api/2/devices/{}.json",
                self.username
            ))?
            .json()?)
    }
}

impl ListDevices for DeviceClient {
    fn list_devices(&self) -> Result<Vec<Device>, Error> {
        self.as_ref().list_devices()
    }
}

impl GetDeviceUpdates for DeviceClient {
    fn get_device_updates(
        &self,
        since: u64,
        include_actions: bool,
    ) -> Result<DeviceUpdates, Error> {
        let mut query_parameters: Vec<&(&str, &str)> = Vec::new();

        let since_string = since.to_string();
        let query_parameter_since = ("since", since_string.as_ref());
        query_parameters.push(&query_parameter_since);

        let include_actions_string = include_actions.to_string();
        let query_parameter_include_actions = ("include_actions", include_actions_string.as_ref());
        query_parameters.push(&query_parameter_include_actions);

        Ok(self
            .get_with_query(
                &format!(
                    "https://gpodder.net/api/2/updates/{}/{}.json",
                    self.authenticated_client.username, self.device_id
                ),
                &query_parameters,
            )?
            .json()?)
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Device {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Device {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Device {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} (id={})", self.device_type, self.caption, self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::{Device, DeviceType};
    use std::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn equal_device_means_equal_hash() {
        let device1 = Device {
            id: String::from("abcdef"),
            caption: String::from("gPodder on my Lappy"),
            device_type: DeviceType::Laptop,
            subscriptions: 27,
        };
        let device2 = Device {
            id: String::from("abcdef"),
            caption: String::from("unnamed"),
            device_type: DeviceType::Other,
            subscriptions: 1,
        };

        assert_eq!(device1, device2);
        assert_eq!(device1.partial_cmp(&device2), Some(Ordering::Equal));

        let mut hasher1 = DefaultHasher::new();
        device1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        device2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn not_equal_devices_have_non_equal_ordering() {
        let device1 = Device {
            id: String::from("abcdef"),
            caption: String::from("gPodder on my Lappy"),
            device_type: DeviceType::Laptop,
            subscriptions: 27,
        };
        let device2 = Device {
            id: String::from("phone-au90f923023.203f9j23f"),
            caption: String::from("My Phone"),
            device_type: DeviceType::Mobile,
            subscriptions: 5,
        };

        assert_ne!(device1, device2);
        assert_eq!(device1.partial_cmp(&device2), Some(Ordering::Less));

        let mut hasher1 = DefaultHasher::new();
        device1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        device2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn display() {
        let device = Device {
            id: String::from("abcdef"),
            caption: String::from("gPodder on my Lappy"),
            device_type: DeviceType::Laptop,
            subscriptions: 27,
        };

        assert_eq!(
            "Laptop gPodder on my Lappy (id=abcdef)".to_owned(),
            format!("{}", device)
        );
    }
}
