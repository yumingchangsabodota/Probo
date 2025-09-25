


## Extrinsics

```rust
/// Stores an issuance proof on the blockchain
pub fn store_proof(
    origin: OriginFor<T>, // The issuing entity's public key used for verification, should get from Origin<T>
    proof: BoundedVec<u8, ConstU32<512>>,     // The cryptographically signed message hash
    expiration: U256 // Block number in future when this proof expires
) -> DispatchResult

```
