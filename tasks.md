Analyze the following architectural problem and propose a complete, efficient, and scalable implementation:

We are integrating a Ring-LWE based security module as an isolated cryptographic service for a mail application called QMail. The Ring-LWE module exposes three endpoints: /keygen, /encrypt, and /decrypt. Each key generation request returns a public key and a secret key. QMail must use these keys to encrypt outgoing mail content and decrypt incoming mail content, achieving a quantum-resistant message transfer pipeline.

You must analyze the existing Ring-LWE implementation and answer:

1. How can this security module be cleanly incorporated into QMail through HTTP endpoints so the main application securely sends plaintext for encryption and receives ciphertext back, and vice versa?

2. Does the current Ring-LWE implementation support concurrent requests? Can it handle multiple keygen, encrypt, and decrypt requests at high throughput, or is it single-threaded? If not scalable, define how to make it scalable.

3. Since keygen returns a pair of keys (public, secret), what is the correct, secure, and efficient strategy for:
   - Storing keys for each mail transfer?
   - Using the right key again for decrypting a specific message later?
   - Linking keys to message sessions in a clean architecture?

4. Describe how QMail should store and reuse public/secret keys per message transaction. Provide a recommended workflow for:
   - key generation request
   - associating the keys with a message
   - encrypting content using the public key
   - sending encrypted payload
   - decrypting using the secret key when needed

5. Provide an efficient architectural model for how these endpoints should be used together to build a quantum-resistant message sending and receiving pipeline.

6. Propose improvements to the Ring-LWE service so it can handle large-scale concurrent requests (hundreds or thousands per minute). Describe thread-safety, async, connection handling, and performance tuning.

7. Provide the actual recommended implementation pattern (code-level or structural) for achieving:
   - stateless scalable API
   - safe key storage strategy
   - concurrency support
   - request/response structure
   - message-to-key mapping
   - secure serialization formats (Base64, etc.)

Your answer must include:
- A clear workflow diagram (text-based is fine)
- A production-ready architecture
- Concrete, efficient implementation details suitable for real deployment
- Notes on ensuring the system remains quantum-resistant, fast, and scalable

Now analyze and give the best possible solution and implementation plan.
