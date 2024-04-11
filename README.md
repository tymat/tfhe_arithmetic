# Secret Arithmetic Server Using TFHE-rs

## About

Fully Homomorphic Encryption (FHE) is a groundbreaking technology in the field of cryptography that enables computation on encrypted data without ever needing to decrypt it. This means that sensitive information can be processed in encrypted form, maintaining data privacy and security throughout the computation process. The advent of FHE opens up new possibilities for secure data analysis, cloud computing, and privacy-preserving technologies.

Torus FHE is a specific implementation of Fully Homomorphic Encryption that leverages the mathematical structure of a torus for efficient and secure computations. This approach aims to address some of the key challenges in FHE, such as computational complexity and performance scalability, making it more practical for real-world applications

This is a reference implementation that demonstrates the key generation and FHE computations on encrypted data.  

## Usage

### Building

Install Rust development environment 

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Generating client keys


```shell
target/release/create_client <seed>
```
Where seed is an unsigned 128-bit integer.  (0 .. 340282366920938463463374607431768211455)

#### Example

```shell
target/release/create_client 113427455640312821154458222707257705001
```

This generates a file `client.json` in the `$PWD`


### Generating Request

#### Operations 

| Operation | Opcode       | Arithmetic Operator |
|-----------|--------------|---------------------|
| 1         | Add          | a + b               |
| 2         | Sub          | a - b               |
| 3         | Mul          | a * b               |
| 4         | Div          | a / b               |
| 5         | BitAnd       | a & b               |
| 6         | BitAndAssign | a &= b              |
| 7         | BitOr        | a \| b              |
| 8         | BitXor       | a ^ b               |
| 9         | BitXorAssign | a ^= b              |
| 10        | DivAssign    | a /= b              |
| 11        | RotateLeft   | ROL(a)              |
TODO

### Starting the secret arithmetic server

TODO

### Sending request

TODO

### Decrypting the answer

TODO