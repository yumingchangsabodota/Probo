
# Probo

Probo is a PoC blockchain for secure, privacy-preserving document verification across borders. The system stores cryptographic proofs instead of raw document data, enabling real-time authentication without exposing sensitive information or requiring centralized authorities.

Read the [overview](OVERVIEW.md) for the background and justification of the necessity of such blockchain system.


## Main Functionalities
- **Register Whitelist Entity**
    - Only registered entities are legitimate for storing proofs
    - In real-world scenarios, we may require entities to register a DID, or we may have a collective where only collective members can include new entity registrations
- **Store Proof**
    - As this chain is designed to be generic and suit different scenarios, proof generation and verification should be built on the DApp side. The blockchain is mainly for storing and sharing proofs in a trustless way.


