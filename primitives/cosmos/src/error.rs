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

use core::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum DecodeTxError {
	EmptyFeeAmount,
	EmptyMessages,
	EmptyMsgSendAmount,
	EmptySignatures,
	EmptySigners,
	EmptyTxBytes,
	InvalidMsgData,
	InvalidSignDoc,
	InvalidTxData,
	TooLongTxBytes,
	TooManyFeeAmount,
	TooManyMsgSendAmount,
	TooManyMessages,
	TooManySignatures,
	TooManySigners,
	UnsupportedMsgType,
	UnsupportedSignerType,
	UnsupportedSignMode,
}

impl Display for DecodeTxError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			DecodeTxError::EmptyFeeAmount => write!(f, "empty fee amount"),
			DecodeTxError::EmptyMessages => write!(f, "empty messages"),
			DecodeTxError::EmptyMsgSendAmount => write!(f, "empty message send amount"),
			DecodeTxError::EmptySignatures => write!(f, "empty signatures"),
			DecodeTxError::EmptySigners => write!(f, "empty signers"),
			DecodeTxError::EmptyTxBytes => write!(f, "empty tx bytes"),
			DecodeTxError::InvalidMsgData => write!(f, "invalid message data"),
			DecodeTxError::InvalidSignDoc => write!(f, "invalid sign doc"),
			DecodeTxError::InvalidTxData => write!(f, "invalid tx data"),
			DecodeTxError::TooLongTxBytes => write!(f, "too long tx bytes"),
			DecodeTxError::TooManyFeeAmount => write!(f, "too many fee amount"),
			DecodeTxError::TooManyMessages => write!(f, "too many messages"),
			DecodeTxError::TooManyMsgSendAmount => write!(f, "too many message send amount"),
			DecodeTxError::TooManySignatures => write!(f, "too many signatures"),
			DecodeTxError::TooManySigners => write!(f, "too many signers"),
			DecodeTxError::UnsupportedMsgType => write!(f, "unsupported message type"),
			DecodeTxError::UnsupportedSignerType => write!(f, "unsupported signer type"),
			DecodeTxError::UnsupportedSignMode => write!(f, "unsupported sign mode"),
		}
	}
}
