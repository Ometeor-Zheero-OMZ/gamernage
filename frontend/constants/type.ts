// タスク
export type TaskParam = {
  id: number;
  user_id?: number;
  game_id?: number;
  title: string;
  description: string;
  is_completed: boolean;
  status?: number;
  priority?: number;
  difficulty?: number;
  deadline?: string;
  created_at?: string;
  updated_at?: string;
  deleted_at?: string;
};

// コラム　タスクとは1対多の関係
export type ColumnProps = {
  tasks: TaskParam[];
  // onUpdateTask: (updatedTask: TaskParam) => void;
  // onDeleteTask: (taskId: number) => void;
  // onCompleteTask: (taskId: number) => void;
};

// 入力フォーム
export type InputProps = {
  onSubmit: (title: string, description: string) => void;
};
