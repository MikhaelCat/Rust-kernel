pub mod component_01;
pub mod component_02;
pub mod component_03;
pub mod component_04;

pub fn lockdown_enabled() -> bool {
    true
}
pub mod apparmor;
pub mod audit;
pub mod bpf_lsm;
pub mod capability;
pub mod component_05;
pub mod component_06;
pub mod component_07;
pub mod component_08;
pub mod component_09;
pub mod component_10;
pub mod cred;
pub mod ima;
pub mod integrity;
pub mod keyring;
pub mod landlock;
pub mod lsm;
pub mod policy;
pub mod random;
pub mod seccomp;
pub mod selinux;
pub mod tomoyo;
pub mod yama;

pub mod health;

pub mod hardening;

pub mod integration;

pub mod components;

pub mod crypto_gate;

pub mod bundle;
