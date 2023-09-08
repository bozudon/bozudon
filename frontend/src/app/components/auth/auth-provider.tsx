'use client';

import { AuthContext } from './auth-context';

export type Props = {
  token: string;
  children: React.ReactNode;
};

export const AuthProvider = ({ token, children }: Props) => {
  return (
    <AuthContext.Provider value={{ token }}>{children}</AuthContext.Provider>
  );
};
