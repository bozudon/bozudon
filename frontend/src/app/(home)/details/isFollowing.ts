const SERVER_URL = process.env.SERVER_URL;

export const isFollowing = async (
  token: string,
  accountId: string
): Promise<boolean> => {
  const endpoint = `${SERVER_URL}/api/v1/accounts/relationships?id[]=${accountId}`;
  const options = {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  };
  const response = await fetch(endpoint, options);
  if (!response.ok) {
    console.error('Network response was not ok');
    return false;
  }

  const relationships = await response.json();
  if (relationships.length > 0) {
    return relationships[0].following;
  } else {
    console.error('Relationship not found');
    return false;
  }
};
