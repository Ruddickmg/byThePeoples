import { hash } from 'argon2';

const configuration = {

};

export const hashPassword = (password: string): Promise<string> => hash(password, configuration);
export const verifyHashedPassword =
