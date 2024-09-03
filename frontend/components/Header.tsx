import { useState, useEffect } from "react";
import Link from "next/link";
import AuthenticationForm from "./AuthenticationForm";

const Header = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const [animateLines, setAnimateLines] = useState(false);
  const [initialAnimation, setInitialAnimation] = useState(true);

  const toggleMenu = () => {
    setIsMenuOpen(!isMenuOpen);
    setAnimateLines(true);
  };

  useEffect(() => {
    if (initialAnimation) {
      setAnimateLines(true);
      setInitialAnimation(false);
      const timer = setTimeout(() => {
        setAnimateLines(false);
      }, 1000);
      return () => clearTimeout(timer);
    }
  }, [initialAnimation]);

  return (
    <header className="box-border w-full h-[68px] bg-black relative">
      <div className="flex items-center justify-between max-w-[1024px] h-[70px] px-[0.8em] mx-auto text-[1.2em] animate-fadeIn">
        <div id="logo" className="w-[70px]">
          <Link
            href="/"
            className="text-[40px] no-underline font-bold text-white font-[DotGothic16]"
          >
            Ataria
          </Link>
        </div>
        <nav className="hidden 770:flex items-center">
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
            <AuthenticationForm />
          </div>
        </nav>
        <div
          className="770:hidden flex items-center cursor-pointer z-10"
          onClick={toggleMenu}
        >
          <div
            className={`relative w-6 h-6 flex flex-col justify-between items-center transition-transform duration-500 ${
              isMenuOpen ? "animate-hamburger-open" : "animate-hamburger-close"
            }`}
          >
            <div
              className={`w-6 h-0.5 bg-white transition-transform duration-500 ${
                animateLines ? "animate-line-slide" : "opacity-100"
              }`}
              style={{ animationDelay: "0s" }}
            />
            <div
              className={`w-6 h-0.5 bg-white transition-transform duration-500 ${
                animateLines ? "animate-line-slide" : "opacity-100"
              }`}
              style={{ animationDelay: "0.2s" }}
            />
            <div
              className={`w-6 h-0.5 bg-white transition-transform duration-500 ${
                animateLines ? "animate-line-slide" : "opacity-100"
              }`}
              style={{ animationDelay: "0.4s" }}
            />
          </div>
        </div>
      </div>
      {isMenuOpen && (
        <>
          <div
            className="fixed inset-0 bg-black bg-opacity-70 z-40 animate-fadeIn"
            onClick={toggleMenu}
          />
          <div className="fixed top-[0px] left-0 w-full bg-black text-white p-10 z-50 animate-fadeIn">
            <ul className="list-none flex flex-col">
              <li className="py-2 hover:bg-[rgba(87, 87, 87, 0.8)]">
                <Link
                  href="/features"
                  className="block py-2 text-sm no-underline uppercase hover:bg-[#575757] hover:bg-opacity-80"
                >
                  Features
                </Link>
              </li>
              <li className="py-2 hover:bg-[rgba(87, 87, 87, 0.8)]">
                <Link
                  href="#"
                  className="block py-2 text-sm no-underline uppercase hover:bg-[#575757] hover:bg-opacity-80"
                >
                  Blog
                </Link>
              </li>
              <li className="py-2 hover:bg-[rgba(87, 87, 87, 0.8)]">
                <Link
                  href="#"
                  className="block py-2 text-sm no-underline uppercase hover:bg-[#575757] hover:bg-opacity-80"
                >
                  Community
                </Link>
              </li>
              <li className="py-2 hover:bg-[rgba(87, 87, 87, 0.8)]">
                <Link
                  href="#"
                  className="block py-2 text-sm no-underline uppercase hover:bg-[#575757] hover:bg-opacity-80"
                >
                  Download
                </Link>
              </li>
              <li className="pt-10">
                <AuthenticationForm />
              </li>
            </ul>
          </div>
        </>
      )}
    </header>
  );
};

export default Header;
