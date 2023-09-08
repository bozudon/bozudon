'use server';

import type { Account, Status } from '@/app/types';

const SERVER_URL = process.env.SERVER_URL;

export type SearchResultType = {
  statuses: Array<Status>;
  accounts: Array<Account>;
  hashtags: Array<{}>;
};

export async function getSearchResult(formData: FormData, token: string) {
  const query = String(formData.get('query'));
  const endpointBase = `${SERVER_URL}/api/v2/search`;
  const response = await fetch(endpointBase + '?q=' + query, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  if (!response.ok) {
    return undefined;
  }

  const data = await response.json();
  return data;
}
