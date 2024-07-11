// This file is part of Horizon.

// Copyright (C) 2023 Haderech Pte. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use frame_support::weights::Weight;
use pallet_cosmos_types::tx::Any;
use sp_runtime::RuntimeString;

pub struct MsgHandlerErrorInfo {
	pub weight: Weight,
	pub error: MsgHandlerError,
}

#[derive(Debug)]
pub enum MsgHandlerError {
	InvalidMsg,
	Unsupported,
	Custom(RuntimeString),
}

pub trait MsgHandler {
	fn handle(&self, msg: &Any) -> Result<Weight, MsgHandlerErrorInfo>;
}

pub trait MsgServiceRouter {
	fn route(type_url: &[u8]) -> Option<sp_std::boxed::Box<dyn MsgHandler>>;
}
