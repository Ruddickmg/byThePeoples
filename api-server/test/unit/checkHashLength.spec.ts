import { hash } from 'argon2';

describe('checking the hash length', (): void => {
  it('rubs the lotion on it\'s skin', async (): Promise<void> => {
    await Promise.all(
      ['one password', 'kswoieurluequ149yyyyyyyyyyyyyyyyyyyyyyyyy92878134-98', 'oj']
        .map(async (password: string): Promise<void> => console.log((await hash(password)))),
    );
  });
});
