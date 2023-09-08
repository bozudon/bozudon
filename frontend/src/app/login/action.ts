'use server';

import { redirect } from 'next/navigation';
import { cookies } from 'next/headers';

const SERVER_URL = process.env.SERVER_URL;
const CLIENT_ID = process.env.CLIENT_ID;
const CLIENT_SECRET = process.env.CLIENT_SECRET;

const endpoint = `${SERVER_URL}/api/ext/login`;

export async function login(formData: FormData) {
  const email = String(formData.get('email'));
  const password = String(formData.get('password'));

  const options = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      email: email,
      password: password,
      client_id: CLIENT_ID,
      client_secret: CLIENT_SECRET,
    }),
  };
  const response = await fetch(endpoint, options);
  if (response.ok) {
    const data = await response.json();
    cookies().set('access_token', data.access_token);
    redirect('/home');
  } else {
    console.error('Login failed');
    // TODO: アラートを表示する
  }
}
