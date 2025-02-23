"use client";
import { useTasks } from "@/context/taskContext";
import useRedirect from "@/hooks/useUserRedirect";
import Filters from "./Components/Filters/Filters";
import TaskItem from "./Components/TaskItem/TaskItem";
import { Task } from "@/utils/types";
import { filteredTasks } from "@/utils/utilities";
import { useEffect } from "react";
import { motion } from "framer-motion";
import { container, item } from "@/utils/animations";
import { useUserContext } from "@/context/userContext";

export default function Home() {
  console.log("タスクページ");

  const { loggedIn, loading } = useUserContext();
  const { tasks, openModalForAdd, priority, setPriority } = useTasks();

  const filtered = filteredTasks(tasks, priority);

  if (loggedIn === false) {
    useRedirect("/login");
  }

  useEffect(() => {
    console.log("認証ページのuseEffect");
    setPriority("all");
  }, []);

  if (loading) {
    <div className="auth-page text-white text-7xl flex justify-center items-center">
      Loading...
    </div>;
  }

  return (
    <main className="m-6 h-full">
      <div className="flex justify-between">
        <h1 className="text-2xl text-[#EDEDED] font-bold">All Tasks</h1>
        <Filters />
      </div>

      <motion.div
        className="pb-[2rem] mt-6 grid grid-cols-[repeat(auto-fill,minmax(300px,1fr))] gap-[1.5rem]"
        variants={container}
        initial="hidden"
        animate="visible"
      >
        {filtered?.map((task: Task, i: number) => (
          <TaskItem key={i} task={task} />
        ))}
        <motion.button
          className="h-[16rem] w-full py-2 rounded-md text-lg font-medium text-gray-500 border-dashed border-2 border-gray-400
          hover:bg-gray-700 hover:text-[#EDEDED] hover:border-none transition duration-200 ease-in-out"
          onClick={openModalForAdd}
          variants={item}
        >
          Add New Task
        </motion.button>
      </motion.div>
    </main>
  );
}
