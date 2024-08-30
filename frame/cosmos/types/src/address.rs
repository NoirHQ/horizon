// This file is part of Horizon.

// Copyright (C) 2023 Haderech Pte. Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sp_core::H160;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AddressError {
	Bech32Error(bech32::DecodeError),
}

pub fn address_from_bech32(address: &str) -> Result<H160, AddressError> {
	bech32::decode(address)
		.map(|(_hrp, data)| H160::from_slice(&data))
		.map_err(AddressError::Bech32Error)
}
