import express, { Request, Response } from 'express';
import { HOST, PORT } from './constants/connection';

const app = express();

app.get('/', (req: Request, res: Response): Response => res.send('Hello World'));

// eslint-disable-next-line no-console
app.listen(PORT, (): void => console.log(`listening for connections @${HOST}:${PORT}`));
