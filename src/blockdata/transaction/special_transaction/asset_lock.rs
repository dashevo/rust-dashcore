// Rust Dash Library
// Written for Dash in 2022 by
//     The Dash Core Developers
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Dash Asset Lock Special Transaction.
//!
//! The asset lock special transaction is used to add to the asset lock credit pool.
//!
//!
//! It is defined in DIPX https://github.com/dashpay/dips/blob/master/dip-000X.md as follows:
//!
//!
//! The special transaction type used for AssetLockTx Transactions is 8.

use std::io;
use std::io::{Error, Write};
use consensus::{Decodable, Encodable, encode};
use TxOut;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AssetLockPayload {
    version: u8,
    count: u8,
    credit_outputs: Vec<TxOut>,
}

impl Encodable for AssetLockPayload {
    fn consensus_encode<S: Write>(&self, mut s: S) -> Result<usize, Error> {
        let mut len = 0;
        len += self.version.consensus_encode(&mut s)?;
        len += self.count.consensus_encode(&mut s)?;
        len += self.credit_outputs.consensus_encode(&mut s)?;
        Ok(len)
    }
}

impl Decodable for AssetLockPayload {
    fn consensus_decode<D: io::Read>(mut d: D) -> Result<Self, encode::Error> {
        let version = u8::consensus_decode(&mut d)?;
        let count = u8::consensus_decode(&mut d)?;
        let credit_outputs = Vec::<TxOut>::consensus_decode(&mut d)?;
        Ok(AssetLockPayload {
            version,
            count,
            credit_outputs,
        })
    }
}