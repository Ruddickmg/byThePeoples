import { isProduction } from '../utilities/environment';

export const LOCAL_HOST = '127.0.0.1';
export const ALL_ADDRESSES = '0.0.0.0';
export const HOST = isProduction() ? ALL_ADDRESSES : LOCAL_HOST;
export const PORT = '8080';
