"use client";

import {
  DYNAMIC_ERROR_MESSAGES,
  ERROR_MESSAGES,
  PG_ERROR_MESSAGES,
} from "@/constants/message";
import {
  AuthContextType,
  LoginRequest,
  SignupRequest,
  User,
} from "@/types/type";
import axios from "axios";
import React, {
  createContext,
  useContext,
  useState,
  useEffect,
  ReactNode,
} from "react";

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  const signup = async (name: string, email: string, password: string) => {
    const signupRequest: SignupRequest = { name, email, password };

    try {
      const response = await axios.post("/api/auth/signup", signupRequest);

      if (response.status !== 200) {
        console.error("サインアップに失敗しました。");
        return false;
      }

      console.log("確認メールを送信しました。");

      // サインアップ成功後の処理
      return true;
    } catch (error) {
      console.error("サインアップエラー:", error);
      return false;
    }
  };

  const verifyEmail = async (token: string) => {
    try {
      const response = await axios.get(`/api/auth/verify_email?token=${token}`);

      if (response.data) {
        console.log("メールアドレスが確認されました。");
        // ログイン処理または認証後の画面に遷移
        return true;
      } else {
        console.error("メール確認に失敗しました。");
        return false;
      }
    } catch (error) {
      console.error("確認エラー:", error);
      return false;
    }
  };

  const login = async (name: string, password: string) => {
    const loginRequest: LoginRequest = { name, password };

    try {
      const response = await axios.post("/api/auth/login", loginRequest);

      const loginUserData = response.data;
      loginUserData.sub = loginUserData.name;
      loginUserData.id = loginUserData.id;
      console.log(loginUserData);

      // ローカルストレージにトークンを保存
      window.localStorage.setItem("login_token", loginUserData.token);

      // ユーザー情報をアプリケーションの状態に設定
      setUser(loginUserData);
      return true;
    } catch (error) {
      console.error(ERROR_MESSAGES.LOGIN_FAILED_MESSAGE);
      return false;
    }
  };

  const getCurrentUser = async (): Promise<User | false> => {
    try {
      const token = window.localStorage.getItem("login_token");
      if (token === "" || token === "undefined") {
        console.log(token);
        return false;
      }
      const response = await axios.get("/api/auth/current_user", {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      const currentUserData = response.data;
      currentUserData.token = token;
      return currentUserData;
    } catch (error: any) {
      console.error(
        DYNAMIC_ERROR_MESSAGES(error).FETCH_CURRENT_USER_FAILED_MESSAGE
      );
      return false;
    }
  };

  const guestLogin = async () => {
    // テストデータ
    const name = "test_user1";
    const password = "password";

    try {
      const response = await axios.post("/api/auth/guest_login", {
        name,
        password,
      });

      const loginUserData = response.data;
      loginUserData.sub = loginUserData.name;
      loginUserData.id = loginUserData.id;

      // ローカルストレージにトークンを保存
      window.localStorage.setItem("login_token", loginUserData.token);

      // ユーザー情報をアプリケーションの状態に設定
      setUser(loginUserData);
      return true;
    } catch (error) {
      console.error(ERROR_MESSAGES.LOGIN_FAILED_MESSAGE);
      return false;
    }
  };

  useEffect(() => {
    const loadUser = async () => {
      try {
        const user = await getCurrentUser();
        if (user) {
          setUser(user);
        } else {
          console.log(ERROR_MESSAGES.LOAD_USER_NOT_FOUND_MESSAGE);
          setUser(null);
        }
      } catch (error: any) {
        console.error(
          DYNAMIC_ERROR_MESSAGES(error).FETCH_CURRENT_USER_FAILED_MESSAGE
        );
        setUser(null);
      } finally {
        setLoading(false);
      }
    };

    loadUser();
  }, [setUser, setLoading]);

  const logout = async () => {
    try {
      localStorage.removeItem("login_token");
      setUser(null);
      return true;
    } catch (error) {
      return false;
    }
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        setUser,
        loading,
        signup,
        verifyEmail,
        login,
        logout,
        guestLogin,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error(PG_ERROR_MESSAGES.USEAUTH_REQUIRED_MESSAGE);
  }
  return context;
};
