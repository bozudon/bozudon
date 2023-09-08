'use server';

import { getToken } from '@/app/utils/auth';

const SERVER_URL = process.env.SERVER_URL;

// ref: https://docs.joinmastodon.org/methods/statuses/
export type PostStatusProps = {
  status?: string;
  media_ids?: string[];
  poll?: {
    options?: string[];
    expires_in?: number;
    multiple?: boolean;
    hide_totals?: boolean;
  };
  in_reply_to_id?: string;
  sensitive?: boolean;
  spoiler_text?: string;
  visibility?: 'public' | 'unlisted' | 'private' | 'direct';
  language?: string;
  scheduled_at?: string;
};

export const postStatus = async (token: string, body: PostStatusProps) => {
  const url = `${SERVER_URL}/api/v1/statuses`;
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

export const getDisplayName = async () => {
  const token = getToken();
  const endpoint = `${SERVER_URL}/api/v1/accounts/verify_credentials`;
  const options = {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  };
  const response = await fetch(endpoint, options);
  if (!response.ok) {
    console.error('Network response was not ok');
    return 'test';
  }
  const data = await response.json();
  return data.display_name;
};
