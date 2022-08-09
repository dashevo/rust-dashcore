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

//! Dash Provider Update Service Special Transaction.
//!
//! The provider update service special transaction is used to update the operator controlled
//! options for a masternode.
//!
//! It is defined in DIP3 https://github.com/dashpay/dips/blob/master/dip-0003.md as follows:
//!
//! To service update a masternode, the masternode operator must submit another special
//! transaction (DIP2) to the network. This special transaction is called a Provider Update
//! Service Transaction and is abbreviated as ProUpServTx. It can only be done by the operator.
//!
//! An operator can update the IP address and port fields of a masternode entry. If a non-zero
//! operatorReward was set in the initial ProRegTx, the operator may also set the
//! scriptOperatorPayout field in the ProUpServTx. If scriptOperatorPayout is not set and
//! operatorReward is non-zero, the owner gets the full masternode reward.
//!
//! A ProUpServTx is only valid for masternodes in the registered masternodes subset. When
//! processed, it updates the metadata of the masternode entry and revives the masternode if it was
//! previously marked as PoSe-banned.
//!
//! The special transaction type used for ProUpServTx Transactions is 2.


use std::io;
use std::io::{Error, Write};
use hashes::Hash;
use ::{Script};
use blockdata::transaction::special_transaction::SpecialTransactionBasePayloadEncodable;
use bls_sig_utils::BLSSignature;
use consensus::{Decodable, Encodable, encode};
use ::{InputsHash, SpecialTransactionPayloadHash};
use Txid;

/// A Provider Update Service Payload used in a Provider Update Service Special Transaction.
/// This is used to update the operational aspects a Masternode on the network.
/// It must be signed by the operator's key that was set either at registration or by the last
/// registrar update of the masternode.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProviderUpdateServicePayload {
    version: u16,
    pro_tx_hash: Txid,
    ip_address: u128,
    port: u16,
    script_payout: Script,
    inputs_hash: InputsHash,
    payload_sig: BLSSignature,
}

impl SpecialTransactionBasePayloadEncodable for ProviderUpdateServicePayload {
    fn base_payload_data_encode<S: Write>(&self, mut s: S) -> Result<usize, Error> {
        let mut len = 0;
        len += self.version.consensus_encode(&mut s)?;
        len += self.pro_tx_hash.consensus_encode(&mut s)?;
        len += self.ip_address.consensus_encode(&mut s)?;
        len += u16::from_be(self.port).consensus_encode(&mut s)?;
        len += self.script_payout.consensus_encode(&mut s)?;
        len += self.inputs_hash.consensus_encode(&mut s)?;
        Ok(len)
    }

    fn base_payload_hash(&self) -> SpecialTransactionPayloadHash {
        let mut engine = SpecialTransactionPayloadHash::engine();
        self.base_payload_data_encode(&mut engine).expect("engines don't error");
        SpecialTransactionPayloadHash::from_engine(engine)
    }
}

impl Encodable for ProviderUpdateServicePayload {
    fn consensus_encode<S: Write>(&self, mut s: S) -> Result<usize, Error> {
        let mut len = 0;
        len += self.base_payload_data_encode(&mut s)?;
        len += self.payload_sig.consensus_encode(&mut s)?;
        Ok(len)
    }
}

