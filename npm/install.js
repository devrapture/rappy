const { downloadRappy } = require('./download');

downloadRappy().catch((err) => {
  console.error('Failed to download rappy binary:', err);
  process.exit(1);
}); 