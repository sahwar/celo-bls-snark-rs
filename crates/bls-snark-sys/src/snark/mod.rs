pub mod epoch_block;
use epoch_block::{read_slice, EpochBlockFFI};

#[cfg(test)]
mod test_helpers;

use crate::convert_result_to_bool;
use epoch_snark::EpochBlock;
use std::convert::TryFrom;

#[no_mangle]
/// Verifies a Groth16 proof about the validity of the epoch transitions
/// between the provided `first_epoch` and `last_epoch` blocks.
///
/// All elements are assumed to be sent as serialized byte arrays
/// of **compressed elements**. There are no assumptions made about
/// the length of the verifying key or the proof, so that must be
/// provided by the caller.
///
/// # Safety
/// 1. VK and Proof must be valid pointers
/// 1. The vector of pubkeys inside EpochBlockFFI must point to valid memory
pub unsafe extern "C" fn verify(
    // Serialized verifying key
    vk: *const u8,
    // Length of serialized verifying key
    vk_len: u32,
    // Serialized proof
    proof: *const u8,
    // Length of serialized proof
    proof_len: u32,
    // First epoch data (pubkeys serialized)
    first_epoch: EpochBlockFFI,
    // Last epoch data (pubkeys serialized)
    last_epoch: EpochBlockFFI,
) -> bool {
    convert_result_to_bool(|| {
        let first_epoch = EpochBlock::try_from(&first_epoch)?;
        let last_epoch = EpochBlock::try_from(&last_epoch)?;
        let vk = read_slice(vk, vk_len as usize)?;
        let proof = read_slice(proof, proof_len as usize)?;

        epoch_snark::verify(&vk, &first_epoch, &last_epoch, &proof)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snark::EpochBlockFFI;

    #[test]
    // Trimmed down version of the other E2E groth test to ensure
    // that the verifier works correctly for a proof which we have verified on our own
    fn simple_verifier_groth16() {
        let serialized_proof = hex::decode(PROOF).unwrap();
        let serialized_vk = hex::decode(VK).unwrap();

        // Get the corresponding pointers
        let proof_ptr = &serialized_proof[0] as *const u8;
        let vk_ptr = &serialized_vk[0] as *const u8;

        let first_pubkeys = hex::decode(FIRST_PUBKEYS).unwrap();
        let last_pubkeys = hex::decode(LAST_PUBKEYS).unwrap();

        let first_epoch = EpochBlockFFI {
            index: 0,
            maximum_non_signers: 1,
            pubkeys_num: 4,
            pubkeys: &first_pubkeys[0] as *const u8,
        };

        let last_epoch = EpochBlockFFI {
            index: 2,
            maximum_non_signers: 1,
            pubkeys_num: 4,
            pubkeys: &last_pubkeys[0] as *const u8,
        };

        // Make the verification
        let res = unsafe {
            verify(
                vk_ptr,
                serialized_vk.len() as u32,
                proof_ptr,
                serialized_proof.len() as u32,
                first_epoch,
                last_epoch,
            )
        };
        assert!(res);
    }

    const PROOF: &str = "33796bc0cdbc50464a385a36e2c1ef80ae43372efc3a61f6274d6d51e6e3416986255d51a2259a11bf1195b25ac99f45958aaf352bfbd95be29d452aebdc566b54db0088f4ef5ca5365e2f3932c753c54c285ecd320e2a909edc4ac4ee3b2a82fc26bc53b4823a344578112646803524e13835d82ec38437d309d8e7d4d2094444bf0ecc61e3342e3260361fec644dc092346f03f9d1b796be7ef33579a38bddff69b4a9e080d90ce9712e974a450c5f6757807459ff257ccbb76e654ccccb90b4e82e1be04756b49f07f52a9a9eefd5f1ed3896df3ea10d55a6504d38012f60a6dda539ddc258a27004a9f30206c280230ee2928a6562f8e0bf67c60e2770cbc0020051b88c3c087abe93951e492e25a5b15dd99fa04fa0638dbc3b9fe358b9874f68d88e247cbaa0fae4ce250f432acafcc01ec6d248910884a3c9f78f5b0a1020db8c7b5cdca1a8dccb697d56f1a3592c5ae9f629fa17df7df08f94c31d21955dc4de4d5429ef742a69e9b35ff22b1649d4528a52d2f28f97abeeac93c665e1296da84a03723165e9e8fc71c09fd389bb1282fa212777ade68a7bed836ffb79dc1d2f9c091d5dd12c39705ccb4121de3bcf0e21a571a2c777e6271bb9c1e556ed46276fbe31a20aa488f13211883e5cce80692fe08b3ac3f6131435b2dbd28208fc114accdc69cb8b";

    const VK: &str = "27de9b5798fd5c832aaabe2b1bb480473df29203a3a1971aed07a21441a6f21573031b01b821546c16f8470dda69131439854051daf5fb6ab7b6815cd9acc0c583ca1376d37343d69fc397323be1d601c7cd43fc42d4361aa27cdf8bc565d3731a93514b0fd38659ebc40f3c44ba9c6882a2e820828135d292d9bb7734afb8094e1e8a9c755420b93155ac4caefe44dd6a64e732daf6e0e141b26c88a30353d627e3a4afec307e285607d18261c04cf8b568eb4fdb3f48da889e8d46c4ff2712fd5664326a9642161463b234dd7b89f1fccb1a6c9acc26b1994c58b68106276345ecf79e0137a941be4bca06489616810b7db4d93fd8b3f6615e84b9683112cbf8e696867656616b5dbd60553df7f92fc1300c0aaf6217b2561b7f10671ae540c209f22a8e2cb98fb60e118aa2c372a44db6a404eb19d7e68ec9d0f0b4521a7f405ca2f421cf4b4128176a34cdf49a9a471dc197e4ee5c8ccb9342fb967173eb3137659930767af5228d1c36038885512286189c358cfc053c1319d4a2f8e50ca32ab13468d86aa059ed201de13f0eae7f4a687baa5d212df991cc5e656200fe705cb8e293614eebc196acc1a5d493d189162e0eeef2c68a37d58cc6717e9b649a595293cd18cc27f8aed9863b0a727b5e12cfa4e90513a282bb65658bd786968f89993ebbc7006f810f0cb6a7002599739fa40f337eaab7e639f79ebdf85ffecf106d5fa8464a197803f8a742a1ef2f05d03c592c97abdd0847c45f5b8878443319618ff04b7a3de13ce778c58c79b1c7f8d6e900d02fa1279dde856c7a62e1b13aff7a42b63bed3c2100318d515c0175c0c067b1ae5b4dc17c3423290c0d5c5e4c6da641073562f5ba99de560671bce41e2aa0abbeb2176b13802e58ba424ce8d790c7d5b610a409e26c718ef1a939198d3a32aef7f5b83347b15fc2b4adfebc468e7bb4f5667a70ea45c3b7135e0efc2e1738ab5b033176f75780fa17415fb5037c748d6be0a6ae68235cd307bf08f890261378aad62ef89395e33004c1e5f50a9fc261fd6a6f175e64110f15af68d3e888f132348039e3fb8db3ac974b8b07867b452bb73584d4c6e7ba5f790c119500b7b538cddcfc81ee754db65aca15f5853c806a40b38e02c0ed2a797fcc11afe2e26f972b4a7f8fe70e626e5d3b4fa600ee730f6a27b9fb448daa9b364ede0feefba1fc74a2d546eb74ac2c61f7d8033e0df3e214d3f12b8bf229692148305e2d82e3ad4c8266b7da641a42c21689e1e738b2689d1bc31afcb6d2b53a87afabd970de2da2e26431a1d4178a6808a23ca803a479e15ea5b7875adc1f4456ce88a3a94a7aecc9763899a353bcc1363a746ab55bd3322d44919eb0a3350c1a8eb574f7220300000000000000a21df711258265911c858a49dc5e866c47f4da8c6f24c4d5002156f719631fcf22af144657300ce576ca5523c69a2cef3e1771a054f51365321a78e2877a8505a02a25300f414035e4f84517141ada5e702376358634fe0b2d5477bfcda04f6ced04908edbd11f470d8e060600031662f9bb513919ab3b21846461a9c70c486adb9a200f16eeb66bf4d550bcfc44b2b29af9338b9c054f04c00eda28301a5d78ad5352fb88ee7623a5554a08a8f886e325a9a5cc4071fbe168860614a0a4e6ffd3aa7c2837fea5669ccd78c6c4c6573d1512ac83ecca8bb2b31d2a7a8870e84763eee2e6f47dc83ab5a78c6a45254c003853a563d59a00bc9cfec4586ef5b5b6e37564304f1ea1ad6c002b0f016863e0af2229a473258e9f1d91926ad75a310c5adead0bbda1";

    const FIRST_PUBKEYS: &str = "3dfaa2527c6e3870170898aca8b7285c3dd2f748427d9b0d368e3764917831c82d0e5571819162d721f4b1aeb02f4601b7709ef14786ca43c16fd0416265426734af20e705ad97a255d20176759cb0392d29cfffc21733e4e25e86767324a08153e8b0c43b72bfdac10fcdbcd8438a180d11ae7741b8bf34261652eb61d67022beba80d99a1611536188bf3d1966ac000a6c31c78230a841a97191ccd40aa30a5163bcbf655c3d4984750a31753145e5592b3795087996d80869b2ee1708a60026ce5dc1b1519987439e0791eb34ee50c9cc67919d0010232a2b106d60262189e237f228b731d30adfec4152947b5401b92cdf69ad0a3fd355465b29358aefb8ba99b2742fad5d4a032bac6e378ca4a84dfce4bfdad6a5858f0954876687b3803414bb082c1e925500d85971007e42432a5d34b865fd4fc3f91d514617401a8ec47b9b4e72c4c831886f48a71738c400583b8796cf9cf098dcb40548202946fdea0be1c6806d4ecd7784e23eab19b5dcbfec78de74c994717609f39e3d9c9001";

    const LAST_PUBKEYS: &str = "a3fa866f5b51ff226f70a6872aedb89020372b88847a75f1c20dee118a679aa077dd16d1c1d06d079dad2e91808c11002199f32b651a010807264c6a00d034c3d4a31fad24935b2395d2fa80e02b6ac3e1a25e0f202ba99aa6647efe28d68081134bc536f6a172b50ad3a75e633272e5bc53f0a1c9520c584e922dda377eb2b2858ca443769753ba182012e609cc9f01c0df40d4c108fa01e9e4805ccc4947e5ff9dd90ca30bcf4ee258a957118a3b1b1ed8493c595d7598ae4e639147ef4901a84ba4f0cf1beda01fa86c8b3a71f97f657456991a20210ef73f1379156967c822d984e459209961d892c376a67c650056ca5d645a01131836a9382afb936c8ee2a430f9af098df29d776b9b9c2000c3326630a98ba48f4eb610685bb9d66b00ff530875d838c4b68ced77572837a5b335f5056973a41249c6049bab694e1e54bffc3a3c6a940ef69d5029300b7d98009534be42de14dce9d0a74a6ff52ffd36efa050cae9efe47a409bf73f748f795e81a914a9863edad412cf2cfa3b2c9e80";
}
