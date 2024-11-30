jest.setTimeout(100000);

import * as anchor from '@coral-xyz/anchor'
import {Program, BN} from '@coral-xyz/anchor'
import {Keypair, PublicKey} from '@solana/web3.js'
import {TokenVesting} from '../target/types/token_vesting'
import {BankrunProvider} from 'anchor-bankrun'
import {BanksClient, ProgramTestContext, startAnchor, Clock} from 'solana-bankrun';
import {beforeAll} from "@jest/globals";
import {SYSTEM_PROGRAM_ID} from '@coral-xyz/anchor/dist/cjs/native/system'
import {TOKEN_PROGRAM_ID} from '@solana/spl-token'
import {createMint, getAccount, mintTo} from 'spl-token-bankrun';
import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet'


const IDL = require("../target/idl/token_vesting.json")

describe('tokenVesting', () => {
    // Configure the client to use the local cluster.
    const companyName = "OscarTech";
    let employer: Keypair;
    let beneficiary: Keypair;
    let vestingAccountKey: PublicKey;
    let treasuryTokenAccountKey: PublicKey;
    let employeeAccountKey: PublicKey;
    let employer_provider: BankrunProvider;
    let employer_program: Program<TokenVesting>;
    let employee_program: Program<TokenVesting>;
    let banksClient: BanksClient;
    let mint: PublicKey;
    let employee_Provider: BankrunProvider;
    let context: ProgramTestContext;


    beforeAll(async () => {
        beneficiary = new anchor.web3.Keypair();
        context = await startAnchor(
            "",
            [
                {
                    name: "token_vesting",
                    programId: new PublicKey(IDL.address)
                },
            ],
            [
                {
                    address: beneficiary.publicKey,
                    info: {
                        lamports: 1_000_000_000,
                        data: Buffer.alloc(0),
                        executable: false,
                        owner: SYSTEM_PROGRAM_ID
                    },
                },
            ]
        );
        banksClient = context.banksClient;

        employer_provider = new BankrunProvider(context);
        employer = employer_provider.wallet.payer;
        anchor.setProvider(employer_provider);
        employer_program = new Program<TokenVesting>(IDL as TokenVesting, employer_provider);

        mint = await createMint(banksClient, employer, employer.publicKey, null, 2);

        employee_Provider = new BankrunProvider(context);
        employee_Provider.wallet = new NodeWallet(beneficiary);
        employee_program = new Program<TokenVesting>(IDL as TokenVesting, employee_Provider);

        [vestingAccountKey] = PublicKey.findProgramAddressSync(
            [Buffer.from(companyName)],
            employee_program.programId
        )
        console.log("vestingAccountKey:", vestingAccountKey);

        [treasuryTokenAccountKey] = PublicKey.findProgramAddressSync(
            [Buffer.from("treasury_token_account"), Buffer.from(companyName)],
            employer_program.programId
        )
        console.log("treasuryTokenAccountKey:", treasuryTokenAccountKey);

        [employeeAccountKey] = PublicKey.findProgramAddressSync(
            [Buffer.from("employee_vesting"), beneficiary.publicKey.toBuffer(), vestingAccountKey.toBuffer()],
            employer_program.programId
        )
        console.log("employeeAcountKey:", employeeAccountKey);

    }, 10000);

    it("should create vestingAccount", async () => {
        const sig1 = await employer_program.methods
            .createVestingAccountOih(companyName)
            .accounts(
                {
                    signer: employer.publicKey,
                    mint: mint,
                    tokenProgram: TOKEN_PROGRAM_ID
                }
            ).rpc({commitment: "confirmed"});
        console.log("sig1:", sig1);
        const vestingAccountData = await employer_program.account.vestingAccount.fetch(vestingAccountKey, "confirmed");
        console.log(
            "Vesting Account Data:",
            JSON.stringify(vestingAccountData, null, 2)
        );
    }, 10000);

    it("should fund the treasury token account", async () => {
        const amount = 10_000 * 10 ** 9;
        ;
        const mintTX = await mintTo(
            banksClient,
            employer,
            mint,
            treasuryTokenAccountKey,
            employer,
            amount
        )

        console.log("mintTX:", mintTX);
    });

    it("should create employee account", async () => {
        const tx2 = await employer_program.methods
            .createEmployeeAccountOih(new BN(0), new BN(1000), new BN(0), new BN(100))
            .accounts({
                beneficiary: beneficiary.publicKey,
                vestingAccount: vestingAccountKey
            }).rpc({commitment: "confirmed", skipPreflight: true});

        console.log("tx2:", tx2);
        const employeeDataFetched = employer_program.account.employeeAccount.fetch(employeeAccountKey);
        console.log("employeeDataFetched:", employeeDataFetched);
    });

    it("should claim tokens", async () => {
        await new Promise((resolve) => setTimeout(resolve, 1000));
        const currentClock = await banksClient.getClock();
        context.setClock(
            new Clock(
                currentClock.slot,
                currentClock.epochStartTimestamp,
                currentClock.epoch,
                currentClock.leaderScheduleEpoch,
                BigInt(10000)
            ));

        const treasuryBeforeClaimToken = await getAccount(banksClient, treasuryTokenAccountKey);
        console.log("treasuryBeforeClaimToken:", treasuryBeforeClaimToken);

        const employeeAccountBeforeClaim = await employer_program.account.employeeAccount.fetch(employeeAccountKey);
        console.log("employeeAccountBeforeClaim:", employeeAccountBeforeClaim);

        const tx3 = await employee_program.methods
            .claimTokenOih(companyName)
            .accounts({tokenProgram: TOKEN_PROGRAM_ID})
            .rpc({commitment: "confirmed"});

        const treasuryAfterClaimToken = await getAccount(banksClient, treasuryTokenAccountKey);
        console.log("treasuryAfterClaimToken:", treasuryAfterClaimToken);

        const employeeAccountAfterClaim = await employer_program.account.employeeAccount.fetch(employeeAccountKey);
        console.log("employeeAccountAfterClaim:", employeeAccountAfterClaim);
        console.log("tx3:", tx3);
    });

})
