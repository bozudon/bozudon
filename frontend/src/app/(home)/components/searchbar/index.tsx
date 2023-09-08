'use client';
import {
  Card,
  Divider,
  IconButton,
  InputBase,
  Paper,
  CardHeader,
} from '@mui/material';
import SearchIcon from '@mui/icons-material/Search';
import { AlertCollapse, ErrorAlert } from '@/app/components/alert';
import MuiAvatar from '@mui/material/Avatar';
import Link from 'next/link';
import { useToken } from '@/app/components/auth/auth-context';
import { getSearchResult, SearchResultType } from './action';
import { useState } from 'react';
import styles from './search.module.css';

export const Searchbar = () => {
  const { token } = useToken();
  const [error, setError] = useState('');
  const [searchResult, setSearchResult] = useState<SearchResultType>({
    accounts: [],
    statuses: [],
    hashtags: [],
  });
  const onSearch = async (formData: FormData) => {
    const result = await getSearchResult(formData, token);
    if (result) {
      setSearchResult(result);
    } else {
      setError('Failed to search.');
    }
  };

  return (
    <div>
      <Paper
        component="form"
        sx={{
          p: '2px 4px',
          display: 'flex',
          alignItems: 'center',
          borderRadius: 5,
          ':hover': { boxShadow: '0 0 7px #719ECE' },
          ':focus-within': { boxShadow: '0 0 7px #719ECE' },
        }}
        action={onSearch}
      >
        <InputBase
          sx={{ ml: 1, flex: 1 }}
          placeholder="Search ..."
          inputProps={{ 'aria-label': 'search' }}
          name="query"
        />
        <Divider sx={{ height: 28, m: 0.5 }} orientation="vertical" />
        <IconButton type="submit" sx={{ p: '10px' }} aria-label="search">
          <SearchIcon />
        </IconButton>
      </Paper>
      <div className={styles.resultsContainer}>
        {/* 最大で10件まで表示する */}
        {searchResult.accounts.slice(0, 10).map((account, index) => (
          <Link href={`/details/${account.id}`} key={index}>
            <Card className={styles.card}>
              <CardHeader
                avatar={
                  <MuiAvatar
                    sx={{
                      width: 30,
                      height: 30,
                    }}
                  />
                }
                title={account.display_name}
                subheader={account.acct}
              />
            </Card>
          </Link>
        ))}
      </div>
      {/* エラー時にアラートを表示する */}
      <AlertCollapse show={error !== ''}>
        <ErrorAlert error={error} setError={setError} />
      </AlertCollapse>
    </div>
  );
};
