import { Router } from 'express';

export const userRoutes = Router();

// define the home page route
userRoutes.get('/', function (req, res) {
  res.send('Birds home page')
});

// define the about route
userRoutes.get('/about', function (req, res) {
  res.send('About birds')
});

export default userRoutes;