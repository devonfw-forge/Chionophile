import { Config } from '../app/shared/model/config/config.model';
const confOrm = require('../../ormconfig.json');

confOrm.host = process.env.DB_HOST || confOrm.host;
confOrm.port = process.env.DB_PORT !== undefined ? parseInt(process.env.DB_PORT) : confOrm.port;
confOrm.database = process.env.DB_NAME || confOrm.database;
confOrm.username = process.env.DB_USER || confOrm.username;
confOrm.password = process.env.DB_PASSWORD || confOrm.password;

const def: Config = {
  isDev: true,
  host: process.env.HOST || 'localhost',
  port: process.env.PORT !== undefined ? parseInt(process.env.PORT) : 8081,
  clientUrl: 'localhost:4200',
  globalPrefix: process.env.BASE_REST_URL || 'jumpthequeue/services/rest',
  loggerConfig: {
    console: true,
    errorLogFile: './logs/error.log',
    generalLogFile: './logs/general.log',
    loggerLevel: 'info',
  },
  database: confOrm,
  swaggerConfig: {
    swaggerTitle: 'NestJS Application',
    swaggerDescription: 'API Documentation',
    swaggerVersion: '0.0.1',
  },
  jwtConfig: { secret: 'SECRET', signOptions: { expiresIn: '24h' } },
};

export default def;
