import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { StudentProfile } from "../target/types/student_profile";
import { getAccount, getAssociatedTokenAddress } from "@solana/spl-token";

describe("anchor-movie-review-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.StudentProfile as Program<StudentProfile>;

  const std = {
    name: "Harsh",
    about: "Harsh is a good student",
  };

  const [studentPda] = anchor.web3.PublicKey.findProgramAddressSync([provider.wallet.publicKey.toBuffer()], program.programId);

  const [mint] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("mint")], program.programId);

  let tokenAccount: anchor.web3.PublicKey;

  before(async () => {
    tokenAccount = await getAssociatedTokenAddress(mint, provider.wallet.publicKey);
  });

  it("Initializes the reward token", async () => {
    try {
      await program.methods.initializeTokenMint().rpc();
    } catch (e) {
      console.error("Error initializing token mint: ", e);
      throw e;
    }
  });

  it("Student is added", async () => {
    try {
      await program.methods
        .addStudentProfile(std.about, std.name)
        .accounts(tokenAccount)
        .rpc();

      const student = await program.account.studentProfile.fetch(studentPda);
      expect(student.name === std.name);
      expect(student.about === std.about);

      console.log(tokenAccount)
      const studentAta = await getAccount(provider.connection, tokenAccount);
      expect(Number(studentAta.amount)).to.equal((10 * 10) ^ 6);
    } catch (error) {
      console.error("Error adding student:", error);
      throw error;
    }
  });

  it("Student is updated`", async () => {
    const updatedName = "Harsh updated";
    const updatedAbout = "Harsh is a good student updated";

    const tx = await program.methods.updateStudentProfile(updatedAbout, updatedName).rpc();
    const student = await program.account.studentProfile.fetch(studentPda);
    expect(student.name === updatedName);
    expect(student.about === updatedAbout);
  });

  it("Deletes a student", async () => {
    const tx = await program.methods.deleteStudentProfile().rpc();
  });
});
