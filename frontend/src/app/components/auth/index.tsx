import { redirect } from 'next/navigation';
import { isLoggedIn } from '@/app/utils/auth';
import { getToken } from '@/app/utils/auth';
import { AuthProvider } from './auth-provider';

type Props = {
  children: React.ReactNode;
};

// ログインが必要なページはAuthで囲ってください
export const Auth = ({ children }: Props) => {
  if (!isLoggedIn()) {
    redirect('/login');
  }

  const token = getToken();
  return <AuthProvider token={token}>{children}</AuthProvider>;
};
