const WPM: u16 = 10;
pub const T_DIT_MS: u16 = (60000) / (50 * WPM);
pub const T_DAH_MS: u16 = 3 * T_DIT_MS;
pub const T_ICS_MS: u16 = 3 * T_DIT_MS;
pub const T_IWS_MS: u16 = 7 * T_DIT_MS;
