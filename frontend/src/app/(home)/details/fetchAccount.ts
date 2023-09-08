import type { Account } from '@/app/types';

const SERVER_URL = process.env.SERVER_URL;

export const fetchAccount = async (
  token: string,
  accountId: string
): Promise<Account | undefined> => {
  const endpoint = `${SERVER_URL}/api/v1/accounts/${accountId}`;

  const options = {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  };

  try {
    const res = await fetch(endpoint, options);
    if (res && res.ok) {
      const data = await res.json();
      return data;
    } else {
      console.error('There was a problem with the fetch operation:', res);
      return undefined;
    }
  } catch (error) {
    console.error('There was a problem with the fetch operation:', error);
    return undefined;
  }
};
