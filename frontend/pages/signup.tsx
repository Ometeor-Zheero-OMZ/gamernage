import Link from "next/link";
import React, { useState } from "react";

const Signup = () => {
  const [isFormVisible, setIsFormVisible] = useState(false);

  const handleLinkClick = (e: React.MouseEvent<HTMLAnchorElement>) => {
    e.preventDefault();
    setIsFormVisible(true);
  };

  const handleCloseForm = () => {
    setIsFormVisible(false);
  };

  return (
    <div>
      <Link
        id="signup-btn"
        className={`signup-form-wrapper w-[400px] bg-gradient-to-b from-[rgba(81,127,253,0.8)] to-[rgba(88,183,247,0.8)] bg-center bg-no-repeat bg-cover absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 rounded-[20px] transition-all duration-500 ${
          isFormVisible ? "opacity-100 visible" : "opacity-0 invisible"
        }`}
        href="#"
        onClick={handleLinkClick}
      >
        Get started
      </Link>

      <div className="signup-form-wrapper w-[400px] bg-gradient-to-b from-[rgba(81,127,253,0.8)] to-[rgba(88,183,247,0.8)] bg-center bg-no-repeat bg-cover absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 rounded-[20px] opacity-0 invisible transition-all duration-500">
        <div
          className="signup-x absolute right-[20px] text-[50px] text-[#222] cursor-pointer"
          onClick={handleCloseForm}
        >
          &times
        </div>
        <div className="text-center my-[80px] mb-[150px] text-white">
          <h1 className="font-josefin-sans text-[50px] font-light mb-[30px]">
            Sign up to
          </h1>
          <h3 className="font-montserrat text-[35px] font-light">
            Get a new account
          </h3>
        </div>
        <form className="flex flex-col items-center">
          <div className="mb-2.5 relative">
            <i className="fas fa-user absolute top-[16px] left-[17px] text-[18px] text-[#aaa]"></i>
            <input
              className="w-[250px] py-[15px] pl-[50px] pr-[15px] border-none outline-none rounded-[30px] bg-[#00000099] text-[#ddd] text-[16px] font-[Josefin Sans], sans-serif"
              type="text"
              placeholder="Username"
            />
          </div>
          <div className="mb-2.5 relative">
            <i className="far fa-envelope absolute top-[16px] left-[17px] text-[18px] text-[#aaa]"></i>
            <input
              className="w-[250px] py-[15px] pl-[50px] pr-[15px] border-none outline-none rounded-[30px] bg-[#00000099] text-[#ddd] text-[16px] font-[Josefin Sans], sans-serif"
              type="email"
              placeholder="Email"
            />
          </div>
          <div className="mb-2.5 relative">
            <i className="fas fa-key absolute top-[16px] left-[17px] text-[18px] text-[#aaa]"></i>
            <input
              className="w-[250px] py-[15px] pl-[50px] pr-[15px] border-none outline-none rounded-[30px] bg-[#00000099] text-[#ddd] text-[16px] font-[Josefin Sans], sans-serif"
              type="password"
              placeholder="Password"
            />
          </div>
          <button
            className="w-[310px] py-3 px-4 border-none outline-none rounded-[30px] bg-[#00b5ad] text-white font-[Josefin Sans] text-[16px] uppercase my-2.5 shadow-[0_5px_20px_rgba(0,0,0,0.4)] cursor-pointer transition-transform duration-300 hover:translate-y-[-2px] hover:shadow-[0_8px_25px_rgba(0,0,0,0.3)] active:translate-y-0"
            type="button"
          >
            Sign Up
          </button>
        </form>
      </div>
    </div>
  );
};

export default Signup;
