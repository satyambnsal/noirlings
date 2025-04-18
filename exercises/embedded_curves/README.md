# Embedded Curves in Noir


## What are Embedded Curves?

In cryptography, an elliptic curve is a mathematical structure that provides a way to do complex cryptographic operations efficiently. These curves are used in cryptographic protocols like digital signatures and key exchanges.

To learn more about **Elliptic Curve**, go through [Ellipitic Curve Point Addition](https://www.rareskills.io/post/elliptic-curve-addition]) RareSkills Tutorial.

An **embedded curve** in Noir refers to an elliptic curve that lives within the context of another cryptographic system. The term "embedded" indicates that one curve is mathematically embedded within another. This concept is particularly important in zero-knowledge proof systems, where Noir operates.

Think of it this way:
- The "outer" curve is the one used by the proving system (like BN254 or BLS12-381)
- The "embedded" curve lives inside the scalar field of the outer curve

This setup allows for efficient implementations of cryptographic operations like digital signatures and encryption within zero-knowledge proofs.

## Key Concepts

1. **Elliptic Curve Point**: A point on an elliptic curve, typically represented by x and y coordinates, plus a flag indicating if it's the "point at infinity" (a special point that serves as the identity element in the group).

2. **Scalar Multiplication**: The operation of adding a point to itself multiple times, which is a fundamental operation in elliptic curve cryptography.

3. **Point Addition**: The operation of combining two points on an elliptic curve to get a third point.

4. **Generator**: A special point on the curve that can generate all other points through scalar multiplication.

5. **Multi-Scalar Multiplication (MSM)**: An optimized way to compute the sum of multiple scalar multiplications, which is a common operation in cryptographic protocols.
