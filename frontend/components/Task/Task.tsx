import "../../styles/Task.module.css";
import { TaskParam } from "../../constants/type";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import React from "react";

type TaskProps = {
  task: TaskParam;
  onUpdateTask: (updatedTask: TaskParam) => void;
  onDeleteTask: (taskId: number) => void;
  onCompleteTask: (taskId: number) => void;
};

const Task: React.FC<TaskProps> = ({
  task,
  onUpdateTask,
  onDeleteTask,
  onCompleteTask,
}) => {
  const handleComplete = () => {
    onCompleteTask(task.id);
  };

  const handleDelete = () => {
    onDeleteTask(task.id);
  };

  const handleUpdate = () => {
    const updatedTask = { ...task, title: "Updated Title" }; // Example update
    onUpdateTask(updatedTask);
  };

  return (
    <div className={`task ${task.is_completed ? "completed" : ""}`}>
      <div className="task-content">
        <p>{task.title}</p>
      </div>
      <button onClick={handleComplete}>Complete</button>
      <button onClick={handleUpdate}>Update</button>
      <button onClick={handleDelete}>Delete</button>
    </div>
  );
};

export default Task;
