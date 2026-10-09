//! Molten salt reactor models built on the OUTRAM PARK crates.
//!
//! Nothing is modelled yet. This crate pulls in every OUTRAM PARK backend crate
//! published on crates.io, so that models can be added here.
//!
//! Research, education and V&V only; not for facility operation, licensing or
//! safety decisions.

#[cfg(test)]
mod tests {
    /// Every dependency resolves and links: naming each crate here makes the
    /// build fail if one disappears from the dependency list.
    #[test]
    fn all_outram_park_crates_link() {
        use boon_lay as _;
        use chem_eng_real_time_process_control_simulator as _;
        use farrer_park as _;
        use kovan as _;
        use kovan_codegen as _;
        use kovan_common as _;
        use kovan_discovery as _;
        use kovan_literature as _;
        use kovan_metrics as _;
        use kovan_semantics as _;
        use kovan_web as _;
        use nee_soon as _;
        use njoy_outram_park_fork as _;
        use outram_blender as _;
        use outram_foam_appbuilder_lib as _;
        use outram_foam_basic_lib as _;
        use outram_foam_mesh as _;
        use outram_foam_multiphase as _;
        use outram_foam_turbulence_lib as _;
        use outram_mc_libs as _;
        use outram_park_digital_twin_engine as _;
        use outram_park_fork_cfmesh as _;
        use outram_park_fork_coolprop as _;
        use outram_park_fork_dwsim_libs as _;
        use outram_park_fork_liggghts as _;
        use petir as _;
        use raffles as _;
        use tampines as _;
        use tampines_steam_tables as _;
        use teh_o_prke as _;
        use tuas_boussinesq_solver as _;
    }
}
