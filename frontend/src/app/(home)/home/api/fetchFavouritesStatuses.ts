'use server';

import type { Status } from '@/app/types';

const SERVER_URL = process.env.SERVER_URL;

export type Params = {
  [key: string]: string | number | undefined;
  max_id?: string;
  since_id?: string;
  min_id?: string;
  limit?: string; // Defaults: 20, Max: 40
};

export const fetchFavouriteStatuses = async (
  token: string,
  params?: Params
): Promise<Status[]> => {
  const endpoint = `${SERVER_URL}/api/v1/favourites`;
  const options = {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  };
  const url = new URL(endpoint);
  // paramsがある場合は、URLにクエリパラメータを追加する
  if (params) {
    Object.keys(params).forEach((key) => {
      const value = params[key];
      if (value !== undefined) {
        url.searchParams.append(key, value.toString());
      }
    });
  }

  try {
    const res = await fetch(endpoint, options);
    if (res && res.ok) {
      const data = await res.json();
      // お気に入りに追加されたステータスのリストが返される
      return data;
    } else {
      console.error('There was a problem with the fetch operation:', res);
    }
  } catch (error) {
    console.error('There was a problem with the fetch operation:', error);
  }
  return [];
};
