'use server';

import { cookies } from 'next/headers';

// cookieに保存するアクセストークンのキー
const TOKEN_KEY = 'access_token';

// Cookieからアクセストークンを取得する
export const getToken = () => {
  const cookieStore = cookies();
  const cookie = cookieStore.get(TOKEN_KEY);
  // アクセストークンが存在しない場合は空文字を返す
  const value = cookie ? cookie.value : '';

  return value;
};

// Login状態を確認する
export const isLoggedIn = () => {
  const cookieStore = cookies();
  const loggedIn = cookieStore.has(TOKEN_KEY);
  return loggedIn;
};
