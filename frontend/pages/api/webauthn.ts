import {
  startAuthentication,
  startRegistration,
} from "@simplewebauthn/browser";

// WebAuthn 登録
async function register() {
  const registrationOptions = await fetch(
    "/api/webauthn/register-options"
  ).then((res) => res.json());

  const attestationResponse = await startRegistration(registrationOptions);

  // サーバーに登録データを送信
  await fetch("/api/webauthn/register", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(attestationResponse),
  });
}

// WebAuthn 認証
async function authenticate() {
  const authenticationOptions = await fetch(
    "/api/webauthn/authenticate-options"
  ).then((res) => res.json());

  const assertionResponse = await startAuthentication(authenticationOptions);

  // サーバーに認証データを送信
  await fetch("/api/webauthn/authenticate", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(assertionResponse),
  });
}
