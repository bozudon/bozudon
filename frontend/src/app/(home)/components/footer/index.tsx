import styles from './footer.module.css';

export default function Footer() {
  return (
    <footer className={styles.container}>
      <p>build ver.0.0.1</p>
      <hr className={styles.border} />
      <div>
        <a href="#">About us</a>・<a href="#">Privacy</a>・
        <a href="https://github.com">GitHub</a>
      </div>
    </footer>
  );
}
