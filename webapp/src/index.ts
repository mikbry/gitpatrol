import { createRoot } from 'react-dom/client';
import { StrictMode } from 'react';
import Home from './page';

const container = document.getElementById('root');

if (!container) {
  throw new Error('Failed to find the root element');
}

const root = createRoot(container);

root.render(
  <StrictMode>
    <Home />
  </StrictMode>
);
