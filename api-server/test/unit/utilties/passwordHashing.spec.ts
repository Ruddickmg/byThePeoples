import { expect } from 'chai';
import { hashPassword, verifyHashedPassword } from '../../../utilities/passwordHashing';

describe('passwordHashing', (): void => {
  describe('hashPassword', (): void => {
    it(' Hashes a password and verifies it against the original', async (): Promise<void> => {
      const password = 'OverGrown123';
      const hash = await hashPassword(password);
      expect(await verifyHashedPassword(hash, password)).to.equal(true);
    });
  });
});
