import { getToken } from '@/app/utils/auth';
import { Timeline } from '@/app/(home)/home/components/timeline';
import { fetchFavouriteStatuses } from '@/app/(home)/home/api/fetchFavouritesStatuses';

const Favourites = async () => {
  const token = getToken();
  const statuses = await fetchFavouriteStatuses(token);

  return <Timeline statuses={statuses} />;
};
export default Favourites;
