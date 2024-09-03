import React, { useState } from "react";
import { useAuth } from "../context/AuthContext";
import SignupModal from "./Modal/SignupModal";
import LoginModal from "./Modal/LoginModal";
import CommonButton from "./Button/CommonButton";

const AuthenticationForm: React.FC = () => {
  const [isSignupVisible, setIsSignupVisible] = useState(false);
  const [isLoginVisible, setIsLoginVisible] = useState(false);

  const { user, loading, logout } = useAuth();

  const handleSignupClick = () => {
    setIsSignupVisible(true);
  };

  const handleLoginClick = () => {
    setIsLoginVisible(true);
  };

  const handleCloseSignup = () => {
    setIsSignupVisible(false);
  };

  const handleCloseLogin = () => {
    setIsLoginVisible(false);
  };

  return (
    <div>
      {user ? (
        <CommonButton onClick={logout}>ログアウト</CommonButton>
      ) : (
        <div>
          <div
            id="signup-btn"
            className="inline-block px-[1.2em] py-[0.5em] text-[15px] no-underline cursor-pointer select-none font-bold text-white bg-[#9d5df1] border border-[#9d5df1] rounded-sm transition-all duration-300 hover:text-[#9d5df1] hover:bg-black"
            onClick={handleSignupClick}
          >
            Get started
          </div>
          <div
            id="login-btn"
            className="inline-block px-[1.2em] py-[0.5em] text-[15px] no-underline cursor-pointer select-none font-bold text-gray-400 hover:bg-[rgba(87,87,87,0.8)]"
            onClick={handleLoginClick}
          >
            Login
          </div>
        </div>
      )}

      {/* モーダルを表示 */}
      <SignupModal isVisible={isSignupVisible} onClose={handleCloseSignup} />
      <LoginModal isVisible={isLoginVisible} onClose={handleCloseLogin} />
    </div>
  );
};

export default AuthenticationForm;
