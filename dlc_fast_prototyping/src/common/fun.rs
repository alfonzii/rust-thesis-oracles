// src/common/fun.rs

use secp256k1_zkp::Message;
use sha2::{Digest, Sha256};

// Based on payout and total collateral (from contract descriptor), create a string that represents the CET
pub fn create_cet(payout: u64, total_collateral: u64) -> String {
    format!(
        "Alice gets {} sats and Bob gets {} sats from DLC",
        total_collateral - payout,
        payout
    )
}

// Create a message from the CET (might be of any type representable as bytes)
pub fn create_message<T: AsRef<[u8]>>(cet: T) -> Result<Message, secp256k1_zkp::UpstreamError> {
    let hash = Sha256::digest(cet.as_ref());
    let hashed_msg: [u8; 32] = hash.into();
    Message::from_digest_slice(&hashed_msg)
}

// TODO: asi najleepsie by to bolo potom niekedy prerobit tak, ze CET struct alebo trait by enkapsulovala tieto 2 funkcie, s ohladom na to, ci bude CET typu String alebo realna Bitcoin TX (z btc api).
// zaroven, ak bude cas, tak sa moze premysliet sposob jak to ukladat, ci ulozit iba CET, ci ulozit iba payout, ci ukladat message a rekonstruovat zbytok... kazdy pristup ma svoje pre a proti
// aktualne, pre jednoduchost lebo to uz takto mam urobene pouzivam toto ze ulozene mam CETy (tj stringy teraz) a message si vytvaram (avsak toto netrva dlho, len nejak 60 ns, takze zanedbatelne)

// najlepsie asi komprromisne riesenie mi pride, ze je drzat si payout a msg. pretoze msg ked raz vytvorim tak nemusim ho vytvarat znova a zaroven, ak mame namiestoo CET ulozeny len payout tak setrime miestom oproti CET.
// a volat CET nam treba iba raz, pri finalize a eventualne pri broadcast.
