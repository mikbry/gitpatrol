import { createRoot } from 'react-dom/client';
import React, { StrictMode } from 'react';
import App from './App';
import '@/styles/global.css';

const container = document.getElementById('root');

if (!container) {
  throw new Error('Failed to find the root element');
}

const root = createRoot(container);

root.render(React.createElement(StrictMode, null, React.createElement(App)));
