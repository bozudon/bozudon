'use client';

import { Card, CardContent, CardHeader, Skeleton } from '@mui/material';

export default function CardLoading() {
  return (
    <Card sx={{ mt: 1 }}>
      <CardHeader
        avatar={
          <Skeleton
            animation="wave"
            variant="circular"
            width={40}
            height={40}
          />
        }
        action={null}
        title={
          <Skeleton
            animation="wave"
            height={10}
            width="80%"
            style={{ marginBottom: 6 }}
          />
        }
        subheader={<Skeleton animation="wave" height={10} width="40%" />}
      />
      <Skeleton
        sx={{ height: 80, marginLeft: 2, marginRight: 2 }}
        animation="wave"
        variant="rectangular"
      />
      <CardContent>
        <div>
          <Skeleton animation="wave" height={10} style={{ marginBottom: 6 }} />
          <Skeleton animation="wave" height={10} width="80%" />
        </div>
      </CardContent>
    </Card>
  );
}
