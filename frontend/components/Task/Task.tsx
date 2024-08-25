import { TaskParam } from "../../constants/type";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import React from "react";

type TaskProps = {
  task: TaskParam;
  // onUpdateTask: (updatedTask: TaskParam) => void;
  // onDeleteTask: (taskId: number) => void;
  // onCompleteTask: (taskId: number) => void;
};

const Task: React.FC<TaskProps> = ({ task }) => {
  const id = task.id;
  const title = task.title;
  const description = task.description;

  const { attributes, listeners, setNodeRef, transform, transition } =
    useSortable({ id });

  const style = {
    transition,
    transform: CSS.Transform.toString(transform),
  };

  return (
    <div
      ref={setNodeRef}
      {...attributes}
      {...listeners}
      style={style}
      className="bg-white rounded-md shadow-md w-full p-5 flex items-center justify-start gap-5 touch-none"
    >
      <input type="checkbox" className="h-5 w-5" />
      {title}
    </div>
  );
};

export default Task;
