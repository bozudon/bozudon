'use client';

import { memo, useEffect, ReactNode } from 'react';
import { IconButton, Alert, AlertColor } from '@mui/material';
import { Box, Collapse } from '@mui/material';

import CloseIcon from '@mui/icons-material/Close';

type Props = {
  displayingTime?: number;
  severity: AlertColor;
  message: string;
  setMessage: (message: string) => void;
};

// 指定がない場合は以下の秒数(ミリ秒)でアラートを非表示にする
const DEFAULT_DISPLAYING_TIME = 3000;

export const MyAlert = memo(function MyAlert({
  displayingTime = DEFAULT_DISPLAYING_TIME,
  severity,
  message,
  setMessage,
}: Props) {
  useEffect(() => {
    // 一定時間経過したらアラートを非表示にする
    const timeId = setTimeout(() => {
      setMessage('');
    }, displayingTime);

    return () => {
      clearTimeout(timeId);
    };
    // NOTE: 依存配列にmessageを追加しないとアラートが呼ばれるたびにtimeoutが実行されない
  }, [message, displayingTime, setMessage]);

  return (
    <Alert
      severity={severity}
      action={
        <IconButton
          aria-label="close"
          color="inherit"
          size="small"
          onClick={() => {
            setMessage('');
          }}
        >
          <CloseIcon fontSize="inherit" />
        </IconButton>
      }
      sx={{
        mb: 2,
        position: 'fixed',
        top: 0,
        left: 0,
        margin: '2em',
        zIndex: 1500,
      }}
    >
      {message}
    </Alert>
  );
});

type ErrorAlertProps = {
  error: string;
  setError: (error: string) => void;
};

export const ErrorAlert = memo(function ChatErrorAlert({
  error,
  setError,
}: ErrorAlertProps) {
  return <MyAlert severity="error" message={error} setMessage={setError} />;
});

type AlertCollapseProps = {
  show: boolean;
  children: ReactNode;
};

export const AlertCollapse = memo(function AlertCollapse({
  show,
  children,
}: AlertCollapseProps) {
  return (
    <Box sx={{ width: '100%' }}>
      <Collapse in={show}>{children}</Collapse>
    </Box>
  );
});
