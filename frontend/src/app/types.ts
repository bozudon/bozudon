export type Statuses = Array<Status>;

// サーバーサイドで実装していないものはオプションにしている
export type Account = {
  id: string;
  username: string;
  acct: string;
  display_name: string;
  created_at: string;
  locked?: boolean;
  bot?: boolean;
  note?: string;
  url?: string;
  avatar?: string;
  avatar_static?: string;
  header?: string;
  header_static?: string;
  followers_count?: number;
  following_count?: number;
  statuses_count?: number;
  last_status_at?: string;
  emojis?: Array<string>; // 絵文字に関する具体的な構造がわからないため、とりあえず文字列の配列としています
  fields?: Array<Field>;
};

export type Field = {
  name: string;
  value: string;
  verified_at: string | null;
};

type Application = {
  name: string;
  website: string | null;
};

export type Status = {
  id: string;
  created_at: string;
  in_reply_to_id: string | null;
  in_reply_to_account_id: string | null;
  sensitive: boolean;
  spoiler_text: string;
  visibility: string;
  language: string | null;
  uri: string;
  url: string;
  replies_count: number;
  reblogs_count: number;
  favourites_count: number;
  edited_at: string | null;
  favourited: boolean;
  reblogged: boolean;
  muted: boolean;
  bookmarked: boolean;
  content: string;
  filtered: string[];
  reblog: Status | null;
  application: Application | null;
  account: Account;
  media_attachments: any[];
  mentions: any[];
  tags: any[];
  emojis: string[];
  card: any | null;
  poll: any | null;
};
