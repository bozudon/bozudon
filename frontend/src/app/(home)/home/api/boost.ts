'use server';

const SERVER_URL = process.env.SERVER_URL;

export type BoostProps = {
  token: string;
  postId: string;
  visibility?: 'public' | 'unlisted' | 'private';
};

export const postBoost = async ({
  token,
  postId,
  visibility = 'public',
}: BoostProps): Promise<boolean> => {
  const endpoint = `${SERVER_URL}/api/v1/statuses/${postId}/reblog`;
  const options = {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      visibility,
    }),
  };

  try {
    const res = await fetch(endpoint, options);
    if (res && res.ok) {
      return true;
    } else {
      console.error('There was a problem with the fetch operation:', res);
      return false;
    }
  } catch (error) {
    console.error('There was a problem with the fetch operation:', error);
    return false;
  }
};

export type UnboostProps = {
  token: string;
  postId: string;
};

export const postUnboost = async ({
  token,
  postId,
}: UnboostProps): Promise<boolean> => {
  const endpoint = `${SERVER_URL}/api/v1/statuses/${postId}/unreblog`;
  const options = {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  };

  try {
    const res = await fetch(endpoint, options);
    if (res && res.ok) {
      return true;
    } else {
      console.error('There was a problem with the fetch operation:', res);
      return false;
    }
  } catch (error) {
    console.error('There was a problem with the fetch operation:', error);
    return false;
  }
};
