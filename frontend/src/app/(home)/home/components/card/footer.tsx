'use client';

import { styled } from '@mui/material/styles';
import CardActions from '@mui/material/CardActions';
import Collapse from '@mui/material/Collapse';
import IconButton, { IconButtonProps } from '@mui/material/IconButton';
import FavoriteIcon from '@mui/icons-material/Favorite';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import ReplyIcon from '@mui/icons-material/Reply';
import LoopIcon from '@mui/icons-material/Loop';
import BookmarkIcon from '@mui/icons-material/Bookmark';
import { useState } from 'react';
import { Status } from '@/app/types';
import { pink, blue } from '@mui/material/colors';
import {
  postFavourite,
  postUnfavourite,
} from '@/app/(home)/home/api/favourite';
import type {
  FavouriteProps,
  UnfavouriteProps,
} from '@/app/(home)/home/api/favourite';
import { postBoost, postUnboost } from '@/app/(home)/home/api/boost';
import type { BoostProps, UnboostProps } from '@/app/(home)/home/api/boost';

import { useToken } from '@/app/components/auth/auth-context';
import { AlertCollapse, ErrorAlert } from '@/app/components/alert';
import Postbox from '@/app/(home)/post/components/postbox';
import { getDisplayName } from '@/app/(home)/post/handler';

interface ExpandMoreProps extends IconButtonProps {
  expand: boolean;
}

const ExpandMore = styled((props: ExpandMoreProps) => {
  const { expand, ...other } = props;
  return <IconButton {...other} />;
})(({ theme, expand }) => ({
  transform: !expand ? 'rotate(0deg)' : 'rotate(180deg)',
  marginLeft: 'auto',
  transition: theme.transitions.create('transform', {
    duration: theme.transitions.duration.shortest,
  }),
}));

type Props = {
  status: Status;
  // リプライボックスをクリックしたときに実行するハンドラー
  onReply: () => void;
  isOpen: boolean;
};

// リプ、いいね、リツイート
export const CardFooter = ({ status, onReply, isOpen }: Props) => {
  const { token } = useToken();
  const [expanded, setExpanded] = useState(false);
  const [displayName, setDisplayName] = useState<string>('');
  const [favourited, setFavourited] = useState(status.favourited);

  const handleClickReply = async () => {
    const name = await getDisplayName();
    setDisplayName(name);
    onReply();
  };
  const [boosted, setBoosted] = useState(status.reblogged);

  // アラートで表示するエラーメッセージ
  const [error, setError] = useState('');

  const handleExpandClick = () => {
    setExpanded(!expanded);
  };

  const favouritePost = async (props: FavouriteProps) => {
    const isSuccess = await postFavourite(props);
    if (isSuccess) {
      setFavourited(true);
    } else {
      setError('Failed to favourite the post.');
    }
  };

  const unfavouritePost = async (props: UnfavouriteProps) => {
    const isSuccess = await postUnfavourite(props);

    if (isSuccess) {
      setFavourited(false);
    } else {
      setError('Failed to unfavourite the post.');
    }
  };

  const boostPost = async (props: BoostProps) => {
    const isBoosted = await postBoost(props);
    if (isBoosted) {
      setBoosted(true);
    } else {
      setError('Failed to boost the post.');
    }
  };

  const unboostPost = async (props: UnboostProps) => {
    const isUnboosted = await postUnboost(props);
    if (isUnboosted) {
      setBoosted(false);
    } else {
      setError('Failed to boost the post.');
    }
  };

  return (
    <>
      {/* エラー時にアラートを表示する */}
      <AlertCollapse show={error !== ''}>
        <ErrorAlert error={error} setError={setError} />
      </AlertCollapse>
      <CardActions disableSpacing>
        <IconButton
          aria-label="reply"
          // クリックしたらリプライボックスを表示する
          onClick={handleClickReply}
          sx={{
            border: isOpen ? '1px solid rgba(0, 0, 0, 0.23)' : 'none',
          }}
        >
          <ReplyIcon />
        </IconButton>
        <IconButton
          aria-label="boost"
          onClick={() =>
            boosted
              ? unboostPost({ token, postId: status.id })
              : boostPost({ token, postId: status.id })
          }
        >
          <LoopIcon
            color={boosted ? 'primary' : 'inherit'}
            sx={{ color: boosted ? blue[500] : 'inherit' }}
          />
        </IconButton>
        <IconButton
          aria-label="add to favorites"
          onClick={() =>
            favourited
              ? unfavouritePost({ token, postId: status.id })
              : favouritePost({ token, postId: status.id })
          }
        >
          <FavoriteIcon
            color={favourited ? 'primary' : 'inherit'}
            sx={{ color: favourited ? pink[500] : 'inherit' }}
          />
        </IconButton>
        <IconButton aria-label="add to bookmark">
          <BookmarkIcon />
        </IconButton>
        {/* リプライを表示するボックス */}
        <ExpandMore
          expand={expanded}
          onClick={handleExpandClick}
          aria-expanded={expanded}
          aria-label="show reply"
        >
          <ExpandMoreIcon />
        </ExpandMore>
      </CardActions>
      <Collapse in={expanded} timeout="auto" unmountOnExit>
        {/* TODO: ここにリプライを入れる */}
      </Collapse>
      {/* 返信用のテキストボックスを表示させる */}
      {isOpen && <Postbox postId={status.id} displayName={displayName} />}
    </>
  );
};
