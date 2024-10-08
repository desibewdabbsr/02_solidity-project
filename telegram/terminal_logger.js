const fs = require('fs');
const path = require('path');
const TelegramBot = require('node-telegram-bot-api');

const outputFile = 'telegram/telegram_terminal_output';
const projectStructureFile = path.join(process.env.HOME, 'Desktop', 'Macadamia_Project_Structure.txt');

const bots = [
  {
    token: '7240154674:AAF6XgmL8trGMpz1csAZSqSNsXEJwDTChIg',
    chatIds: ['6946103366']
  },
  {
    token: '7545537094:AAHpfrYus-g_qf6aaqq-RgYVe2KiBtWgfrc',
    chatIds: ['7036827196']
  }
];

const telegramBots = bots.map(bot => new TelegramBot(bot.token));

async function sendProjectStructure(bot, chatId) {
  try {
    await bot.sendDocument(chatId, projectStructureFile);
    console.log(`Project structure sent to chat ${chatId}`);
  } catch (error) {
    console.error(`Failed to send project structure to chat ${chatId}:`, error);
  }
}

async function logAndSend(message) {
  console.log(`Logging: ${message}`);
  fs.appendFileSync(outputFile, message + '\n');
  
  for (let i = 0; i < bots.length; i++) {
    for (const chatId of bots[i].chatIds) {
      try {
        await telegramBots[i].sendMessage(chatId, message);
        console.log(`Message sent to chat ${chatId}`);
        await sendProjectStructure(telegramBots[i], chatId);
      } catch (error) {
        console.error(`Failed to send message to chat ${chatId}:`, error);
      }
    }
  }
}

process.stdin.on('data', (data) => {
  logAndSend(data.toString().trim());
});

process.stdin.on('end', () => {
  console.log('Input stream ended. Logging complete.');
});

process.on('uncaughtException', (error) => {
  console.error('Uncaught Exception:', error);
  logAndSend(`Error: ${error.message}`);
});

console.log('Terminal logger started. Waiting for input...');
