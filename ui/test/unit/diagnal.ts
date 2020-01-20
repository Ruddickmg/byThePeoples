import { expect } from 'chai';

describe('diagonal', () => {
    const diagonal = (matrix) => {
        const len = matrix.length;
        const doubleLen = len * 2;
        const ordered = [];
        let x;
        let y;
        for (let i = 0; i < doubleLen; i += 1) {
            x = 0;
            y = i;
            for (let j = 0; j <= i; j += 1) {
                if (x < len && y < len) {
                    ordered.push(matrix[x][y]);
                }
                x += 1;
                y -= 1;
            }
        }
        return ordered;
    };
   it('prints a matrix diagonally', () => expect(diagonal([
       [1,2,4],
       [3,5,7],
       [6,8,9],
   ])).to.eql([1,2,3,4,5,6,7,8,9]));
});