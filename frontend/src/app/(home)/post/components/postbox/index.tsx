'use client';

import { useRef, useState } from 'react';
import styles from './postbox.module.css';
import { postStatus } from '../../handler';

import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';
import InputAdornment from '@mui/material/InputAdornment';
import LoadingButton from '@mui/lab/LoadingButton';

import IconButton from '@mui/material/IconButton';
import InsertPhotoIcon from '@mui/icons-material/InsertPhoto';
import InsertEmoticonIcon from '@mui/icons-material/InsertEmoticon';
import GifIcon from '@mui/icons-material/Gif';

import { useToken } from '@/app/components/auth/auth-context';
import { AlertCollapse, ErrorAlert } from '@/app/components/alert';

import { useRouter } from 'next/navigation';
import { CardAvatar } from '@/app/(home)/home/components/card/avatar';

import { postImageStatus } from '@/app/api/media';
import Image from 'next/image';
import { sanitize } from '@/app/utils/sanitizer';

// postIdが与えられた場合はリプライの投稿ボックスを表示する
type Props = {
  postId?: string;
  displayName: string;
};

export default function Postbox({ postId, displayName }: Props) {
  const [loading, setLoading] = useState<boolean>(false);
  const statusRef = useRef<HTMLInputElement>();
  const [error, setError] = useState<string>('');
  const [file, setFile] = useState<File | null>(null);
  const { token } = useToken();
  const router = useRouter();

  const fileInputRef = useRef<HTMLInputElement>(null);
  const handlePhotoIconClick = () => {
    // ファイル選択ダイアログを開く
    fileInputRef.current?.click();
  };

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files || files.length === 0) {
      setFile(null);
      return;
    }

    const file = files[0];
    setFile(file);
  };

  //　投稿ボタンを押したときの処理
  const handleClick = async (e: React.MouseEvent<HTMLElement>) => {
    e.preventDefault();
    setLoading(true);

    const value = statusRef.current?.value.trim();
    const message = value ? sanitize(value) : '';
    if (file) {
      const response = await postImageStatus(token, message, file);
      if (response) {
        statusRef.current!.value = '';
        fileInputRef.current!.value = '';
      }
    } else {
      if (message !== '' && message !== null) {
        const body = {
          status: sanitize(message),
          in_reply_to_id: postId, // postIdが存在する場合、リプライ先の投稿IDを設定
          // 以下は必須項目
          media_ids: [],
        };
        const response = await postStatus(token, body);
        if (response) {
          // メッセージとファイル選択をリセットする
          statusRef.current!.value = '';
          fileInputRef.current!.value = '';
        } else {
          setError('failed to post.. try some time later');
        }
      }
    }

    setLoading(false);
    setFile(null);
  };

  return (
    <>
      <AlertCollapse show={error !== ''}>
        <ErrorAlert error={error} setError={setError} />
      </AlertCollapse>
      <Box
        component="form"
        sx={{
          flexDirection: 'column',
          p: 1,
          mt: 1,
          border: '1px solid',
          borderRadius: 2,
          backgroundColor: 'white',
          borderColor: (theme) =>
            theme.palette.mode === 'dark' ? 'grey.800' : 'grey.300',
        }}
        autoComplete="off"
      >
        <TextField
          fullWidth
          multiline
          sx={{
            '& fieldset': { border: 'none' },
            paddingRight: 12,
            paddingTop: 4,
          }}
          InputProps={{
            startAdornment: (
              <InputAdornment position="start">
                <CardAvatar name={displayName} />
              </InputAdornment>
            ),
          }}
          rows={3}
          placeholder="how is your life?"
          inputRef={statusRef}
        />
        {file && (
          <Image
            src={URL.createObjectURL(file)}
            alt="Thumb"
            width={500}
            height={500}
          />
        )}
        <div className={styles.functionsContainer}>
          <div className={styles.dummy} aria-hidden="true" />
          <input
            ref={fileInputRef}
            type="file"
            accept="image/*"
            style={{ display: 'none' }} // ファイル選択inputを隠す
            onChange={handleFileChange}
          />
          <div className={styles.utilsContainer}>
            <IconButton aria-label="insert emotions">
              <InsertEmoticonIcon />
            </IconButton>
            <IconButton
              aria-label="insert photo"
              onClick={handlePhotoIconClick}
            >
              <InsertPhotoIcon />
            </IconButton>
            <IconButton aria-label="insert gif">
              <GifIcon />
            </IconButton>
          </div>
          <LoadingButton
            sx={{ mb: 2, mr: 3 }}
            loading={loading}
            onClick={handleClick}
            variant="outlined"
            type="submit"
          >
            gogo!
          </LoadingButton>
        </div>
      </Box>
    </>
  );
}
