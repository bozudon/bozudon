'use client';

import { createContext, useContext } from 'react';

type AuthContextType = {
  token: string;
};

// 初期値として null を設定
export const AuthContext = createContext<AuthContextType>({ token: '' });

export const useToken = () => useContext(AuthContext);
