import { NextResponse } from 'next/server';
import { cookies } from 'next/headers';

const CLIENT_ID = process.env.CLIENT_ID;
const CLIENT_SECRET = process.env.CLIENT_SECRET;
const SERVER_URL = process.env.SERVER_URL;
const REDIRECT_URI = process.env.REDIRECT_URI;
const CLIENT_URL = process.env.CLIENT_URL;

// https://nextjs.org/docs/app/building-your-application/routing/router-handlers#dynamic-route-handlers
export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const code = searchParams.get('code');
  if (!code) {
    return NextResponse.json(
      { error: 'Missing authentication code.' },
      { status: 500 }
    );
  }

  try {
    const data = await getAccessToken(code);
    // アクセストークンをcookieに保存する
    cookies().set('access_token', data.access_token);
    // Homeにリダイレクトする
    if (!CLIENT_URL) {
      return NextResponse.json({
        message: 'Please set the CLIENT_URL in the environment variables',
        status: 500,
      });
    }
    // request.urlを使用するとdomainがおかしくなってしまうためフルパスで指定する
    return NextResponse.redirect(`${CLIENT_URL}/`);
  } catch (error) {
    if (error instanceof Error) {
      return NextResponse.json({ error: error.message }, { status: 500 });
    }
    return NextResponse.json(
      { error: 'Failed to get token.' },
      { status: 500 }
    );
  }
}

// tokenを取得する。
async function getAccessToken(code: string) {
  const endpoint = `${SERVER_URL}/oauth/token`;
  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      client_id: CLIENT_ID,
      client_secret: CLIENT_SECRET,
      redirect_uri: REDIRECT_URI,
      grant_type: 'authorization_code',
      code,
      scope: 'read write follow push',
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to get token: ${response.status}`);
  }

  const data = await response.json();
  return data;
}
