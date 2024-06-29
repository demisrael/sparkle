use crate::imports::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PingRequest {}

impl Serializer for PingRequest {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u16, &1, writer)?;
        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u16, reader)?;
        Ok(Self {})
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PingResponse {}

impl Serializer for PingResponse {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u16, &1, writer)?;
        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u16, reader)?;
        Ok(Self {})
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetStatusRequest {}

impl Serializer for GetStatusRequest {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u16, &1, writer)?;
        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u16, reader)?;
        Ok(Self {})
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetStatusResponse {
    pub sparkled_version: String,
    pub network_id: NetworkId,
}

impl Serializer for GetStatusResponse {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        store!(u16, &1, writer)?;
        store!(String, &self.sparkled_version, writer)?;
        store!(NetworkId, &self.network_id, writer)?;
        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let _version = load!(u16, reader)?;
        let sparkled_version = load!(String, reader)?;
        let network_id = load!(NetworkId, reader)?;
        Ok(Self {
            sparkled_version,
            network_id,
        })
    }
}
