import Postbox from './components/postbox';
import { getDisplayName } from './handler';

export default async function Post() {
  const displayName = await getDisplayName();

  return <Postbox displayName={displayName} />;
}
