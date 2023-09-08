import CreateIcon from '@mui/icons-material/Create';
import styles from './postIcon.module.css';

export default function PostIcon({ title }: { title: string }) {
  return (
    <div className={styles.container}>
      <CreateIcon />
      <h2>{title}</h2>
    </div>
  );
}
