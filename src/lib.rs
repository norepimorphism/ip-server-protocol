// SPDX-License-Identifier: MPL-2.0

use std::fmt;

pub const PORT: u16 = 5350;
pub static VERSION: &str = "0.1.0";

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Query {
    IpspVersion,
    PrivacyPolicy,
    TermsOfService,
    ContactInformation,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IpspVersion => write!(f, "Protocol Version"),
            Self::PrivacyPolicy => write!(f, "Privacy Policy"),
            Self::TermsOfService => write!(f, "Terms of Service"),
            Self::ContactInformation => write!(f, "Contact Information"),
        }
    }
}

impl Query {
    pub const fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'v' => Some(Self::IpspVersion),
            b'p' => Some(Self::PrivacyPolicy),
            b't' => Some(Self::TermsOfService),
            b'c' => Some(Self::ContactInformation),
            _ => None,
        }
    }

    pub const fn as_byte(self) -> u8 {
        match self {
            Self::IpspVersion => b'v',
            Self::PrivacyPolicy => b'p',
            Self::TermsOfService => b't',
            Self::ContactInformation => b'c',
        }
    }
}

pub fn run(print_usage: impl FnOnce(), main: impl FnOnce()) {
    if user_wants_help() {
        print_usage();
    } else {
        main();
    }
}

fn user_wants_help() -> bool {
    std::env::args().any(|arg| matches!(arg.as_str(), "-h" | "--help" | "help"))
}
