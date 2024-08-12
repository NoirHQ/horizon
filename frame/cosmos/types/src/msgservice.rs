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

use crate::{errors::CosmosError, events::CosmosEvent};
use cosmos_sdk_proto::Any;
use frame_support::weights::Weight;
use sp_std::vec::Vec;

pub struct MsgHandlerErrorInfo {
	pub weight: Weight,
	pub error: CosmosError,
}

pub trait MsgHandler {
	fn handle(&self, msg: &Any) -> Result<(Weight, Vec<CosmosEvent>), MsgHandlerErrorInfo>;
}

pub trait MsgServiceRouter {
	fn route(type_url: &str) -> Option<sp_std::boxed::Box<dyn MsgHandler>>;
}
