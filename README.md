# Turbin3 Q3 Sol Cohort Portfolio
![turbin3](https://github.com/user-attachments/assets/ea12e633-30fc-4efa-825f-3e11088d0b2c)

# Solana Notes
- [Account](#solana-account-model)

### Solana Account Model
On Solana, all data is stored in "accounts," similar to a key-value store where each entry is an account.
<img width="506" alt="Account" src="https://github.com/user-attachments/assets/48d44b17-dd2c-4cb9-a035-bb84eb42818a">



#### Key Points

- **Account Storage**: Up to 10MB of data, including executable code or program state.
- **Rent**: Accounts require a SOL deposit proportional to data size, refundable upon account closure.
- **Account Ownership**: Only the owning program can modify an account's data or reduce its lamport balance. Anyone can increase the balance.
<img width="528" alt="account-2" src="https://github.com/user-attachments/assets/57d7fa60-7585-47ef-bc9d-a7f7f4dccb06">

#### Account Structure
- **Address**: A unique 32-byte Ed25519 PublicKey.
  
> [!IMPORTANT🚨]
> The address of a normal account in Solana is a Base58-encoded string of a 256-bit ed25519 public key. Not all bit patterns are valid public keys for the ed25519 curve, so it is possible to ensure user-supplied account 
> addresses are at least correct ed25519 public keys.
- **Data**: Byte array storing account state or executable code.
- **Executable**: Flag indicating if the account is a program.
- **Lamports**: Account balance in lamports (1 SOL = 1 billion lamports).
- **Owner**: PublicKey of the owning program.

#### Types of Accounts
- **Program Accounts**: Stateless accounts storing executable code.
<img width="999" alt="program accounts" src="https://github.com/user-attachments/assets/510035c4-59f0-48c7-bb3d-21d89fe5ee89">

- **Data Accounts**: Store and manage program state, created by programs.
- **Native Programs**: Built-in programs with core functionalities, e.g., System Program and BPF Loader.
  <img width="528" alt="Native program" src="https://github.com/user-attachments/assets/09d9b139-0bbc-412c-b06d-fbad7f9571c9">

- **Sysvar Accounts**: Store network cluster state.

#### Key Programs
- **System Program**:
  <img width="564" alt="System program" src="https://github.com/user-attachments/assets/157b903f-0061-4065-b08d-f88885a70eef">

  - Creates new accounts.
  - Allocates data space.
  - Assigns program ownership.
  - Manages wallet accounts.

- **BPF Loader Program**: 
  - Deploys, upgrades, and executes custom programs.

#### Custom Programs (Smart Contracts)
- **Program Account**: Stores program's executable data and update authority.
- **Program Executable Data Account**: Contains executable byte code.
- **Buffer Account**: Temporary storage during deployment/upgrades.

#### Creating Data Accounts
1. **System Program**: Creates the account and transfers ownership.
2. **Custom Program**: Initializes account data.

## Turbin3 Prerequisites and Projects on Solana Devnet

This portfolio showcases my work and progress within the Turbin3 Q3 Sol Cohort program. Below are the detailed steps, scripts, and outcomes of my projects on the Solana blockchain.

## Table of Contents
1. [Transaction Links](#transaction-links)
2. [Project Setup](#project-setup)
3. [Scripts and Commands](#scripts-and-commands)
4. [File Descriptions](#file-descriptions)
5. [WBA-solana-starter](#wba-solana-starter)

### Transaction Links
| Description | Transaction Link |
|-------------|------------------|
| **Airdrop Transaction** | [View Transaction](https://explorer.solana.com/tx/NrVZjN9vFjy3pnaYYwbHG7bYa3r1Z79FrpbZQFL9cc3xTLJyMcehxKWHpnie9LpPE4t47KaLmwBF5uN2Yjmkcr9?cluster=devnet) |
| **Transfer Transaction** | [View Transaction](https://explorer.solana.com/tx/3h5U2bnddngzTv3UHATEVPdcnw14i29YFEzbsmrZBJyqD7JL8qV7SuYTKCS7Cx8FQ5hKt45SE6MsCvyq9sF7XaPK?cluster=devnet) |
| **Enroll Transaction** | [View Transaction](https://explorer.solana.com/tx/4pqinfBBVcmXK3cAMRRg7EK8jZZ5kFJWVrby65nbcvppxMiZRiFuiL8pLmdERUkuAgY6m2q1HLUS6aMZEkJC4yAJ?cluster=devnet) |

### Project Setup
1. **Environment Setup**:
   - Ensure Node.js and Yarn are installed.
   - Initialize project directory:
     ```sh
     mkdir airdrop && cd airdrop
     yarn init -y
     ```

2. **Package Installation**:
   - Install required packages:
     ```sh
     yarn add @types/node typescript @solana/web3.js bs58
     yarn add -D ts-node
     ```

3. **TypeScript Configuration**:
   - Generate TypeScript configuration:
     ```sh
     yarn tsc --init --rootDir ./ --outDir ./dist --esModuleInterop --lib ES2019 --module commonjs --resolveJsonModule true --noImplicitAny true
     ```

### Scripts and Commands

#### Creating Keypair
- Generate a new Solana wallet keypair:
  ```sh
  yarn keygen
  ```

#### Claiming Airdrop
- Request 2 SOL airdrop from Solana Devnet:
  ```sh
  yarn airdrop
  ```

#### Making a Transfer
- Transfer 0.1 SOL to another wallet:
  ```sh
  yarn transfer
  ```

#### Enrolling in Program
- Submit your GitHub account to the Solana program:
  ```sh
  yarn enroll
  ```

### File Descriptions

| File Name       | Description                                                                 |
|-----------------|-----------------------------------------------------------------------------|
| **keygen.ts**   | Script to generate a new Solana wallet keypair.                             |
| **airdrop.ts**  | Script to request SOL tokens from Solana Devnet to the generated wallet.    |
| **transfer.ts** | Script to transfer SOL from the generated wallet to another specified wallet. |
| **enroll.ts**   | Script to interact with the Solana program and submit your GitHub account.  |

# WBA-solana-starter
Turbin3 Qohort Homework-1 & Homework-2

### Case 1: Rug Day Task Completion
Participated in a collaborative task development session, creating and managing token accounts.

#### Commands & Tx Links
- **Create Mint Account**:
  ```bash
  ts-node ./cluster1/spl_init.ts
  Success! Your mint address is: 2sZ4ZHHTHeB3wSbNvnZeD27FLUrmMx8hzQa18EfyKjKX
  ```

- **Create Token Account**:
  ```bash
  ts-node ./cluster1/spl_mint.ts
  Your ata is: BAJaTUTE3eyyeQs7g4eBbhZBPtyG2aYYRX3yycWZnj7Z
  Success! Your mint transaction is: 3JSEffd5kXeKTXduzeVVRixNmdfeqEkqruX5LTpv1FqbBkJzyw84gCC96TsBvAU3EPBvCjFkd7tNpuMfkc9fcfjL
  ```

- **Mint Tokens**:
  ```bash
  Your ata is: BAJaTUTE3eyyeQs7g4eBbhZBPtyG2aYYRX3yycWZnj7Z
  Success! Your mint transaction is: 3TUxcfvZFseFyo4JndcprzLtJkPAREvEzoNqnCzX6j3uZ7MLyBgdUDiKRvP7sQsPyxEKatgywtaXJCPrJWvQS2y4
  ```

### Case 2: Presentation on Blinks
Watched a recorded presentation by Nick Frostbutter about Blinks.

### Case 3: Additional Tasks

#### Commands & Tx Links
- **Complete `spl_transfer`**:
  ```bash
  yarn spl_transfer
  yarn

 run v1.22.21
  $ ts-node ./cluster1/spl_transfer.ts
  Transaction signature: NzCn6uC7oNNWupmm4gLNtujncTEvQG9igodmfB2SK6ah2oz1oWjd7NooyoCPzmJ8ZnEsfeVwtL7siEjsj1CJpaT
  ✨  Done in 2.93s.
  ```

- **Complete `sql_metadata`**:
  ```bash
  ➜  ts git:(master) ✗ yarn nft_metadata
  yarn run v1.22.21
  $ ts-node ./cluster1/nft_metadata.ts
  Your image URI:  https://arweave.net/IgUpys1136O9Uf-A08Rc20_eP6_FWIAqx4TA3D2_DYI
  ```

- **Random Rugs**:
  [Generate Random Rugs](https://deanmlittle.github.io/generug/)

- **Initialized NFT**:
  ```bash
  yarn nft_mint
  yarn run v1.22.21
  $ ts-node ./cluster1/nft_mint.ts
  Succesfully Minted! Check out your TX here:
  https://explorer.solana.com/tx/3Cqz37kLDgzAtihx3gspHMiqvgqXPyT19t5wmWCWydUn8PCJt5N7KemWWB8KaeUcfTfFJotw735oqgJ48XihTDAS?cluster=devnet
  Mint Address:  4iDK7p4n3G5C66zhSCcSoVB9nTZfSUy3j88jdZpA8222
  ✨  Done in 3.28s.
  ```

- **RUG NFT Address**:
  [RUG NFT Address](https://explorer.solana.com/address/4iDK7p4n3G5C66zhSCcSoVB9nTZfSUy3j88jdZpA8222?cluster=devnet)

---
