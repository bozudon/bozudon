'use server';

import type { Status } from '@/app/types';

const SERVER_URL = process.env.SERVER_URL;

export const fetchStatus = async (
  token: string,
  statusId: string
): Promise<Status | undefined> => {
  const endpoint = `${SERVER_URL}/api/v1/statuses/${statusId}`;

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

type ContextResponse = {
  ancestors: Status[];
  descendants: Status[];
};

export const fetchParentAndChildStatuses = async (
  token: string,
  statusId: string
): Promise<ContextResponse> => {
  const endpoint = `${SERVER_URL}/api/v1/statuses/${statusId}/context`;

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
      console.log(data);
      return data;
    } else {
      console.error('There was a problem with the fetch operation:', res);
    }
  } catch (error) {
    console.error('There was a problem with the fetch operation:', error);
  }
  return {
    ancestors: [],
    descendants: [],
  };
};
