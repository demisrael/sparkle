use kaspa_consensus_core::constants::SOMPI_PER_KASPA;

pub const FEE_DEPLOY: u64 = 1_000 * SOMPI_PER_KASPA;
pub const FEE_MINT: u64 = SOMPI_PER_KASPA;
pub const PROTOCOL_NAMESPACE: &str = "kasplex";
pub const PROTOCOL_ID: &str = "KRC-20";

pub const KASPLEX_HEADER_LC: [u8; 7] = [107, 97, 115, 112, 108, 101, 120];
pub const KASPLEX_HEADER_UC: [u8; 7] = [75, 65, 83, 80, 76, 69, 88];
pub const KRC20_HEADER_UC: [u8; 6] = [75, 82, 67, 45, 50, 48];
pub const KRC20_HEADER_LC: [u8; 6] = [107, 114, 99, 45, 50, 48];
