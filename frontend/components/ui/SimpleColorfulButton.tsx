import React from "react";

const ColorfulButton = ({
  title,
  icon,
  position,
  handleClick,
  otherClasses,
}: {
  title: string;
  icon?: React.ReactNode;
  position?: string;
  handleClick?: () => void;
  otherClasses?: string;
}) => {
  return (
    <button className="p-[3px] relative" onClick={handleClick}>
      <div className="absolute inset-0 bg-gradient-to-r from-indigo-500 to-purple-500 rounded-lg animate-animate-btn-left" />

      <div
        className={`px-10 py-3  bg-black rounded-[6px]  relative group transition duration-300 text-white hover:bg-transparent animate-animate-btn-left  ${otherClasses}`}
      >
        {position === "left" && icon}
        {title}
        {position === "right" && icon}
      </div>
    </button>
  );
};

export default ColorfulButton;
