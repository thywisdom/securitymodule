💡 **TASK: FULL CODEBASE ANALYSIS + API CONVERSION**

You are an expert Rust engineer and cryptography specialist.
I have cloned the repository:

**[https://github.com/lattice-based-labs/ring-lwe.git](https://github.com/lattice-based-labs/ring-lwe.git)**

Your job is to perform the following:

### **1. FULL PROJECT ANALYSIS**

* Read and analyze the entire repository.
* Summarize the architecture and how the Ring-LWE scheme is implemented.
* Identify:

  * key generation functions
  * encryption function(s)
  * decryption function(s)
  * polynomial operations
  * noise sampling mechanism
  * parameter sets
* Explain how the library ensures correctness and security (in simple terms).

### **2. DESIGN A SECURE, ISOLATED API SERVICE**

Transform this Rust project into a standalone microservice:

#### The service should expose these REST API endpoints:

| Endpoint   | Method | Description                                       |
| ---------- | ------ | ------------------------------------------------- |
| `/health`  | GET    | Returns “OK”                                      |
| `/keygen`  | POST   | Returns publicKey + secretKey                     |
| `/encrypt` | POST   | Input: message + publicKey → Output: ciphertext   |
| `/decrypt` | POST   | Input: ciphertext + secretKey → Output: plaintext |

### **3. DEVELOP THE APPLICATION STRUCTURE**

Generate a full plan for turning this into a production-ready API:

* Decide on framework (Axum preferred, otherwise Actix/Web)
* Directory structure
* Modules:

  * `crypto/` → Ring-LWE core functions
  * `api/` → handlers for endpoints
  * `models/` → request/response DTOs
  * `service/` → encryption service logic
* Error-handling design
* Serialization format (JSON + Base64 for ciphertext)

### **4. IMPLEMENTATION**

Write the full implementation, including:

* `Cargo.toml` (with dependencies)
* `main.rs`
* `routes.rs`
* `handlers/*.rs`
* `crypto/*.rs`
* `models/*.rs`
* And any helper modules.

### **5. SECURITY GUIDELINES**

Apply:

* Zero-copy optimizations where safe
* Avoid serialization leaks
* Explain limitations of this simple Ring-LWE implementation
* Recommend protections for production use

### **6. DOCKER SUPPORT**

Generate:

* A production-ready `Dockerfile`
* A multi-stage build to minimize image size
* Instructions for running the service on any platform

### **7. OPTIONAL (IF CODEBASE IS OUTDATED)**

* Update code to Rust 2021/2024 edition
* Remove deprecated dependencies
* Improve module structure

### **8. FINAL OUTPUT**

Produce:

✔ A full explanation of the repo
✔ A detailed plan for turning it into an API service
✔ The actual Rust source code for the API
✔ Ready-to-deploy Dockerfile
✔ Commands for testing endpoints using curl or Postman
✔ A short security review

---