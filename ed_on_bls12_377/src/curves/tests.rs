#![cfg_attr(not(feature = "std"), no_std)]
use crate::EdwardsProjective;
use ark_algebra_test_templates::*;

test_group!(te; super::EdwardsProjective; te);
