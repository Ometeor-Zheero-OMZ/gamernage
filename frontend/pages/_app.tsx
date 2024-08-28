"use client";

import "../styles/globals.css";
import Link from "next/link";
import React, { useState } from "react";
import { AuthProvider } from "../context/AuthContext";
import { Inter } from "next/font/google";
import { AppProps } from "next/app";
import Head from "next/head";
import Image from "next/image";
import TwitterIcon from "../public/img/icon-twitter.svg";
import DiscordIcon from "../public/img/icon-discord.svg";
import YoutubeIcon from "../public/img/icon-youtube.svg";
import SteamIcon from "../public/img/icon-steam.svg";

const inter = Inter({ subsets: ["latin"] });

const App = ({ Component, pageProps }: AppProps) => {
  const [isFormVisible, setIsFormVisible] = useState(false);

  const handleLinkClick = (e: React.MouseEvent<HTMLAnchorElement>) => {
    e.preventDefault();
    setIsFormVisible(true);
  };

  const handleCloseForm = () => {
    setIsFormVisible(false);
  };

  return (
    <>
      <Head>
        <title>Ataria - Game Engine & Emulator</title>
        <meta
          name="description"
          content="Ataria is a game engine aimed for the versatiles who yearns to create, share, and play your own games"
        />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>

      <AuthProvider>
        {/* Header */}
        <header className="box-border w-full h-[68px] bg-black">
          <div className="flex items-center justify-between max-w-[1024px] h-[70px] px-[0.8em] mx-auto text-[1.2em]">
            <div id="logo" className="w-[70px]">
              <Link
                href="/"
                className="text-[40px] no-underline font-bold text-white font-[DotGothic16]"
              >
                Ataria
              </Link>
            </div>
            <nav className="flex items-center">
              <ul className="list-none flex">
                <li className="pr-[30px] hover:bg-[rgba(87, 87, 87, 0.8)]">
                  <Link
                    href="/features"
                    className="block w-full py-5 text-sm text-white no-underline uppercase hover:bg-[#575757] hover:bg-opacity-80"
                  >
                    Features
                  </Link>
                </li>
                <li className="pr-[30px] hover:bg-[rgba(87, 87, 87, 0.8)]">
                  <Link
                    href="#"
                    className="block w-full py-5 text-sm text-white no-underline uppercase hover:bg-[#575757] hover:bg-opacity-80"
                  >
                    Blog
                  </Link>
                </li>
                <li className="pr-[30px] hover:bg-[rgba(87, 87, 87, 0.8)]">
                  <Link
                    href="#"
                    className="block w-full py-5 text-sm text-white no-underline uppercase hover:bg-[#575757] hover:bg-opacity-80"
                  >
                    Community
                  </Link>
                </li>
                <li className="pr-[30px] hover:bg-[rgba(87, 87, 87, 0.8)]">
                  <Link
                    href="#"
                    className="block w-full py-5 text-sm text-white no-underline uppercase hover:bg-[#575757] hover:bg-opacity-80"
                  >
                    Download
                  </Link>
                </li>
              </ul>
              <div className="flex items-center ml-[16px]">
                <Link
                  id="signup-btn"
                  className="inline-block px-[1.2em] py-[0.5em] text-[15px] no-underline cursor-pointer select-none font-bold text-white bg-[#00b5ad] border border-[#00b5ad] rounded-sm transition-all duration-300 hover:text-[#00b5ad] hover:bg-black max-sm:hidden"
                  href="#"
                  onClick={handleLinkClick}
                >
                  Get started
                </Link>
              </div>
              <div className="flex items-center ml-[16px]">
                <Link
                  id="login-btn"
                  className="inline-block px-[1.2em] py-[0.5em] text-[15px] no-underline cursor-pointer select-none font-bold text-gray-400 hover:bg-[rgba(87,87,87,0.8)]"
                  href="#"
                  onClick={handleLinkClick}
                >
                  Login
                </Link>
              </div>
            </nav>
          </div>
        </header>

        {/* Form Container */}
        <div className="fixed bg-black bg-opacity-70 inset-0 opacity-0 invisible transition-all duration-500">
          <div
            className={`signup-form-wrapper w-[400px] bg-gradient-to-b from-[rgba(81,127,253,0.8)] to-[rgba(88,183,247,0.8)] bg-center bg-no-repeat bg-cover absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 rounded-[20px] transition-all duration-500 ${
              isFormVisible ? "opacity-100 visible" : "opacity-0 invisible"
            }`}
          >
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
          <div
            className={`login-form-wrapper modal w-[400px] bg-gradient-to-b from-[rgba(81,127,253,0.8)] to-[rgba(88,183,247,0.8)] bg-center bg-no-repeat bg-cover absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 rounded-[20px] transition-all duration-500 ${
              isFormVisible ? "opacity-100 visible" : "opacity-0 invisible"
            }`}
          >
            <div className="x-btn login-x" onClick={handleCloseForm}>
              &times
            </div>
            <div className="text-center my-[80px] mb-[150px] text-white">
              <h1 className="font-josefin-sans text-[50px] font-light mb-[30px]">
                Sign into
              </h1>
              <h3 className="font-montserrat text-[35px] font-light">
                Your Ataria ID
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
                Login
              </button>
            </form>
          </div>
        </div>

        <main className={inter.className}>
          <Component {...pageProps} />
        </main>

        {/* CTA Section */}
        <section id="cta" className="px-[96px] pb-[50px] bg-black">
          <div className="flex flex-col justify-center items-center p-2">
            <h5 className="mt-20 font-bold text-white text-5xl">
              Boost Your Creativity Now
            </h5>
            <div className="btns">
              <button
                id="signup-btn"
                className="box-border inline-block py-4 px-8 text-lg no-underline cursor-pointer select-none font-bold text-white bg-[#00b5ad] border border-[#00b5ad] rounded transition-all duration-300 hover:text-[#00b5ad] hover:bg-black"
              >
                Get Started
              </button>
            </div>
          </div>
        </section>

        {/* Footer */}
        <footer id="footer" className="px-[96px] pb-[50px] bg-black">
          <div className="flex flex-col items-center justify-between mt-14 mx-auto md:flex-row md:items-start md:mt-0">
            <div id="logo" className="w-[70px]">
              <Link
                href="#"
                className="text-[40px] no-underline font-bold text-white font-[DotGothic16]"
              >
                Ataria
              </Link>
            </div>

            {/* Menus Container */}
            <div className="flex flex-col mt-14 ml-0 md:flex-row md:mt-0 md:ml-20">
              {/* Menu 1 */}
              <div className="menu-items flex flex-col md:items-start md:mr-12">
                <div className="mb-[5px] font-bold text-[30px] text-white capitalize">
                  Features
                </div>
                <div className="flex flex-col items-start mt-[12px]">
                  <Link
                    href="#"
                    className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#00b5ad]"
                  >
                    Ataria Engine
                  </Link>
                  <Link
                    href="#"
                    className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#00b5ad]"
                  >
                    Ataria 2D Maker
                  </Link>
                  <Link
                    href="#"
                    className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#00b5ad]"
                  >
                    Ataria VR
                  </Link>
                </div>
              </div>
              {/* Menu 2 */}
              <div className="flex flex-col items-center w-full md:items-start md:mr-[50px]">
                <div className="mb-1.5 font-bold text-[30px] text-white capitalize">
                  Resources
                </div>
                <div className="flex flex-col items-start mt-[12px]">
                  <Link
                    href="#"
                    className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#00b5ad]"
                  >
                    Assets
                  </Link>
                  <Link
                    href="#"
                    className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#00b5ad]"
                  >
                    Gallery
                  </Link>
                  <Link
                    href="#"
                    className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#00b5ad]"
                  >
                    Support
                  </Link>
                </div>
              </div>
              {/* Menu 3 */}
              <div className="flex flex-col items-center w-full md:items-start md:mr-[50px]">
                <div className="mb-1.5 font-bold text-[30px] text-white capitalize">
                  Services
                </div>
                <div className="flex flex-col items-start mt-[12px]">
                  <Link
                    href="#"
                    className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#00b5ad]"
                  >
                    About
                  </Link>
                  <Link
                    href="#"
                    className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#00b5ad]"
                  >
                    FAQ
                  </Link>
                  <Link
                    href="#"
                    className="capitalize no-underline text-[#6c8df8] text-[20px] pb-[10px] hover:text-[#00b5ad]"
                  >
                    Contact
                  </Link>
                </div>
              </div>
            </div>
            {/* Social Container */}
            <div className="flex space-x-6">
              <Link href="#">
                <Image
                  src={TwitterIcon}
                  alt="twitter"
                  className="w-[40px] h-auto transition-[filter] duration-300 hover:filter hover:brightness-[89%] hover:contrast-[85%] hover:hue-rotate-[340deg] hover:invert-[76%] hover:saturate-[300%] hover:sepia-[61%]"
                  id="twitter"
                  width={10}
                  height={10}
                />
              </Link>
              <Link href="#">
                <Image
                  src={DiscordIcon}
                  alt="discord"
                  className="w-[40px] h-auto transition-[filter] duration-300 hover:filter hover:brightness-[89%] hover:contrast-[85%] hover:hue-rotate-[340deg] hover:invert-[76%] hover:saturate-[300%] hover:sepia-[61%]"
                  id="discord"
                  width={10}
                  height={10}
                />
              </Link>
              <Link href="#">
                <Image
                  src={YoutubeIcon}
                  alt="youtube"
                  className="w-[40px] h-auto transition-[filter] duration-300 hover:filter hover:brightness-[89%] hover:contrast-[85%] hover:hue-rotate-[340deg] hover:invert-[76%] hover:saturate-[300%] hover:sepia-[61%]"
                  id="youtube"
                  width={10}
                  height={10}
                />
              </Link>
              <Link href="#">
                <Image
                  src={SteamIcon}
                  alt="steam"
                  className="w-[40px] h-auto transition-[filter] duration-300 hover:filter hover:brightness-[89%] hover:contrast-[85%] hover:hue-rotate-[340deg] hover:invert-[76%] hover:saturate-[300%] hover:sepia-[61%]"
                  id="steam"
                  width={10}
                  height={10}
                />
              </Link>
            </div>
          </div>
          {/* Copyright */}
          <div className="flex justify-center items-center pt-[40px]">
            <h3 className="text-white">
              &copy;2024 <span className="copyright-title">Ataria</span>
            </h3>
          </div>
        </footer>
      </AuthProvider>
    </>
  );
};

export default App;
