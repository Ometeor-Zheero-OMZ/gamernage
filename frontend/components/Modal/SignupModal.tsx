"use client";

import { useAuth } from "../../context/AuthContext";
import { ChangeEvent, FormEvent, useState } from "react";
import React, { useRef } from "react";
import { ERROR_MESSAGES } from "@/constants/message";

type LoginModalProps = {
  isVisible: boolean;
  onClose: () => void;
};

const SignupModal: React.FC<LoginModalProps> = ({ isVisible, onClose }) => {
  const modalRef = useRef<HTMLDivElement>(null);
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isLoggingIn, setIsLoggingIn] = useState(false);
  const { signup } = useAuth();

  const handleOverlayClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (modalRef.current && !modalRef.current.contains(e.target as Node)) {
      onClose();
    }
  };

  const handleNameChange = (e: ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  const handleEmailChange = (e: ChangeEvent<HTMLInputElement>) => {
    setEmail(e.target.value);
  };

  const handlePasswordChange = (e: ChangeEvent<HTMLInputElement>) => {
    setPassword(e.target.value);
  };

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsLoggingIn(true);
    let isSuccess = await signup(name, email, password);
    setIsLoggingIn(false);

    if (isSuccess) {
      onClose();
      alert("Successfully signed up");
    } else {
      alert(ERROR_MESSAGES.LOGIN_FAILED_MESSAGE);
    }
  };

  return (
    <div
      className={`fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 transition-opacity duration-500 ${
        isVisible ? "opacity-100 visible" : "opacity-0 invisible"
      }`}
      onClick={handleOverlayClick}
    >
      <div
        ref={modalRef}
        className={`w-[400px] bg-gradient-to-b from-[rgba(98,81,253,0.8)] to-[rgba(88,144,247,0.8)] rounded-[20px] p-8 relative transition-transform duration-500 transform ${
          isVisible ? "translate-y-0" : "translate-y-4"
        }`}
      >
        <button
          className="absolute top-4 right-4 text-2xl text-white"
          onClick={onClose}
        >
          &times;
        </button>
        <div className="text-center my-8 text-white">
          <h1 className="font-josefin-sans text-2xl font-light mb-4">
            Sign into
          </h1>
          <h3 className="font-montserrat text-xl font-light">Your Ataria ID</h3>
        </div>

        {/* Form */}
        <form
          className="flex flex-col items-center"
          onSubmit={handleSubmit}
          method="post"
        >
          <div className="mb-4 relative">
            <i className="fas fa-user absolute top-4 left-4 text-lg text-[#aaa]"></i>
            <input
              className="w-[250px] py-3 pl-12 pr-4 border-none outline-none rounded-[30px] bg-[#00000099] text-[#ddd] text-lg"
              type="text"
              placeholder="Username"
              name="name"
              onChange={handleNameChange}
              required
            />
          </div>
          <div className="mb-4 relative">
            <i className="far fa-envelope absolute top-4 left-4 text-lg text-[#aaa]"></i>
            <input
              className="w-[250px] py-3 pl-12 pr-4 border-none outline-none rounded-[30px] bg-[#00000099] text-[#ddd] text-lg"
              type="text"
              placeholder="Email"
              name="email"
              onChange={handleEmailChange}
              required
            />
          </div>
          <div className="mb-4 relative">
            <i className="fas fa-key absolute top-4 left-4 text-lg text-[#aaa]"></i>
            <input
              className="w-[250px] py-3 pl-12 pr-4 border-none outline-none rounded-[30px] bg-[#00000099] text-[#ddd] text-lg"
              type="password"
              placeholder="Password"
              name="password"
              onChange={handlePasswordChange}
              disabled={isLoggingIn}
              required
            />
          </div>
          <button
            className="w-[310px] py-3 px-4 border-none outline-none rounded-[30px] bg-gradient-to-r from-indigo-500 to-purple-500 text-white text-lg uppercase my-2.5 shadow-[0_5px_20px_rgba(0,0,0,0.4)] cursor-pointer transition-transform duration-300 hover:translate-y-[-2px] hover:shadow-[0_8px_25px_rgba(0,0,0,0.3)] active:translate-y-0"
            type="submit"
          >
            Log In
          </button>
        </form>
      </div>
    </div>
  );
};

export default SignupModal;
