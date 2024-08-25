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
      if (!loading && !user) {
        router.push("/login");
        return;
      }

      if (!loading && user) {
        setIsLoading(true); // Fetchingが始まったのでローディング状態を開始する

        try {
          const response = await axios.get("/api/todos", {
            headers: { Authorization: `Bearer ${user.token}` },
          });

          console.log(`response.data.todos: ${response.data.todos}`);

          if (response.data.todos && Array.isArray(response.data.todos)) {
            setTasks(response.data.todos);
          } else {
            setError("データ形式が正しくありません。");
          }
        } catch (err) {
          setError("ミッションの取得に失敗しました。");
        } finally {
          setIsLoading(false); // Fetchingが終了したのでローディング状態を終了する
        }
      }
    };

    fetchTasks();
  }, [loading, user, router]);

  // TODO追加
  const addTask = async (title: string, description: string) => {
    console.log("Called addTask");

    if (!loading && !user) {
      router.push("/login");
      return;
    }

    const reqBody = { title, description };
    if (user) {
      try {
        const response = await axios.post("/api/todo", reqBody, {
          headers: {
            Authorization: `Bearer ${user.token}`,
            "Content-Type": "application/json",
          },
        });

        console.log(`response: ${JSON.stringify(response)}`);

        setTasks((tasks) => [...tasks, response.data]);
      } catch (err) {
        setError("ミッションの追加に失敗しました。");
      }
    }
  };

  // TODO更新
  const updateTask = async (updatedTask: TaskParam) => {
    console.log("Called updateTask");

    if (!loading && !user) {
      router.push("/login");
      return;
    }

    if (user) {
      try {
        await axios.post("/api/todo", {
          headers: {
            Authorization: `Bearer ${user.token}`,
            "Content-Type": "application/json",
          },
          updatedTask,
        });
        setTasks((tasks) =>
          tasks.map((task) => (task.id === updatedTask.id ? updatedTask : task))
        );
      } catch (err) {
        setError("ミッションの更新に失敗しました。");
      }
    }
  };

  // TODO削除
  const deleteTask = async (taskId: number) => {
    console.log("Called deleteTask");

    if (!loading && !user) {
      router.push("/login");
      return;
    }

    if (user) {
      try {
        await axios.delete("/api/todo", {
          headers: {
            Authorization: `Bearer ${user.token}`,
            "Content-Type": "application/json",
          },
          data: { id: taskId },
        });
        setTasks((tasks) => tasks.filter((task) => task.id !== taskId));
      } catch (err) {
        setError("ミッションの削除に失敗しました。");
      }
    }
  };

  // TODOステータス更新
  const changeTaskStatus = async (taskId: number) => {
    console.log("Called changeTaskStatus");

    if (!loading && !user) {
      router.push("/login");
      return;
    }

    if (user) {
      try {
        await axios.post("/api/todo/complete", {
          headers: {
            Authorization: `Bearer ${user.token}`,
            "Content-Type": "application/json",
          },
          id: taskId,
        });
        setTasks((tasks) =>
          tasks.map((task) =>
            task.id === taskId ? { ...task, is_completed: true } : task
          )
        );
      } catch (err) {
        setError("ミッションの完了に失敗しました。");
      }
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
  // ミッションを選択し、キーボードのEnterを入力すると、カーソルボタンで↑↓で操作できる
  // useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates })
  const sensors = useSensors(useSensor(PointerSensor), useSensor(TouchSensor));

  return (
    <div className="w-full h-full flex flex-col items-center gap-12 mt-2.5">
      <h1>今日のミッション</h1>
      <DndContext
        sensors={sensors}
        collisionDetection={closestCorners}
        onDragEnd={handleDragEnd}
      >
        <Input onSubmit={addTask} />
        <Column tasks={tasks} />
      </DndContext>
    </div>
  );
};

export default Gamission;
