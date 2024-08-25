import "../styles/globals.css";
import { AuthProvider, useAuth } from "../context/AuthContext";
import { Inter } from "next/font/google";
import { AppProps } from "next/app";
import Head from "next/head";

const inter = Inter({ subsets: ["latin"] });

const App = ({ Component, pageProps }: AppProps) => {
  return (
    <>
      <Head>
        <title></title>
        <meta
          name="description"
          content="ゲーム練習に勤しむゲーマーのためのコミュニティサイト"
        />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        {/* <link rel="icon" href="/favicon.ico" /> */}
      </Head>
      <main className={inter.className}>
        <AuthProvider>
          <Component {...pageProps} />
        </AuthProvider>
      </main>
    </>
  );
};

export default App;
