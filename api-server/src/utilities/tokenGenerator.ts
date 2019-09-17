import { randomBytes } from 'crypto';

const tokenSize = 64;
const tokenType = 'hex';

export const tokenGenerator = (): Promise<string> => new Promise<string>((
  resolve,
  reject,
): void => randomBytes(tokenSize, (err, buffer): void => {
  if (err) {
    reject(err);
  }
  resolve(buffer.toString(tokenType));
}));
