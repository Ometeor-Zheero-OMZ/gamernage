"use client";

import "@/public/styles/globals.css";
import { Inter } from "next/font/google";
import { AppProps } from "next/app";
import { AuthProvider } from "@/context/AuthContext";

const inter = Inter({ subsets: ["latin"] });

const App = ({ Component, pageProps }: AppProps) => {
  return (
    <>
      <AuthProvider>
        <main className={inter.className}>
          <Component {...pageProps} />
        </main>
      </AuthProvider>
    </>
  );
};

export default App;