impl Decodable for ProviderUpdateServicePayload {
    fn consensus_decode<D: io::Read>(mut d: D) -> Result<Self, encode::Error> {
        let version = u16::consensus_decode(&mut d)?;
        let pro_tx_hash = Txid::consensus_decode(&mut d)?;
        let ip_address = u128::consensus_decode(&mut d)?;
        let port = u16::from_be(u16::consensus_decode(&mut d)?);
        let script_payout = Script::consensus_decode(&mut d)?;
        let inputs_hash = InputsHash::consensus_decode(&mut d)?;
        let payload_sig = BLSSignature::consensus_decode(&mut d)?;

        Ok(ProviderUpdateServicePayload {
            version,
            pro_tx_hash,
            ip_address,
            port,
            script_payout,
            inputs_hash,
            payload_sig,
        })
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use std::net::Ipv4Addr;
    use hashes::hex::{FromHex, ToHex};
    use consensus::deserialize;
    use ::{Network, Transaction};
    use ::{InputsHash, Txid};
    use blockdata::transaction::special_transaction::provider_update_service::ProviderUpdateServicePayload;
    use blockdata::transaction::special_transaction::SpecialTransactionBasePayloadEncodable;
    use blockdata::transaction::special_transaction::TransactionPayload::ProviderUpdateServicePayloadType;
    use ::{Script};

    #[test]
    fn test_provider_update_service_transaction() {
        // This is a test for testnet
        let network = Network::Testnet;

        let expected_transaction_bytes = hex::decode("03000200018f3fe6683e36326669b6e34876fb2a2264e8327e822f6fec304b66f47d61b3e1010000006b48304502210082af6727408f0f2ec16c7da1c42ccf0a026abea6a3a422776272b03c8f4e262a022033b406e556f6de980b2d728e6812b3ae18ee1c863ae573ece1cbdf777ca3e56101210351036c1192eaf763cd8345b44137482ad24b12003f23e9022ce46752edf47e6effffffff0180220e43000000001976a914123cbc06289e768ca7d743c8174b1e6eeb610f1488ac00000000b501003a72099db84b1c1158568eec863bea1b64f90eccee3304209cebe1df5e7539fd00000000000000000000ffff342440944e1f00e6725f799ea20480f06fb105ebe27e7c4845ab84155e4c2adf2d6e5b73a998b1174f9621bbeda5009c5a6487bdf75edcf602b67fe0da15c275cc91777cb25f5fd4bb94e84fd42cb2bb547c83792e57c80d196acd47020e4054895a0640b7861b3729c41dd681d4996090d5750f65c4b649a5cd5b2bdf55c880459821e53d91c9").unwrap();

        let expected_transaction: Transaction = deserialize(expected_transaction_bytes.as_slice()).expect("expected a transaction");

        let expected_provider_update_service_payload = expected_transaction.special_transaction_payload.clone().unwrap().to_update_service_payload().expect("expected to get a provider registration payload");

        let tx_id = Txid::from_hex("fa2f2eba320c56fb0efebe2ace3333024104d8d0a30753da36db4bf97c119be7").expect("expected to decode tx id");
        let input_transaction_hash_value = InputsHash::from_hex("ca9a43051750da7c5f858008f2ff7732d15691e48eb7f845c791e5dca78bab58").expect("expected to decode inputs hash");

        let provider_update_service_payload_version = 1;
        assert_eq!(expected_provider_update_service_payload.version, provider_update_service_payload_version);
        let pro_tx_hash = Txid::from_hex("fd39755edfe1eb9c200433eecc0ef9641bea3b86ec8e5658111c4bb89d09723a").expect("expected to decode tx id");
        assert_eq!(expected_provider_update_service_payload.pro_tx_hash, pro_tx_hash);

        let address = Ipv4Addr::from_str("52.36.64.148").expect("expected an ipv4 address");
        let [a, b, c, d] = address.octets();
        let ipv6_bytes: [u8;16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, a, b, c, d];
        assert_eq!(expected_provider_update_service_payload.ip_address.to_le_bytes().to_hex(), ipv6_bytes.to_hex());

        let port = 19999;
        assert_eq!(expected_provider_update_service_payload.port, port);

        let inputs_hash_hex = "b198a9735b6e2ddf2a4c5e1584ab45487c7ee2eb05b16ff08004a29e795f72e6";
        assert_eq!(expected_provider_update_service_payload.inputs_hash.to_hex(), inputs_hash_hex, "inputs hash calculation has issues");

        assert_eq!(expected_provider_update_service_payload.base_payload_hash().to_hex(), "9784b3663039784858420677b00f0b3f34af8ff1f1788adfd0e681d345b776ba", "Payload hash calculation has issues");

        // We should verify the script payouts match
        let script_payout = Script::new();
        assert_eq!(expected_provider_update_service_payload.script_payout, script_payout);

        assert_eq!(expected_transaction.txid(), tx_id);

        //todo: once we have a BLS signatures library in rust we should implement signing
        let payload_sig = expected_transaction.special_transaction_payload.clone().unwrap().to_update_service_payload().unwrap().payload_sig;

        let transaction = Transaction {
            version: 3,
            lock_time: 0,
            input: expected_transaction.input.clone(), // todo:implement this
            output: expected_transaction.output.clone(), // todo:implement this
            special_transaction_payload: Some(ProviderUpdateServicePayloadType(ProviderUpdateServicePayload {
                version: provider_update_service_payload_version,
                pro_tx_hash,
                ip_address: u128::from_le_bytes(ipv6_bytes),
                port,
                script_payout,
                inputs_hash: InputsHash::from_hex(inputs_hash_hex).unwrap(),
                payload_sig
            }))
        };

        assert_eq!(transaction.hash_inputs().to_hex(), inputs_hash_hex);

        assert_eq!(transaction, expected_transaction);

        assert_eq!(transaction.txid(), tx_id);
    }
}