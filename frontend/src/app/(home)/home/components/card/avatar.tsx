import MuiAvatar from '@mui/material/Avatar';

// ref: https://mui.com/material-ui/react-avatar/#letter-avatars
const stringToColor = (string: string) => {
  let hash = 0;
  let i;

  /* eslint-disable no-bitwise */
  for (i = 0; i < string.length; i += 1) {
    hash = string.charCodeAt(i) + ((hash << 5) - hash);
  }

  let color = '#';

  for (i = 0; i < 3; i += 1) {
    const value = (hash >> (i * 8)) & 0xff;
    color += `00${value.toString(16)}`.slice(-2);
  }
  /* eslint-enable no-bitwise */

  return color;
};

// 名前に応じてアバターの色を変える
const stringAvatar = (name: string) => {
  return {
    sx: {
      bgcolor: stringToColor(name),
    },
    children: name[0],
  };
};

type Props = {
  name: string;
  width?: number;
  height?: number;
};

export const CardAvatar = ({ name, width = 40, height = 40 }: Props) => {
  const option = stringAvatar(name);
  return (
    <MuiAvatar
      {...option}
      sx={{
        ...option.sx,
        width: width,
        height: height,
        // 中央に表示される文字をCardの円の大きさに応じて変更する
        fontSize: `${width / 2}px`,
      }}
    />
  );
};
