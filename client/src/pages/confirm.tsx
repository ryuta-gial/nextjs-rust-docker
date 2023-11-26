import { useRouter } from "next/router";
import Link from "next/link";

export default function Confirm() {
  const router = useRouter();

  // URLクエリからnameとageを取得し、型を確認
  const name = typeof router.query.name === "string" ? router.query.name : "";
  const age = typeof router.query.age === "string" ? router.query.age : "";

  const handleSubmit = async () => {
    try {
      // router.queryから取得したageを整数に変換
      const ageInt = parseInt(age, 10);
      const response = await fetch("http://127.0.0.1:8080/submit", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ name, age: ageInt }),
      });

      if (response.ok) {
        // レスポンスが成功した場合、成功ページに遷移
        router.push("/success");
      } else {
        // エラーハンドリング
        console.error("サーバーエラー:", response.statusText);
      }
    } catch (error) {
      // ネットワークエラーやリクエストの失敗をキャッチ
      console.error("リクエストエラー:", error);
    }
  };

  return (
    <div>
      <h1>確認画面</h1>
      <p>名前: {name}</p>
      <p>年齢: {age}</p>
      <button onClick={handleSubmit}>送信</button>
      <Link href="/">
        <button type="button">戻る</button>
      </Link>
    </div>
  );
}
