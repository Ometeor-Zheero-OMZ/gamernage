import OutlineButton from "@/components/ui/OutlineButton";
import SimpleColorfulButton from "@/components/ui/SimpleColorfulButton";
import { FaLocationArrow } from "react-icons/fa6";
import { useAuth } from "@/context/AuthContext";
import { useState } from "react";
import { useRouter } from "next/router";
import { ERROR_MESSAGES } from "@/constants/message";
import SignupModal from "@/components/Modal/SignupModal";

export default function Home() {
  const [isLoggingIn, setIsLoggingIn] = useState(false);
  const [isSignupVisible, setIsSignupVisible] = useState(false);
  const router = useRouter();
  const { guestLogin } = useAuth();

  const handleGuestLogin = async () => {
    setIsLoggingIn(true);
    let isSuccess = await guestLogin();
    setIsLoggingIn(false);

    if (isSuccess) {
      router.push("/index");
    } else {
      alert(ERROR_MESSAGES.LOGIN_FAILED_MESSAGE);
    }
  };

  const handleCloseSignup = () => {
    setIsSignupVisible(false);
  };

  const handleSignupClick = () => {
    setIsSignupVisible(true);
  };

  return (
    <>
      {/* Hero Container */}
      <div
        className="w-full h-screen bg-cover bg-center bg-no-repeat overflow-hidden relative"
        style={{
          backgroundImage:
            "linear-gradient(rgba(53, 82, 66, 0.5), rgba(47, 80, 63, 0.6)), url(/img/bg-nightsky.png)",
        }}
      >
        <div className="w-full absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 text-center">
          <h1 className="text-[80px] text-[#ddd] font-light mb-[30px] animate-moveToLeft font-dm">
            Harness{" "}
            <span className="text-white font-dotGothic16 font-normal uppercase text-center m-0 animate-shadows text-[calc(2rem+5vw)] tracking-[0.4rem]">
              Ataria
            </span>
          </h1>
          <p className="font-teko text-6xl font-bold text-light mb-20 animate-moveToRight tracking-wider sm:text-5xl md:text-5xl lg:text-6xl">
            Share Your Game Training Menus
          </p>
          <OutlineButton
            title="Guest Login"
            position="right"
            icon={<FaLocationArrow />}
            handleClick={handleGuestLogin}
          />
          <SimpleColorfulButton
            title="Get Started"
            handleClick={handleSignupClick}
          />
        </div>
      </div>

      {/* モーダルを表示 */}
      <SignupModal isVisible={isSignupVisible} onClose={handleCloseSignup} />
    </>
  );
}
