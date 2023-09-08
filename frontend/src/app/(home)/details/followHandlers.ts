export type FetchFollowersParams = {
  [key: string]: string | number | undefined;
  max_id?: string;
  since_id?: string;
  min_id?: string;
  limit?: number;
};

export type DoFollowParams = {
  reblogs?: boolean;
  notify?: boolean;
  language?: string[];
};

export type DoFollowResult = {
  ok: boolean;
  id: string;
  following: boolean;
  showing_reblogs: boolean;
  notifying: boolean;
  languages: [];
  followed_by: string;
  blocking: boolean;
  blocked_by: boolean;
  muting: boolean;
  muting_notifications: boolean;
  requested: boolean;
  domain_blocking: boolean;
  endorsed: boolean;
  note: string;
};

const SERVER_URL = process.env.NEXT_PUBLIC_SERVER_URL; // server上では　PUBLIC_SERVER_URLで行けるはずなので、NEXT_PUBLIC消しておいた方が良さそうかな？

export const fetchFollowers = async (
  id: string,
  token: string,
  params?: FetchFollowersParams
) => {
  const options = {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  };
  const endpoint = `${SERVER_URL}/api/v1/accounts/${id}/followers`;
  const url = new URL(endpoint);
  // paramsがある場合、URLにクエリパラメータを追加
  if (params) {
    Object.keys(params).forEach((key) => {
      const value = params[key];
      if (value !== undefined) {
        url.searchParams.append(key, value.toString());
      }
    });
  }

  try {
    const res = await fetch(url, options);
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

export const doFollow = async (
  // TODO: return type Promiseの方法　要確認
  id: string,
  token: string,
  body: DoFollowParams
) => {
  // TODO: :Promise<DoFollowResult>のtypeどう扱うべきか考えるべき..
  const url = `${SERVER_URL}/api/v1/accounts/${id}/follow`;
  const options = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(body),
  };

  try {
    const res = await fetch(url, options);
    if (res && res.ok) {
      const data = await res.json();
      console.log(data);
      return true;
    } else {
      console.error('There was a problem with the fetch operation1:', res);
      return false;
    }
  } catch (error) {
    console.error('There was a problem with the fetch operation:', error);
    return false;
  }
};

export const unFollow = async (
  // TODO: return type Promiseの方法　要確認
  id: string,
  token: string
) => {
  const url = `${SERVER_URL}/api/v1/accounts/${id}/unfollow`;
  const options = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
  };

  try {
    const res = await fetch(url, options);
    if (res && res.ok) {
      const data = await res.json();
      console.log(data);
      return true;
    } else {
      console.error('There was a problem with the fetch operation1:', res);
      return false;
    }
  } catch (error) {
    console.error('There was a problem with the fetch operation:', error);
    return false;
  }
};
