'use server';

const SERVER_URL = process.env.SERVER_URL;

export type FavouriteProps = {
  token: string;
  postId: string;
};

export const postFavourite = async ({
  token,
  postId,
}: FavouriteProps): Promise<boolean> => {
  const endpoint = `${SERVER_URL}/api/v1/statuses/${postId}/favourite`;
  const options = {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  };

  try {
    const res = await fetch(endpoint, options);
    if (res && res.ok) {
      const data = await res.json();
      return true;
    } else {
      console.error('There was a problem with the fetch operation:', res);
    }
  } catch (error) {
    console.error('There was a problem with the fetch operation:', error);
  }
  return false;
};

export type UnfavouriteProps = {
  token: string;
  postId: string;
};

export const postUnfavourite = async ({
  token,
  postId,
}: UnfavouriteProps): Promise<boolean> => {
  const endpoint = `${SERVER_URL}/api/v1/statuses/${postId}/unfavourite`;
  const options = {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  };

  try {
    const res = await fetch(endpoint, options);
    if (res && res.ok) {
      const data = await res.json();
      return true;
    } else {
      console.error('There was a problem with the fetch operation:', res);
    }
  } catch (error) {
    console.error('There was a problem with the fetch operation:', error);
  }
  return false;
};
