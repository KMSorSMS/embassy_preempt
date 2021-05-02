use crate::time::Hertz;

/// Frozen core clock frequencies
#[derive(Clone, Copy)]
pub struct CoreClocks {
    pub hclk: Hertz,
    pub pclk1: Hertz,
    pub pclk2: Hertz,
    pub pclk3: Hertz,
    pub pclk4: Hertz,
    pub ppre1: u8,
    pub ppre2: u8,
    pub ppre3: u8,
    pub ppre4: u8,
    pub csi_ck: Option<Hertz>,
    pub hsi_ck: Option<Hertz>,
    pub hsi48_ck: Option<Hertz>,
    pub lsi_ck: Option<Hertz>,
    pub per_ck: Option<Hertz>,
    pub hse_ck: Option<Hertz>,
    pub mco1_ck: Option<Hertz>,
    pub mco2_ck: Option<Hertz>,
    pub pll1_p_ck: Option<Hertz>,
    pub pll1_q_ck: Option<Hertz>,
    pub pll1_r_ck: Option<Hertz>,
    pub pll2_p_ck: Option<Hertz>,
    pub pll2_q_ck: Option<Hertz>,
    pub pll2_r_ck: Option<Hertz>,
    pub pll3_p_ck: Option<Hertz>,
    pub pll3_q_ck: Option<Hertz>,
    pub pll3_r_ck: Option<Hertz>,
    pub timx_ker_ck: Hertz,
    pub timy_ker_ck: Hertz,
    pub sys_ck: Hertz,
    pub c_ck: Hertz,
}
