"use client";

import { useAuth } from "../context/AuthContext";
import { ChangeEvent, FormEvent, useState } from "react";
import { useRouter } from "next/navigation";
import styles from "@/styles/Login.module.css";

const LoginPage = () => {
  const router = useRouter();
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [isLoggingIn, setIsLoggingIn] = useState(false);
  const { login } = useAuth();

  const handleNameChange = (e: ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  const handlePasswordChange = (e: ChangeEvent<HTMLInputElement>) => {
    setPassword(e.target.value);
  };

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsLoggingIn(true);
    let isSuccess = await login(name, password);
    setIsLoggingIn(false);

    if (isSuccess) {
      router.push("/gamission");
    } else {
      alert("ログインに失敗しました。");
    }
  };

  return (
    <div>
      <form id={`${styles.loginForm}`} onSubmit={handleSubmit} method="post">
        <div className={`${styles.container}`}>
          <label htmlFor="name">
            <b>Username</b>
          </label>
          <input
            id="name"
            type="text"
            className={`${styles.formInput}`}
            placeholder="ユーザー名を入力しましょう！"
            name="name"
            onChange={handleNameChange}
            required
          />
          <label htmlFor="password">
            <b>Password</b>
          </label>
          <input
            id="password"
            type="password"
            className={`${styles.formInput}`}
            placeholder="パスワードを入力しましょう！"
            name="password"
            onChange={handlePasswordChange}
            disabled={isLoggingIn}
            required
          />
          <button className={`${styles.formButton}`} type="submit">
            Login
          </button>
        </div>
      </form>
    </div>
  );
};

export default LoginPage;
