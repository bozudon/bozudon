import { AccountProfile } from '../components/account-profile';
import { getToken } from '@/app/utils/auth';
import { fetchAccountStatuses } from '../fetchAccountStatuses';
import { fetchAccount } from '../fetchAccount';
import { isCurrentAccount } from '../isCurrentAccount';
import { isFollowing } from '../isFollowing';

// https://nextjs.org/docs/app/building-your-application/routing/dynamic-routes
const Details = async ({ params }: { params: { slug: string } }) => {
  const token = getToken();
  const account = await fetchAccount(token, params.slug);

  const fetchAccountParams = {};
  const statuses = await fetchAccountStatuses(
    params.slug,
    token,
    fetchAccountParams
  );
  if (!account) {
    return <div>アカウントが見つかりませんでした。</div>;
  }

  const isCurrentAcct = await isCurrentAccount(token, account.id);
  const isFollowingAcct = await isFollowing(token, account.id);
  return (
    <AccountProfile
      account={account}
      statuses={statuses}
      isCurrentAccount={isCurrentAcct}
      isFollowing={isFollowingAcct}
      token={token}
    />
  );
};

export default Details;
