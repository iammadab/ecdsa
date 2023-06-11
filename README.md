# ECDSA
ECDSA Implementation with secp256k1 Curve

## Description
This project implements the ECDSA (Elliptic Curve Digital Signature Algorithm) on the secp256k1 elliptic curve from scratch in Rust. It provides functionality for both signing and verification of digital signatures.

The secp256k1 curve is widely used in various cryptographic applications, including cryptocurrencies like Bitcoin. This project aims to provide a basic understanding of the ECDSA algorithm and Elliptic curve operations specifically secp256k1.

## Features
- Basic implementation of elliptic curve point operations.
- Implementation of the secp256k1 curve. 
- ECDSA signing of messages using the generated keys.
- Verification of ECDSA signatures using the corresponding public keys.

## Future Work
- Performance: current implementation is extremely slow, plan to use this as a driver for exploring state of the art ecc operation optimization. 
- Security: enhance the security by implementing countermeasures against potential attacks e.g. side-channel attacks. 

## Note

⚠️ **Important: This Project is for Educational Purposes Only**

This project is implemented for educational purposes and should not be used in production systems. The code provided serves as a learning resource for understanding elliptic curve cryptography (ECC) and digital signature schemes, specifically ECDSA on the secp256k1 curve.

Implementing secure and robust cryptographic algorithms requires extensive expertise and rigorous testing. It is strongly recommended to rely on established and thoroughly reviewed cryptographic libraries, such as `secp256k1`, for production-grade systems.

Use this project responsibly and solely for educational purposes. Do not use it for real-world applications or to secure sensitive information without proper expert guidance and code auditing.