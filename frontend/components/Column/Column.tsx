import "../../styles/Column.module.css";
import React from "react";
import {
  SortableContext,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { ColumnProps, TaskParam } from "../../constants/type";
import Task from "../Task/Task";

const Column: React.FC<ColumnProps> = ({
  tasks,
  onUpdateTask,
  onDeleteTask,
  onCompleteTask,
}) => {
  return (
    <div className="column">
      <SortableContext
        items={tasks.map((task) => task.id)}
        strategy={verticalListSortingStrategy}
      >
        {tasks.map((task: TaskParam) => (
          <Task
            key={task.id}
            task={task}
            onUpdateTask={onUpdateTask}
            onDeleteTask={onDeleteTask}
            onCompleteTask={onCompleteTask}
          />
        ))}
      </SortableContext>
    </div>
  );
};

export default Column;
