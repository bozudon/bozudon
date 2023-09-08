import type { Statuses } from '@/app/types';

const SERVER_URL = process.env.SERVER_URL;

export type Params = {
  [key: string]: string | number | boolean | undefined;
  max_id?: string;
  since_id?: string;
  min_id?: string;
  limit?: number;
  only_media?: boolean;
  exclude_replies?: boolean;
  exclude_reblogs?: boolean;
  pinned?: boolean;
  tagged?: string;
};

export const fetchAccountStatuses = async (
  accountId: string,
  token: string,
  params?: Params
): Promise<Statuses> => {
  const endpoint = `${SERVER_URL}/api/v1/accounts/${accountId}/statuses`;

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
    const res = await fetch(url.toString(), options);
    if (res && res.ok) {
      const data = await res.json();
      return data;
    } else {
      console.error('There was a problem with the fetch operation:', res);
      return [];
    }
  } catch (error) {
    console.error('There was a problem with the fetch operation:', error);
    return [];
  }
};
