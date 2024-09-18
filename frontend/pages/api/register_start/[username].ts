import { NextApiRequest, NextApiResponse } from "next";

// Define types for the registration challenge response
interface RegisterChallengeResponse {
  challenge: string;
  user: {
    id: string;
    name: string;
    displayName: string;
  };
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<RegisterChallengeResponse | { error: string }>
) {
  const { username } = req.query;

  // Ensure the request method is POST
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method Not Allowed" });
  }

  // Ensure username is a string
  if (typeof username !== "string") {
    return res.status(400).json({ error: "Invalid username" });
  }

  // Generate challenge here, e.g., using a WebAuthn library
  // For demonstration purposes, we'll use placeholder values
  const challenge = "base64url-encoded-challenge";
  const userId = "base64url-encoded-user-id";

  // Respond with the challenge
  res.status(200).json({
    challenge,
    user: {
      id: userId,
      name: username,
      displayName: username,
    },
  });
}
