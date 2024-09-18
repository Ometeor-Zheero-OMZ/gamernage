import { useState, FormEvent } from "react";
import axios from "axios";

interface AuthenticatorSelectionCriteria {
  residentKey?: ResidentKeyRequirement;
  requireResidentKey?: boolean;
  userVerification?: UserVerificationRequirement;
}

interface ChallengeResponse {
  publicKey: {
    rp: {
      name: string;
      id: string;
    };
    user: {
      id: string;
      name: string;
      displayName: string;
    };
    challenge: string;
    pubKeyCredParams: Array<{
      type: "public-key";
      alg: number;
    }>;
    timeout: number;
    authenticatorSelection: AuthenticatorSelectionCriteria;
    attestation: AttestationConveyancePreference;
    extensions: {
      credentialProtectionPolicy: string;
      enforceCredentialProtectionPolicy: boolean;
      uvm: boolean;
      credProps: boolean;
    };
  };
}

interface PublicKeyCredentialCreationOptions {
  challenge: Uint8Array;
  rp: {
    name: string;
    id: string;
  };
  user: {
    id: Uint8Array;
    name: string;
    displayName: string;
  };
  pubKeyCredParams: Array<{
    type: "public-key";
    alg: number;
  }>;
  timeout?: number;
  authenticatorSelection?: AuthenticatorSelectionCriteria;
  attestation?: AttestationConveyancePreference;
  extensions?: {
    credentialProtectionPolicy?: string;
    enforceCredentialProtectionPolicy?: boolean;
    uvm?: boolean;
    credProps?: boolean;
  };
}

export default function Register() {
  const [username, setUsername] = useState<string>("");
  const [status, setStatus] = useState<string>("");

  const handleRegister = async (event: FormEvent) => {
    event.preventDefault();
    setStatus("Loading...");

    try {
      // Use axios to fetch the challenge
      const { data: challenge } = await axios.post<ChallengeResponse>(
        `/api/register_start/${username}`
      );

      // Convert the challenge to the format expected by the WebAuthn API
      const publicKey = challengeToPublicKey(challenge);
      console.log("Converted publicKey:", publicKey);

      // Request credential creation
      try {
        const credential = await navigator.credentials.create({ publicKey });
        console.log("Created credential:", credential);

        // Send the credential to the server for completion
        const { status: credentialStatus } = await axios.post(
          "/api/register_finish",
          credential
        );

        console.log("Credential status:", credentialStatus);

        if (credentialStatus === 200) {
          setStatus("Registration Successful!");
        } else {
          setStatus("Registration Failed");
        }
      } catch (createError) {
        console.error("Error creating credential:", createError);
        setStatus("Failed to create credential");
      }
    } catch (error) {
      console.error("An error occurred:", error);
      setStatus("An error occurred");
    }
  };

  const challengeToPublicKey = (
    challenge: ChallengeResponse
  ): PublicKeyCredentialCreationOptions => {
    console.log("challengeToPublicKey called");
    console.log(
      "challenge.publicKey.challenge = ",
      challenge.publicKey.challenge
    );
    console.log("challenge.publicKey.user = ", challenge.publicKey.user);

    const publicKey: PublicKeyCredentialCreationOptions = {
      challenge: Uint8Array.from(atob(challenge.publicKey.challenge), (c) =>
        c.charCodeAt(0)
      ),
      rp: {
        name: challenge.publicKey.rp.name,
        id: challenge.publicKey.rp.id,
      },
      user: {
        id: Uint8Array.from(atob(challenge.publicKey.user.id), (c) =>
          c.charCodeAt(0)
        ),
        name: challenge.publicKey.user.name,
        displayName: challenge.publicKey.user.displayName,
      },
      pubKeyCredParams: challenge.publicKey.pubKeyCredParams,
      timeout: challenge.publicKey.timeout,
      authenticatorSelection: {
        residentKey: challenge.publicKey.authenticatorSelection
          .residentKey as ResidentKeyRequirement,
        requireResidentKey:
          challenge.publicKey.authenticatorSelection.requireResidentKey,
        userVerification: challenge.publicKey.authenticatorSelection
          .userVerification as UserVerificationRequirement,
      },
      attestation: challenge.publicKey
        .attestation as AttestationConveyancePreference,
      extensions: challenge.publicKey.extensions,
    };

    return publicKey;
  };

  return (
    <div>
      <h1>Register</h1>
      <form onSubmit={handleRegister}>
        <input
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          placeholder="Enter username"
          required
        />
        <button type="submit">Start Registration</button>
      </form>
      <p>{status}</p>
    </div>
  );
}
