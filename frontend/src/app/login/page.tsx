import styles from './login.module.css';
import { isLoggedIn } from '@/app/utils/auth';
import { redirect } from 'next/navigation';
import { login } from './action';

const SERVER_URL = process.env.SERVER_URL;
const CLIENT_ID = process.env.CLIENT_ID;
const REDIRECT_URI = process.env.REDIRECT_URI;
const AUTH_URI = `${SERVER_URL}/oauth/authorize?client_id=${CLIENT_ID}&scope=read+write+follow+push&redirect_uri=${REDIRECT_URI}&response_type=code`;

const Login = () => {
  if (isLoggedIn()) {
    redirect('/home');
  }

  return (
    <div className={styles.loginContainer}>
      <form className={styles.loginForm} action={login}>
        <h2 className={styles.title}>Bozudon</h2>
        <a href={AUTH_URI} className={`${styles.loginButton} ${styles.oauth}`}>
          Oauth Login
        </a>
        <hr className={styles.line} />
        <div>
          <label>
            <span>Email</span>
            <input
              type="text"
              name="email"
              placeholder="your@example.com"
              required
              className={styles.input}
            />
          </label>
          <label>
            <span>Password</span>
            <input
              type="password"
              name="password"
              placeholder="Enter your password"
              required
              className={styles.input}
            />
          </label>
        </div>
        <button type="submit" className={styles.loginButton}>
          Login
        </button>
      </form>
    </div>
  );
};

export default Login;
