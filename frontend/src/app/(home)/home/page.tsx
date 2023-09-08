import { getToken } from '@/app/utils/auth';
import { Timeline } from './components/timeline';
import { fetchTimeline } from '@/app/(home)/home/api/fetchTimeline';
import type { Params } from '@/app/(home)/home/api/fetchTimeline';

const Page = async () => {
  const token = getToken();

  const params: Params = {};
  const statuses = await fetchTimeline(token, params);

  return <Timeline statuses={statuses} />;
};

export default Page;
