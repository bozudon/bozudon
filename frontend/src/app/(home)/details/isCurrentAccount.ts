const SERVER_URL = process.env.SERVER_URL;
const endpoint = `${SERVER_URL}/api/v1/accounts/verify_credentials`;

export const isCurrentAccount = async (token: string, accountId: string) => {
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

  const account = await response.json();

  // userIDは文字列として比較します（APIから返されるIDは文字列の形式の場合があります）
  return account.id.toString() === accountId.toString();
};
