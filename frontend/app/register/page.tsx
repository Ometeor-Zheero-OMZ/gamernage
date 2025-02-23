"use client";
import React, { useEffect } from "react";
import RegisterForm from "../Components/auth/RegisterForm/RegisterForm";
import { useUserContext } from "@/context/userContext";
import { useRouter } from "next/navigation";
import useRedirect from "@/hooks/useUserRedirect";

function page() {
  console.log("新規登録ページ");

  useRedirect("/register");

  const { user, loading } = useUserContext();
  const router = useRouter();

  useEffect(() => {
    console.log("新規登録ページのuseEffect");

    if (loading) return;

    // redirect to home page if user is already logged in
    if (user && user.id) {
      router.push("/");
    }
  }, [user, loading, router]);

  // return null or a loading spinner/indicator
  if (user && user.id) {
    return null;
  }

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <div className="auth-page w-full h-full flex justify-center items-center">
      <RegisterForm />
    </div>
  );
}

export default page;
