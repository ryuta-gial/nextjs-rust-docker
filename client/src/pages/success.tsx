import Link from 'next/link';

export default function Success() {
  return (
    <div>
      <h1>完了画面</h1>
      <p>データの送信が完了しました。</p>
      <Link href="/">
        <button>ホームに戻る</button>
      </Link>
    </div>
  );
}
