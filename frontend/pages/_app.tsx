import "@/styles/globals.css";
import { AuthProvider, useAuth } from "../context/AuthContext";
import { AppProps } from "next/app";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <AuthProvider>
      <Component {...pageProps} />
    </AuthProvider>
  );
}
