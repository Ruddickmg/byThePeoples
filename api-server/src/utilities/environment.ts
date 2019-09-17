import { PRODUCTION } from '../constants/environment';

export const isProduction = (): boolean => process.env.NODE_ENV === PRODUCTION;
