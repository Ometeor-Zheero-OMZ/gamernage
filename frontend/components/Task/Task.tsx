import { TaskParam } from "../../constants/type";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import React, { useState } from "react";
import EditModal from "../Modal/EditModal";
import Image from "next/image";
import editIcon from "../../public/img/icon-edit.svg";

type TaskProps = {
  task: TaskParam;
  // onUpdateTask: (updatedTask: TaskParam) => void;
  // onDeleteTask: (taskId: number) => void;
  // onCompleteTask: (taskId: number) => void;
};

const Task: React.FC<TaskProps> = ({ task }) => {
  const id = task.id;
  const title = task.title;

  const { attributes, listeners, setNodeRef, transform, transition } =
    useSortable({ id });

  const style = {
    transition,
    transform: CSS.Transform.toString(transform),
  };

  const [isModalOpen, setIsModalOpen] = useState(false);

  const handleOpenModal = () => {
    console.log("called handleOpenModal");
    setIsModalOpen(true);
  };

  const handleCloseModal = () => {
    console.log("called handleCloseModal");
    setIsModalOpen(false);
  };

  const handleSave = (updatedTask: TaskParam) => {
    console.log("called handleSave");
    // ここでタスクの更新
  };

  return (
    <>
      <div
        ref={setNodeRef}
        {...attributes}
        {...listeners}
        style={style}
        className="bg-white rounded-md shadow-md w-full p-5 flex items-center justify-start gap-5 touch-none cursor-pointer"
      >
        <input type="checkbox" className="h-5 w-5" />
        {title}
        <div className="ml-auto">
          <Image
            src={editIcon}
            alt="Edit"
            width={20}
            height={20}
            onClick={handleOpenModal}
          />
        </div>
      </div>

      <EditModal
        task={task}
        isOpen={isModalOpen}
        onClose={handleCloseModal}
        onSave={handleSave}
      />
    </>
  );
};

export default Task;
