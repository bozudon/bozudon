'use client';

import styles from './card.module.css';
import MuiCard from '@mui/material/Card';
import CardHeader from '@mui/material/CardHeader';
import CardMedia from '@mui/material/CardMedia';
import CardContent from '@mui/material/CardContent';
import IconButton from '@mui/material/IconButton';
import MoreVertIcon from '@mui/icons-material/MoreVert';
import LoopIcon from '@mui/icons-material/Loop';
import ReplyIcon from '@mui/icons-material/Reply';
import Link from 'next/link';
import { useToken } from '@/app/components/auth/auth-context';
import { useState, useEffect } from 'react';
import { CardFooter } from './footer';
import { CardAvatar } from './avatar';
import { fetchStatus } from '@/app/api/status';
import type { Status } from '@/app/types';
import { useRouter } from 'next/navigation';
import { sanitize } from '@/app/utils/sanitizer';

type Props = {
  item: Status;
  // リプライボックスをクリックしたときに実行するハンドラー
  onReply: (id: string) => void;
  isOpen: boolean;
};

//  dateStringを次の形式に変換する 'YYYY-MM-DD HH:MM'
const formatDate = (date: string) => {
  const d = new Date(date);
  const formattedDate =
    d.getFullYear() +
    '-' +
    ('0' + (d.getMonth() + 1)).slice(-2) +
    '-' +
    ('0' + d.getDate()).slice(-2) +
    ' ' +
    ('0' + d.getHours()).slice(-2) +
    ':' +
    ('0' + d.getMinutes()).slice(-2);
  return formattedDate;
};

const ReblogMessage = ({ status }: { status: Status }) => {
  return (
    <div className={styles.actionContainer}>
      <LoopIcon className={styles.actionIcon} />
      <span className={styles.actionStatus}>
        {status.account.display_name} boosted
      </span>
    </div>
  );
};

// statusのidから投稿者のユーザー名を取得する
const ReplyMessage = ({ status }: { status: Status }) => {
  const { token } = useToken();
  const [displayName, setDisplayName] = useState('');

  useEffect(() => {
    const fetchData = async () => {
      const replyStatus = await fetchStatus(token, status.in_reply_to_id || '');
      if (!replyStatus) {
        // TODO: errorを表示させる
        return null;
      }
      setDisplayName(replyStatus.account.display_name);
    };

    fetchData();
  }, [token, status.in_reply_to_id]);

  return (
    <div className={styles.actionContainer}>
      <ReplyIcon className={styles.actionIcon} />
      <span className={styles.actionStatus}>In reply to {displayName}</span>
    </div>
  );
};

const Card = ({ item, onReply, isOpen }: Props) => {
  const isReblog = Boolean(item.reblog);
  const isReply = Boolean(item.in_reply_to_account_id);

  // reblogがある場合reblogにステータスに変更する
  let status = item;
  if (item.reblog) {
    status = item.reblog;
  }
  const account = status.account;
  const medias = status.media_attachments;
  const createdAt = formatDate(status.created_at);
  const router = useRouter();
  const handleClick = () => {
    router.push(`/status/${status.id}`);
  };

  const content = sanitize(status.content);
  const displayName = account.display_name;
  const imageMedia = medias.find((media) => media.type === 'image');
  const hasImage = !!imageMedia;
  const imageUrl = imageMedia?.url || '';

  return (
    <MuiCard sx={{ maxWidth: 800 }}>
      {isReblog && <ReblogMessage status={item} />}
      {isReply && <ReplyMessage status={item} />}
      <CardHeader
        avatar={
          <Link href={`/details/${account.id}`}>
            <button>
              <CardAvatar name={displayName} />
            </button>
          </Link>
        }
        action={
          <IconButton aria-label="settings">
            <MoreVertIcon />
          </IconButton>
        }
        title={displayName}
        subheader={createdAt}
      />
      {/* Cardの中央をクリックすると詳細画面に飛ばす */}
      <div onClick={handleClick} style={{ cursor: 'pointer' }}>
        <CardContent>
          {/* NOTE: mastodon の場合contentがタグに囲われているため、dangerouslySetInnerHTMLを使用している */}
          <div
            className={styles.contentText}
            dangerouslySetInnerHTML={{ __html: content }}
          />
          {hasImage && (
            <CardMedia component="img" height="150" image={imageUrl} alt="" />
          )}
        </CardContent>
      </div>
      <CardFooter
        status={status}
        onReply={() => onReply(status.id)}
        isOpen={isOpen}
      />
    </MuiCard>
  );
};

export default Card;
