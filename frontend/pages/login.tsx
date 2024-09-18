import { useState, FormEvent } from "react";
import axios from "axios";

// Define interfaces for the challenge and credential responses
interface ChallengeResponse {
  challenge: string;
  rpId: string;
}

interface PublicKeyCredentialRequestOptions {
  challenge: Uint8Array;
  rpId: string;
  userVerification: "required" | "preferred" | "discouraged";
}

export default function Login() {
  const [username, setUsername] = useState<string>("");
  const [status, setStatus] = useState<string>(""); // Loading, Success, Error

  const handleAuthenticate = async (event: FormEvent) => {
    event.preventDefault();
    setStatus("Loading...");

    try {
      // Use axios to fetch the challenge
      const { data: challenge } = await axios.post<ChallengeResponse>(
        `/api/login_start/${username}`
      );

      // Convert the challenge to the format expected by the WebAuthn API
      const publicKey = challengeToPublicKey(challenge);

      // Request credential
      const credential = await navigator.credentials.get({ publicKey });

      // Send the credential to the server for verification
      const { status: credentialStatus } = await axios.post(
        "/api/login_finish",
        credential
      );

      if (credentialStatus === 200) {
        setStatus("Authentication Successful!");
      } else {
        setStatus("Authentication Failed");
      }
    } catch (error) {
      setStatus("An error occurred");
    }
  };

  const challengeToPublicKey = (
    challenge: ChallengeResponse
  ): PublicKeyCredentialRequestOptions => {
    // Convert the challenge to the format expected by the WebAuthn API
    return {
      challenge: Uint8Array.from(atob(challenge.challenge), (c) =>
        c.charCodeAt(0)
      ),
      rpId: challenge.rpId,
      userVerification: "required",
    };
  };

  return (
    <div>
      <h1>Login</h1>
      <form onSubmit={handleAuthenticate}>
        <input
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          placeholder="Enter username"
          required
        />
        <button type="submit">Start Authentication</button>
      </form>
      <p>{status}</p>
    </div>
  );
}
