use log::trace;
use std::marker::PhantomData;

use paired::bls12_381::Fr;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use sha2raw::utils as sha2utils;
use storage_proofs_core::{fr32::bytes_into_fr_repr_safe, hasher::Hasher};

use crate::encode::encode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodingProof<H: Hasher> {
    pub(crate) parents: Vec<H::Domain>,
    pub(crate) layer_index: u32,
    pub(crate) node: u64,
    #[serde(skip)]
    _h: PhantomData<H>,
}

impl<H: Hasher> EncodingProof<H> {
    pub fn new(layer_index: u32, node: u64, parents: Vec<H::Domain>) -> Self {
        EncodingProof {
            layer_index,
            node,
            parents,
            _h: PhantomData,
        }
    }

    fn create_key(&self, replica_id: &H::Domain) -> H::Domain {
        let mut hasher = Sha512::new();
        let mut buffer = [0u8; 128];

        // replica_id
        buffer[..64].copy_from_slice(
            &sha2utils::bits256_expand_to_bits512(AsRef::<[u8]>::as_ref(replica_id))[..],
        );

        // layer index
        buffer[64..68].copy_from_slice(&(self.layer_index as u32).to_be_bytes());
        // node id
        buffer[68..76].copy_from_slice(&(self.node as u64).to_be_bytes());

        hasher.update(&buffer[..]);

        // parents
        for parent in &self.parents {
            hasher.update(&sha2utils::bits256_expand_to_bits512(AsRef::<[u8]>::as_ref(parent))[..]);
        }

        bytes_into_fr_repr_safe(&hasher.finalize().as_ref()[..32]).into()
    }

    pub fn verify<G: Hasher>(
        &self,
        replica_id: &H::Domain,
        exp_encoded_node: &H::Domain,
        decoded_node: &G::Domain,
    ) -> bool {
        let key = self.create_key(replica_id);

        let fr: Fr = (*decoded_node).into();
        let encoded_node = encode(key, fr.into());

        check_eq!(exp_encoded_node, &encoded_node);

        true
    }
}
