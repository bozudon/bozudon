import { fetchStatus, fetchParentAndChildStatuses } from '@/app/api/status';
import { getToken } from '@/app/utils/auth';
import { Timeline } from '@/app/(home)/home/components/timeline';

// ステータスの詳細ページ
const DetailsStatus = async ({ params }: { params: { slug: string } }) => {
  const token = getToken();
  // 返信含めて前後の投稿を取得する
  const context = await fetchParentAndChildStatuses(token, params.slug);
  const status = await fetchStatus(token, params.slug);
  if (!status) {
    return <div>Sorry, Status is not found.</div>;
  }
  const statuses = [...context.ancestors, status, ...context.descendants];

  // focusを当てるidを与えるようにする
  return <Timeline statuses={statuses} currentStatusId={status.id} />;
};

export default DetailsStatus;
