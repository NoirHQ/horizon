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

use hp_cosmos::{AccountId, PublicKey, SignerPublicKey, Tx};
use hp_io::crypto::secp256k1_ecdsa_verify;
use pallet_cosmos_modules::ante::AnteHandler;
use sp_core::{sha2_256, H160};
use sp_runtime::transaction_validity::{InvalidTransaction, TransactionValidityError};
use sp_std::{marker::PhantomData, vec::Vec};

pub struct SigVerificationHandler<T>(PhantomData<T>);

impl<T> AnteHandler for SigVerificationHandler<T>
where
	T: frame_system::Config + pallet_cosmos::Config,
{
	fn handle(tx: &Tx) -> Result<(), TransactionValidityError> {
		let signatures = &tx.signatures;

		let mut signers = Vec::<AccountId>::new();
		for msg in &tx.body.messages {
			if let Some(msg_signers) =
				hp_io::signers::get_msg_any_signers(&msg.type_url, &msg.value)
			{
				for msg_signer in msg_signers {
					if !signers.contains(&msg_signer) {
						signers.push(msg_signer);
					}
				}
			}
		}
		if let Some(fee_payer) = &tx.auth_info.fee.payer {
			if !signers.contains(fee_payer) {
				signers.push(fee_payer.clone());
			}
		}

		if signatures.len() != signers.len() {
			return Err(TransactionValidityError::Invalid(InvalidTransaction::BadSigner));
		}

		let signer_infos = &tx.auth_info.signer_infos;
		if signatures.len() != signer_infos.len() {
			return Err(TransactionValidityError::Invalid(InvalidTransaction::BadSigner));
		}

		for (i, sig) in signatures.iter().enumerate() {
			let signer = signers
				.get(i)
				.ok_or(TransactionValidityError::Invalid(InvalidTransaction::BadSigner))?;

			// TODO: Support other types of Signers as well
			let signer_info = signer_infos
				.get(i)
				.ok_or(TransactionValidityError::Invalid(InvalidTransaction::BadSigner))?;

			if let Some(SignerPublicKey::Single(PublicKey::Secp256k1(public_key))) =
				signer_info.public_key
			{
				let address: H160 = hp_io::crypto::ripemd160(&sha2_256(&public_key)).into();
				if signer.address != address {
					return Err(TransactionValidityError::Invalid(InvalidTransaction::BadSigner));
				}

				let (account, _) = pallet_cosmos::Pallet::<T>::account(&signer.address);
				if signer_info.sequence > account.sequence {
					return Err(TransactionValidityError::Invalid(InvalidTransaction::Future));
				} else if signer_info.sequence < account.sequence {
					return Err(TransactionValidityError::Invalid(InvalidTransaction::Stale));
				}

				if !secp256k1_ecdsa_verify(sig, &tx.hash.0, &public_key) {
					return Err(TransactionValidityError::Invalid(InvalidTransaction::BadProof));
				}
			} else {
				return Err(TransactionValidityError::Invalid(InvalidTransaction::BadSigner));
			}
		}

		Ok(())
	}
}
