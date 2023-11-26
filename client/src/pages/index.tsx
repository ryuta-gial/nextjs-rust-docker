import { useState } from "react";
import Link from "next/link";

export default function Home() {
  const [name, setName] = useState("");
  const [age, setAge] = useState("");

  return (
    <div>
      <h1>入力フォーム</h1>
      <form>
        <div>
          <label>名前: </label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <div>
          <label>年齢: </label>
          <input
            type="number"
            value={age}
            onChange={(e) => setAge(e.target.value)}
          />
        </div>
        <Link href={{ pathname: "/confirm", query: { name, age } }}>
          <button type="button">確認</button>
        </Link>
      </form>
    </div>
  );
}
