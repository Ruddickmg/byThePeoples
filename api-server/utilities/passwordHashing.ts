import { totalmem } from 'os';
import { hash, verify, argon2i } from 'argon2';
import { getPercentage } from './math';

const percentOfMemoryToUse = 0.0001;
const memory = totalmem();

const configuration = {
  timeCost: 20,
  type: argon2i,
  parallelism: 1,
  memoryCost: Math.ceil(getPercentage(percentOfMemoryToUse, memory)),
};

export const hashPassword = (password: string): Promise<string> => hash(password, configuration);

export const verifyHashedPassword = (
  hashedPassword: string,
  password: string,
): Promise<boolean> => {
  console.log('hashed', hashedPassword, 'length', hashedPassword.length);
  return verify(hashedPassword, password);
}
