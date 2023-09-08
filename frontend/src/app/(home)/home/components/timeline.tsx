'use client';

import MuiTimeline from '@mui/lab/Timeline';
import MuiTimelineItem from '@mui/lab/TimelineItem';
import TimelineContent from '@mui/lab/TimelineContent';
import Card from './card';
import type { Statuses } from '@/app/types';
import { styled } from '@mui/material/styles';
import { useEffect, useState } from 'react';
import CardsLoading from '../../components/skeletonUi/cardsLoading';

type Props = {
  statuses: Statuses;
  // ステータスの詳細画面でクリックしたステータスを分かりやすくするために使用する
  currentStatusId?: string;
};

/**
 * @mui/lab/Timeline コンポーネントでは、TimelineItem のbeforeに時間軸を示すドットや線などが表示されるデザインとなっている。
 * 不要に空白ができてしまうため、表示させないようにstyleを上書きしている。
 */
const TimelineItem = styled(MuiTimelineItem)({
  '&::before': {
    display: 'none',
  },
});

export const Timeline = ({ statuses, currentStatusId }: Props) => {
  const [isMounted, setIsMounted] = useState<boolean>(false);

  // 現在開いているリプライボックスがどのカードに関連するものかを追跡するstate
  const [openReplyId, setOpenReplyId] = useState<string | null>(null);

  useEffect(() => {
    setIsMounted(true);
  }, []);

  // 別のカードのリプライボタンを押したときに、リプライボックスを閉じるためにハンドラーを定義する
  const handleReply = (id: string) => {
    setOpenReplyId(id);
  };

  if (!isMounted) {
    return <CardsLoading />;
  }
  return (
    <MuiTimeline sx={{ padding: '0px' }}>
      {statuses.map((item, index) => (
        <TimelineItem
          key={index}
          // クリックしたステータスのIDと一致する場合にハイライトする
          sx={{
            borderColor:
              item.id === currentStatusId ? '#808080' : 'transparent', // Highlight color
            borderWidth: '2px',
            borderStyle: 'solid',
            borderRadius: '8px',
            marginBottom: '8px',
            marginTop: '8px',
            padding: '8px',
          }}
        >
          <TimelineContent
            sx={{
              paddingLeft: '0px',
              paddingRight: '0px',
            }}
          >
            <Card
              item={item}
              onReply={handleReply}
              isOpen={item.id === openReplyId}
            />
            {/* リプライボックスが開くかどうかは、そのカードのIDがopenReplyIdと一致するかどうかで決定される */}
          </TimelineContent>
        </TimelineItem>
      ))}
    </MuiTimeline>
  );
};
