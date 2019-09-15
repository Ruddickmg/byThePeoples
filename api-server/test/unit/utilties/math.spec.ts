import { expect } from 'chai';
import { getPercentage } from '../../../utilities/math';

describe('math', (): void => {
  describe('getPercentage', (): void => {
    it('Will get the correct percentage of a number', (): void => {
      expect(getPercentage(25, 100)).to.equal(25);
    });
  });
});
