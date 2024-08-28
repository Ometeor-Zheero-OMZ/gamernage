import Head from "next/head";

export default function Home() {
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
          <h1 className="text-[80px] text-[#ddd] font-light mb-[30px] animate-moveToLeft">
            Harness{" "}
            <span className="text-shadow font-bungee text-center m-0 text-[calc(2rem+5vw)] uppercase font-bold text-white font-dotGothic">
              Ataria
            </span>
          </h1>
          <p className="text-5xl font-light text-text-light mb-20ox animate-moveToRight">
            Make your own 2D Action Game
          </p>
          <button
            type="button"
            className="btn bg-btn-left-bg text-btn-left-text animation-animate-btn-left hover:scale-120 w-[180px] p-3 m-5 border-2 border-[#6c8df8] rounded-full text-[18px] outline-none tracking-wide cursor-pointer"
          >
            Learn
          </button>
          <button
            type="button"
            className="btn bg-transparent text-btn-right-text animation-animate-btn-right hover:scale-120 w-[180px] p-3 m-5 border-2 border-[#6c8df8] rounded-full text-[18px] outline-none tracking-wide cursor-pointe"
          >
            Download
          </button>
        </div>
      </div>
    </>
  );
}
