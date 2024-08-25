"use client";

import React, {
  createContext,
  useContext,
  useState,
  useEffect,
  ReactNode,
} from "react";

type User = {
  id: number;
  name: string;
  token: string;
};

type AuthContextType = {
  user: User | null;
  setUser: React.Dispatch<React.SetStateAction<User | null>>;
  loading: boolean;
  login: (name: string, password: string) => Promise<boolean>;
  logout: () => Promise<boolean>;
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  const login = async (name: string, password: string) => {
    try {
      const response = await fetch("/api/auth/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ name, password }),
      });

      if (!response.ok) {
        // Handle login failure
        console.error("Login failed");
        return false;
      }

      const userData = await response.json();
      userData.sub = userData.name;
      userData.id = userData.id;
      console.log(userData);

      // ローカルストレージにトークンを保存
      window.localStorage.setItem("login_token", userData.token);

      // ユーザー情報をアプリケーションの状態に設定
      setUser(userData);
      return true;
    } catch (error) {
      console.error("Login error:", error);
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
      const response = await fetch("/api/auth/current_user", {
        method: "GET",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (response.ok) {
        const user = await response.json();
        user.token = token;
        return user;
      } else {
        return false;
      }
    } catch (error) {
      console.error("Failed to fetch current user", error);
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
          console.log("Failed to load user");
          setUser(null);
        }
      } catch (error) {
        console.error("Failed to fetch current user", error);
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
      // Handle logout logic here
      setUser(null);
      return true; // logout successful
    } catch (error) {
      // Handle logout failure
      return false; // logout fail
    }
  };

  return (
    <AuthContext.Provider value={{ user, setUser, loading, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};
