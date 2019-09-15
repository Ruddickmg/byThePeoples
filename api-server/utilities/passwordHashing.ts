import { totalmem } from 'os';
import { hash, verify, argon2i } from 'argon2';
import { getPercentage } from './math';

const percentOfMemoryToUse = 0.0001;
const memory = totalmem();
const getPercentageOfMemory = (): number => Math.ceil(getPercentage(percentOfMemoryToUse, memory));

const configuration = {
  timeCost: 20,
  hashLength: 32,
  type: argon2i,
  parallelism: 1,
  memoryCost: 8264,
};

export const hashPassword = (password: string): Promise<string> => hash(password, configuration);

export const verifyHashedPassword = (
  hashedPassword: string,
  password: string,
): Promise<boolean> => verify(hashedPassword, password);
