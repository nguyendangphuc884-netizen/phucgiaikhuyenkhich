# 🚀 StreamPay: Real-time Salary Streaming & RWA Yield 

## 📖 Project Overview
**StreamPay** is a Smart Contract built on **Soroban (Stellar Network)**. This project solves the problem of optimizing cash flow between Employers and Employees/Freelancers. 

Instead of traditional end-of-month payrolls, StreamPay allows salaries to "stream" continuously, second by second (Real-time Streaming). Simultaneously, the idle capital that hasn't been withdrawn yet automatically generates yield through Real World Assets (RWA) integration, providing a passive income stream for the Employer.

## ⚙️ Logic Flow
The system operates based on 4 core states:
1. **Initialization:** The Employer locks an amount of tokens (e.g., XLM) into the contract and sets the streaming duration.
2. **Tracking:** The system calculates the actual streamed balance in real-time based on the network's timestamp.
3. **Distribution:** The Employee/Freelancer can actively withdraw the streamed portion of their salary to their personal wallet at any time.
4. **Yield Generation:** When an employee withdraws funds, the contract automatically calculates the interest (APY) on the remaining locked balance and accrues it to the Employer.

## 🛠 Core Functions
* `create_stream`: Initializes the payment stream with `employer`, `employee`, `token`, `amount`, and `duration` parameters.
* `balance_of`: Queries the exact salary amount the employee is entitled to receive up to the current timestamp.
* `withdraw`: Executes the withdrawal of the available streamed salary to the employee's wallet.
* `claim_yield`: Allows the Employer to claim the RWA yield generated from the locked capital.

## 💻 CLI Interaction Guide
The project is currently deployed on the **Stellar Testnet**. 
* **Contract ID:** `[CBVYKNS43NJ2BERLUFIZKDV62ISQWUW3ARAZ2DZR2ZGMXVXDADFUZA26]`
* **Token ID (XLM Testnet):** `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`

<img width="2255" height="1503" alt="image" src="https://github.com/user-attachments/assets/ffe6c46e-c2a6-4a8c-937e-e664435041b5" />


### 1. Create a Stream
```bash
stellar contract invoke \
  --id [CONTRACT_ID] \
  --source [EMPLOYER_WALLET] \
  --network testnet \
  -- create_stream \
  --employer [EMPLOYER_WALLET] \
  --employee [EMPLOYEE_WALLET] \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --amount 1000000000 \
  --duration 600
