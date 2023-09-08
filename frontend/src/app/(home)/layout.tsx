import styles from './layout.module.css';
import Navbar from './components/navbar';
import { Searchbar } from './components/searchbar';
import { Auth } from '../components/auth';
import { Header } from './components/header';
import Footer from './components/footer';

export default function HomeLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  // Authを使用して(home)配下のページはログイン限定にする
  return (
    <Auth>
      <main className={styles.container}>
        <section className={styles.left__container}>
          <Navbar />
        </section>
        <section className={styles.center__container}>
          <Header />
          {children}
        </section>
        <section className={styles.right__container}>
          <Searchbar />
          <Footer />
        </section>
      </main>
    </Auth>
  );
}
