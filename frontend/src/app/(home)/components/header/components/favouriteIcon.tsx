import FavoriteBorderIcon from '@mui/icons-material/FavoriteBorder';
import styles from './favouriteIcon.module.css';

export default function FavouriteIcon({ title }: { title: string }) {
  return (
    <div className={styles.container}>
      <FavoriteBorderIcon />
      <h2>{title}</h2>
    </div>
  );
}
