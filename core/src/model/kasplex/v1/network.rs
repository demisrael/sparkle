use crate::imports::*;
use crate::url::Url;
use kaspa_consensus_core::network::NetworkId;
use kaspa_consensus_core::network::NetworkType;

pub enum Network {
    Mainnet,
    Testnet10,
    Testnet11,
}

impl From<Network> for Url {
    fn from(network: Network) -> Self {
        let url = match network {
            Network::Mainnet => "https://api.kasplex.org/v1",
            Network::Testnet10 => panic!("Testnet10 is not supported"),
            Network::Testnet11 => "https://tn11api.kasplex.org/v1",
        };

        url.into()
    }
}

impl TryFrom<&NetworkId> for Network {
    type Error = Error;
    fn try_from(network_id: &NetworkId) -> std::result::Result<Self, Self::Error> {
        let NetworkId {
            network_type,
            suffix,
        } = network_id;
        match network_type {
            NetworkType::Mainnet => Ok(Network::Mainnet),
            NetworkType::Testnet if suffix == &Some(10) => Ok(Network::Testnet10),
            NetworkType::Testnet if suffix == &Some(11) => Ok(Network::Testnet11),
            _ => Err(Error::NetworkId(network_id.to_string())),
        }
    }
}

impl From<Network> for NetworkType {
    fn from(network: Network) -> Self {
        match network {
            Network::Mainnet => NetworkType::Mainnet,
            Network::Testnet10 => NetworkType::Testnet,
            Network::Testnet11 => NetworkType::Testnet,
        }
    }
}
