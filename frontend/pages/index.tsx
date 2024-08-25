"use client";

import Head from "next/head";
import styles from "@/styles/Home.module.css";
import { Inter } from "next/font/google";
import { useEffect } from "react";
import { useRouter } from "next/router";
import { useAuth } from "../context/AuthContext";

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
  const router = useRouter();
  const { user, loading } = useAuth();

  useEffect(() => {
    if (!loading && !user) {
      router.push("/login");
    }
  }, [user, loading, router]);

  const goToGamission = () => {
    router.push("/gamission");
  };

  return (
    <>
      <Head>
        <title>Gamernage みんなでゲーム練習</title>
        <meta name="description" content="Gamernage みんなゲーム練習" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={`${styles.main} ${inter.className}`}>
        <h1 className="text-2xl font-bold mb-4">Welcome to Gamernage</h1>
        <p className="text-lg mb-6">
          ログイン後にこのページにアクセスしています。
        </p>
        <button
          className="bg-blue-500 text-white rounded-md px-5 py-2 hover:bg-blue-700 transition-colors duration-300"
          type="button"
          onClick={goToGamission}
        >
          Gamission画面
        </button>
      </main>
    </>
  );
}
