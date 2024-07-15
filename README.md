# DAO Voting Program

This repository contains the code for a DAO voting program developed using Anchor. This program allows users to vote on proposals and displays the results. Optionally, it includes privacy voting using Zero-Knowledge (ZK) proofs or verifiable compute, and rewards points to users for participation.

## Features

- **Voting System**: Users can vote on proposals.
- **Result Display**: Display the voting results.
- **Optional Privacy Voting**: Use ZK proofs or verifiable compute for privacy.
- **Reward Points**: Reward users for voting participation.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- Node.js (v14 or higher)
- NPM or Yarn

### Steps

1. **Clone the repository:**

   ```bash
   git clone https://github.com/Dunsin-cyber/voting_dao.git
   cd voting_dao
   ```

2. **Install dependencies:**

   ```
   npm install # or yarn install

   ```

3. **Build the Anchor program:**

   ```
   anchor build

   ```

4. **Deploy the Anchor program:**

   ```
   anchor deploy

   ```

5. **Start the local Solana cluster:**

   ```
   solana-test-validator

   ```

6. **Run the client script:**

   ```
    npm run start

   ```
