const fs = require('fs');
const path = require('path');
const { exec } = require('child_process');

const terminalLoggerPath = path.join(__dirname, '..', 'telegram', 'terminal_logger.js');
const outputFile = path.join(__dirname, '..', 'telegram', 'telegram_terminal_output');
const projectStructureFile = path.join(process.env.HOME, 'Desktop', 'Macadamia_Project_Structure.txt');

describe('Terminal Logger', () => {
  beforeEach(() => {
    fs.writeFileSync(outputFile, '');
    if (!fs.existsSync(projectStructureFile)) {
      fs.writeFileSync(projectStructureFile, 'Dummy project structure');
    }
  });

  test('logs message to file and sends via Telegram', () => {
    return new Promise((resolve) => {
      const testMessage = 'Test message';
      const child = exec(`echo "${testMessage}" | node ${terminalLoggerPath}`);

      let stdoutData = '';
      let stderrData = '';

      child.stdout.on('data', (data) => {
        stdoutData += data;
      });

      child.stderr.on('data', (data) => {
        stderrData += data;
      });

      child.on('close', (code) => {
        expect(code).toBe(0);

        const loggedContent = fs.readFileSync(outputFile, 'utf8');
        expect(loggedContent.trim()).toBe(testMessage);

        expect(stdoutData).toContain('Message sent to chat');
        expect(stdoutData).toContain('Project structure sent to chat');

        resolve();
      });
    });
  }, 10000); // Increase timeout to 10 seconds
});
