const SERVER_URL = process.env.NEXT_PUBLIC_SERVER_URL;

// NOTE: pngとjpgをサポートしている
export const uploadMedia = async (
  token: string,
  file: File,
  description: string
) => {
  const endpoint = `${SERVER_URL}/api/v2/media`;

  const formData = new FormData();
  formData.append('file', file);
  formData.append('description', description);

  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${token}`,
    },
    body: formData,
  });

  if (!response.ok) {
    return undefined;
  }

  const data = await response.json();
  return data.id;
};

export const postStatusWithMedia = async (
  token: string,
  statusText: string,
  mediaId: string
) => {
  const endpoint = `${SERVER_URL}/api/v1/statuses`;

  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      status: statusText,
      media_ids: [mediaId],
    }),
  });

  if (!response.ok) {
    throw new Error('Error in posting status with media');
  }

  const data = await response.json();

  return data;
};

export const postImageStatus = async (
  token: string,
  statusText: string,
  file: File,
  description?: string
) => {
  const mediaId = await uploadMedia(token, file, description || '');

  const statusResponse = await postStatusWithMedia(token, statusText, mediaId);

  return {
    status: statusResponse,
  };
};
