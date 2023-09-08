'use client';
import { usePathname, useRouter } from 'next/navigation';
import styles from './header.module.css';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import { IconButton } from '@mui/material';
import HomeIcon from './components/homeIcon';
import PostIcon from './components/postIcon';
import FavouriteIcon from './components/favouriteIcon';

type CurrentPageTitleProps = {
  pathName: string;
};

const CurrentPageTitle = ({ pathName }: CurrentPageTitleProps) => {
  switch (pathName) {
    case '/home':
      return <HomeIcon title="ホーム" />;
    case '/post':
      return <PostIcon title="投稿" />;
    case '/favourites':
      return <FavouriteIcon title="お気に入り" />;
  }
};

export const Header = () => {
  const router = useRouter();
  const pathName = usePathname() as string;

  const goBack = (e: React.MouseEvent<HTMLElement>) => {
    e.preventDefault();
    router.back();
  };
  return (
    <header className={styles.container}>
      <IconButton
        className={styles.goBackContainer}
        aria-label="go back"
        onClick={goBack}
      >
        <ArrowBackIcon />
        <span className={styles.text}>back</span>
      </IconButton>
      <div aria-label="current page title" className={styles.titleContainer}>
        <CurrentPageTitle pathName={pathName} />
      </div>
    </header>
  );
};
