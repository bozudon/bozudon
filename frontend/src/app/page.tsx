import { redirect } from 'next/navigation';
import { isLoggedIn } from './utils/auth';

const Root = () => {
  // ログインしていない場合はログイン画面にリダイレクトする
  if (!isLoggedIn()) {
    redirect('/login');
  }
  // ログインしている場合はHome画面にリダイレクトする
  redirect('/home');
};

export default Root;
