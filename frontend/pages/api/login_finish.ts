import { NextApiRequest, NextApiResponse } from "next";

// Define the type for the credential response
interface Credential {
  id: string;
  type: string;
  rawId: string;
  response: {
    clientDataJSON: string;
    authenticatorData: string;
    signature: string;
    userHandle?: string;
  };
}

interface SuccessResponse {
  success: boolean;
}

interface ErrorResponse {
  error: string;
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<SuccessResponse | ErrorResponse>
) {
  // Ensure the request method is POST
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method Not Allowed" });
  }

  // Extract the credential from the request body
  const credential: Credential = req.body;

  // Basic validation of the credential (extend this based on your needs)
  if (
    !credential ||
    typeof credential !== "object" ||
    !credential.id ||
    !credential.type ||
    !credential.response
  ) {
    return res.status(400).json({ error: "Invalid credential" });
  }

  // Process the authentication credential
  // Validate the credential here
  // This could involve checking the signature, user handle, etc.

  // Respond with success
  res.status(200).json({ success: true });
}
