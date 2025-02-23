"use client";
import React, { useEffect } from "react";
import LoginForm from "../Components/auth/LoginForm/LoginForm";
import { useUserContext } from "@/context/userContext";
import { useRouter } from "next/navigation";
import useRedirect from "@/hooks/useUserRedirect";

function page() {
  console.log("認証ページ");

  const { user, loggedIn, loading, healthcheck } = useUserContext();
  const router = useRouter();

  console.log("[login] user = ", user);
  console.log("[login] user.id = ", user.id);
  console.log("[login] loggedIn = ", loggedIn);

  if (loggedIn === false) {
    useRedirect("/login");
  }

  useEffect(() => {
    console.log("ログインページのuseEffect");
    // redirect to home page if user is already logged in
    if (user && user.id) {
      router.push("/");
    }
  }, [user, router]);

  // return null or a loading spinner/indicator
  if (user && user.id) {
    return null;
  }

  if (loading) {
    <div className="auth-page text-white text-7xl flex justify-center items-center">
      Loading...
    </div>;
  }

  return (
    <div className="auth-page w-full h-full flex justify-center items-center">
      <div className="flex">
        <button
          onClick={healthcheck}
          className="mt-[1.5rem] flex-1 px-4 py-3 font-bold bg-[#2ECC71] text-white rounded-md hover:bg-[#1abc9c] transition-colors"
        >
          API Test
        </button>
      </div>
      <LoginForm />
    </div>
  );
}

export default page;
