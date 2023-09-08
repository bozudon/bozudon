'use client';

import List from '@mui/joy/List';
import ListItem from '@mui/joy/ListItem';
import ListItemDecorator from '@mui/joy/ListItemDecorator';
import ListItemButton from '@mui/joy/ListItemButton';

import Home from '@mui/icons-material/Home';
import NotificationsNoneIcon from '@mui/icons-material/NotificationsNone';
import CreateIcon from '@mui/icons-material/Create';
import FavoriteBorderIcon from '@mui/icons-material/FavoriteBorder';
import { Divider } from '@mui/material';

import Link from 'next/link';
import Image from 'next/image';
import { usePathname } from 'next/navigation';

export default function Navbar() {
  const pathName = usePathname();

  return (
    <List
      size="lg"
      variant="outlined"
      sx={{
        gap: 2,
        borderRadius: 'sm',
        boxShadow: 'sm',
      }}
    >
      <ListItem>
        <Image
          src="/bozudon_logo-full.png"
          alt="bozudon logo"
          width={200}
          height={60}
        />
      </ListItem>
      <Divider sx={{ marginLeft: 2, marginRight: 2, borderWidth: 1 }} />
      <Link href="/home">
        <ListItem>
          <ListItemButton selected={pathName === '/home'}>
            <ListItemDecorator>
              <Home />
            </ListItemDecorator>
            ホーム
          </ListItemButton>
        </ListItem>
      </Link>
      <Link href="/home">
        <ListItem>
          <ListItemButton selected={pathName === '/notification'}>
            <ListItemDecorator>
              <NotificationsNoneIcon />
            </ListItemDecorator>
            通知
          </ListItemButton>
        </ListItem>
      </Link>
      <Link href="/post">
        <ListItem>
          <ListItemButton selected={pathName === '/post'}>
            <ListItemDecorator>
              <CreateIcon />
            </ListItemDecorator>
            投稿
          </ListItemButton>
        </ListItem>
      </Link>
      <Link href="/favourites">
        <ListItem>
          <ListItemButton selected={pathName === '/favourites'}>
            <ListItemDecorator>
              <FavoriteBorderIcon />
            </ListItemDecorator>
            お気に入り
          </ListItemButton>
        </ListItem>
      </Link>
    </List>
  );
}
