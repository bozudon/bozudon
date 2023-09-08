import { Home } from '@mui/icons-material';
import styles from './homeIcon.module.css';

export default function HomeIcon({ title }: { title: string }) {
  return (
    <div className={styles.container}>
      <Home />
      <h2>{title}</h2>
    </div>
  );
}
