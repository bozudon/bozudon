'use client';

import Image from 'next/image';
import { Timeline } from '@/app/(home)/home/components/timeline';
import { Button } from '@mui/material';
import { CardAvatar } from '@/app/(home)/home/components/card/avatar';
import styles from './account-profile.module.css';
import type { Statuses, Account } from '@/app/types';
import { doFollow, unFollow } from '../followHandlers';
import { useRouter } from 'next/navigation';
import { AlertCollapse, ErrorAlert } from '@/app/components/alert';
import { useEffect, useState } from 'react';
import { LoadingButton } from '@mui/lab';
import CardLoading from '../../components/skeletonUi/cardLoading';

type Props = {
  account: Account;
  statuses: Statuses;
  isCurrentAccount: boolean;
  isFollowing: boolean;
  token: string;
};

export const AccountProfile = ({
  account,
  statuses,
  isCurrentAccount,
  isFollowing,
  token,
}: Props) => {
  const router = useRouter();
  const [error, setError] = useState(''); // エラーメッセージ状態
  const [loading, setLoading] = useState<boolean>(true);
  const [isMounted, setIsMounted] = useState<boolean>(false);

  useEffect(() => {
    setIsMounted(true);
  }, []);

  useEffect(() => {
    setLoading(false);
  }, [statuses]);

  const follow = async (e: React.MouseEvent<HTMLElement>) => {
    e.preventDefault();
    setLoading(true);
    const followed = await doFollow(account.id, token, {});
    if (followed) {
      router.refresh();
    } else {
      setError('failed to follow.. try some time later');
    }
  };

  const stopFollow = async (e: React.MouseEvent<HTMLElement>) => {
    e.preventDefault();
    setLoading(true);
    const unFollowed = await unFollow(account.id, token);
    if (unFollowed) {
      router.refresh();
    } else {
      setError('failed to unfollow.. try some time later');
    }
  };

  if (!isMounted) {
    return <CardLoading />;
  }
  return (
    <>
      {/* エラー時にアラートを表示する */}
      <AlertCollapse show={error !== ''}>
        <ErrorAlert error={error} setError={setError} />
      </AlertCollapse>
      <div className={styles.userProfileContainer}>
        <AvatarWithBackground account={account} />
        {/* 本人の場合は設定ボタン */}
        {/* フォロー済みの場合はアンフォローボタン */}
        <div className={styles.buttonContainer}>
          {isCurrentAccount ? (
            <Button
              variant="contained"
              color="primary"
              size="large"
              className={styles.followButton}
            >
              Edit profile
            </Button>
          ) : (
            <LoadingButton
              loading={loading}
              variant="contained"
              color="primary"
              size="large"
              className={styles.followButton}
              onClick={isFollowing ? stopFollow : follow}
            >
              {isFollowing ? 'Unfollow' : 'Follow'}
            </LoadingButton>
          )}
        </div>
        <div className={styles.infoContainer}>
          <h2 className={styles.userName}>{account.display_name}</h2>
          <h3 className={styles.userAccountName}>@{account.acct}</h3>
          <div className={styles.userNoteContainer}>
            <p className={styles.userNote}>{account.note}</p>
          </div>
          <div className={styles.counts}>
            <p>
              {account.statuses_count}{' '}
              <span className={styles.infoType}>Posts</span>
            </p>
            <p>
              {account.following_count}{' '}
              <span className={styles.infoType}>Following</span>
            </p>
            <p>
              {account.followers_count}{' '}
              <span className={styles.infoType}>Followers</span>
            </p>
          </div>
          <div className={styles.timelineContainer}>
            <Timeline statuses={statuses} />
          </div>
        </div>
      </div>
    </>
  );
};

type AvatarWithBackgroundProps = {
  account: Account;
};

const AvatarWithBackground = ({ account }: AvatarWithBackgroundProps) => {
  // account.headerがない場合はデフォルトのヘッダーにする
  const header = '/bg-image.png';
  return (
    <div className={styles.box}>
      <Image
        src={header}
        alt="Cover"
        className={styles.coverImage}
        width={500}
        height={200}
      />
      <div className={styles.bottomLeft}>
        <CardAvatar name={account.display_name} width={120} height={120} />
      </div>
    </div>
  );
};
