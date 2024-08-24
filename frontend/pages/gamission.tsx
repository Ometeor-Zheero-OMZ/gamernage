"use client";

import { useEffect, useState } from "react";
import {
  closestCorners,
  DndContext,
  DragEndEvent,
  KeyboardSensor,
  PointerSensor,
  TouchSensor,
  UniqueIdentifier,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import { arrayMove, sortableKeyboardCoordinates } from "@dnd-kit/sortable";
import axios from "axios";

import Column from "../components/Column/Column";
import Input from "../components/Input/Input";
import { TaskParam } from "../constants/type";
import { useRouter } from "next/router";
import { useAuth } from "@/context/AuthContext";

const Gamission = () => {
  const router = useRouter();
  const { user, loading } = useAuth();
  const [tasks, setTasks] = useState<TaskParam[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // TODO一覧取得
    const fetchTasks = async () => {
      console.log("Called fetchTasks");
      if (!loading && !user) {
        router.push("/login");
      } else if (!loading && user) {
        try {
          const response = await axios.get("/todos", {
            headers: { Authorization: `Bearer ${user.token}` },
          });

          if (response.data.todos && Array.isArray(response.data.todos)) {
            setTasks(response.data.todos);
          } else {
            setError("データ形式が正しくありません。");
          }
        } catch (err) {
          setError("タスクの取得に失敗しました。");
        } finally {
          setIsLoading(false);
        }
      }
    };

    fetchTasks();
  }, [loading, user, router]);

  // TODO追加
  const addTask = async (title: string, description: string) => {
    console.log("Called addTask");

    try {
      const response = await axios.post("/todo", { title, description });
      setTasks((tasks) => [...tasks, response.data]);
    } catch (err) {
      setError("タスクの追加に失敗しました。");
    }
  };

  // TODO更新
  const updateTask = async (updatedTask: TaskParam) => {
    console.log("Called updateTask");

    try {
      await axios.post("/todo", updatedTask);
      setTasks((tasks) =>
        tasks.map((task) => (task.id === updatedTask.id ? updatedTask : task))
      );
    } catch (err) {
      setError("タスクの更新に失敗しました。");
    }
  };

  // TODO削除
  const deleteTask = async (taskId: number) => {
    console.log("Called deleteTask");

    try {
      await axios.delete("/todo", { data: { id: taskId } });
      setTasks((tasks) => tasks.filter((task) => task.id !== taskId));
    } catch (err) {
      setError("タスクの削除に失敗しました。");
    }
  };

  // TODO完了
  const completeTask = async (taskId: number) => {
    console.log("Called completeTask");

    try {
      await axios.post("/todo/complete", { id: taskId });
      setTasks((tasks) =>
        tasks.map((task) =>
          task.id === taskId ? { ...task, is_completed: true } : task
        )
      );
    } catch (err) {
      setError("タスクの完了に失敗しました。");
    }
  };

  const getTaskPosition = (id: UniqueIdentifier) => {
    const numericId = typeof id === "string" ? Number(id) : id;
    return tasks.findIndex((task) => task.id === numericId);
  };

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;

    if (!over || active.id === over.id) return;

    setTasks((tasks) => {
      const originalPosition = getTaskPosition(active.id);
      const newPosition = getTaskPosition(over.id);

      return arrayMove(tasks, originalPosition, newPosition);
    });
  };

  // 現段階でUXの良さが感じられなかったため、下記引数を除外
  // タスクを選択し、キーボードのEnterを入力すると、カーソルボタンで↑↓で操作できる
  // useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates })
  const sensors = useSensors(useSensor(PointerSensor), useSensor(TouchSensor));

  return (
    <div className="Gamission">
      <h1>今日のミッション</h1>
      <div className="flex justify-center">
        {error && <p>Error: {error}</p>}
      </div>
      {isLoading ? (
        <p>Loading...</p>
      ) : (
        <DndContext
          sensors={sensors}
          collisionDetection={closestCorners}
          onDragEnd={handleDragEnd}
        >
          <Input onSubmit={addTask} />
          <Column
            tasks={tasks}
            onUpdateTask={updateTask}
            onDeleteTask={deleteTask}
            onCompleteTask={completeTask}
          />
        </DndContext>
      )}
    </div>
  );
};

export default Gamission;
