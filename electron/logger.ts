import log from 'electron-log';
import path from 'path';
import { app } from 'electron';

log.transports.file.level = 'info';
log.transports.file.maxSize = 10 * 1024 * 1024;
log.transports.file.format = '[{y}-{m}-{d} {h}:{i}:{s}] [{level}] {text}';
log.transports.file.resolvePathFn = () =>
  path.join(app.getPath('userData'), 'logs', 'main.log');

log.transports.console.level = 'debug';
log.transports.console.format = '[{y}-{m}-{d} {h}:{i}:{s}] [{level}] {text}';

export default log;
