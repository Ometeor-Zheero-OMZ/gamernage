import React from "react";
import Profile from "../Profile/Profile";
import RadialChart from "../RadialChart/RadialChart";
import { useUserContext } from "@/context/userContext";
import { signout } from "@/utils/Icons";

function Sidebar() {
  const { logoutUser } = useUserContext();
  return (
    <div className="w-[20rem] mt-[5rem] h-[calc(100%-5rem)] fixed right-0 top-0 bg-[#000D0D] flex flex-col">
      <Profile />
      <div className="mt-4 mx-6">
        <RadialChart />
      </div>

      <button
        className="flex items-center mt-8 ml-5 mr-5 border-[#000D0D] border-[0.1rem] hover:bg-[#E6E6E6]/20 hover:border-white hover:rounded-md"
        onClick={logoutUser}
      >
        <div className="text-gray-400 flex items-center gap-2 ml-2">
          <div className="text-2xl">{signout}</div>
          <p className="text-base text-white mb-[0.1rem]">Sign out</p>
        </div>
      </button>

      {/* <button
        className="mt-auto mb-6 mx-6 py-4 px-8 bg-[#EB4E31] text-white rounded-[50px] hover:bg-[#3aafae]/40 transition duration-200 ease-in-out"
        onClick={logoutUser}
      >
        Sign Out
      </button> */}
    </div>
  );
}

export default Sidebar;
