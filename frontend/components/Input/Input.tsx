import "../../styles/Input.module.css";
import { InputProps } from "../../constants/type";
import React, { useState } from "react";

const Input: React.FC<InputProps> = ({ onSubmit }) => {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");

  const handleSubmit = () => {
    if (!title || !description) return;

    console.log("送信するパラメータ: ", title, description);
    onSubmit(title, description);

    setTitle("");
    setDescription("");
  };

  return (
    <div className="flex flex-col justify-center items-center bg-gray-500 border border-5">
      <div className="flex space-x-2">
        <input
          type="text"
          className="text-blue-400"
          placeholder="今日のミッションは何ですか？"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
        />
        <input
          type="text"
          className="text-blue-400"
          placeholder="ミッションの詳細を書きましょう！"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
        />
      </div>
      <button className="mt-2" onClick={handleSubmit}>
        ミッションを追加
      </button>
    </div>
  );
};

export default Input;
