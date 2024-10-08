const winston = require('winston');
const logger = require('../src/utils/logger');

describe('Logger', () => {
  let logMessages;

  beforeEach(() => {
    logMessages = [];
    const testTransport = new winston.transports.Console({
      format: winston.format.printf(info => {
        logMessages.push(info[Symbol.for('message')]);
        return info[Symbol.for('message')];
      })
    });
    logger.clear();
    logger.add(testTransport);
  });

  afterEach(() => {
    logger.clear();
  });

  test('debug logs message with correct format', () => {
    logger.debug('Test debug message');
    expect(logMessages[0]).toMatch(/\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z \[DEBUG\] Test debug message/);
  });

  test('info logs message with correct format', () => {
    logger.info('Test info message');
    expect(logMessages[0]).toMatch(/\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z \[INFO\] Test info message/);
  });

  test('warn logs message with correct format', () => {
    logger.warn('Test warn message');
    expect(logMessages[0]).toMatch(/\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z \[WARN\] Test warn message/);
  });

  test('error logs message with correct format', () => {
    logger.error('Test error message');
    expect(logMessages[0]).toMatch(/\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z \[ERROR\] Test error message/);
  });

  test('logs include timestamp', () => {
    logger.info('Test timestamp');
    expect(logMessages[0]).toMatch(/\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z/);
  });
});
