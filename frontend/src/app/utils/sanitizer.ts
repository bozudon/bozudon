import DOMPurify from 'dompurify';

// NOTE: サニタイズのルールは以下を参照
// https://docs.joinmastodon.org/spec/activitypub/#sanitization
const config = {
  ALLOWED_TAGS: [
    'p',
    'span',
    'br',
    'a',
    'del',
    'pre',
    'code',
    'em',
    'strong',
    'b',
    'i',
    'u',
    'ul',
    'ol',
    'li',
    'blockquote',
  ],
  ALLOWED_ATTR: ['class', 'href', 'rel', 'start', 'reversed', 'value'],
  FORBID_TAGS: ['style'],
  FORBID_ATTR: ['style'],
  ADD_ATTR: ['target', 'rel'],
};

export const sanitize = (dirty: string) => {
  const safe = DOMPurify.sanitize(dirty, config);
  return safe;
};
