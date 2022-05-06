"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const def = {
    isDev: false,
    host: 'localhost',
    port: 3000,
    clientUrl: 'localhost:4200',
    globalPrefix: 'v1',
    loggerConfig: {
        console: false,
        errorLogFile: './logs/error.log',
        generalLogFile: './logs/general.log',
        loggerLevel: 'warn',
    },
    database: {
        type: 'postgres',
        host: 'localhost',
        port: 5432,
        username: 'test',
        password: 'test',
        database: 'test',
        synchronize: false,
        migrationsRun: true,
        logging: true,
        entities: ['dist/**/*.entity.js'],
        migrations: ['dist/migration/**/*.js'],
        subscribers: ['dist/subscriber/**/*.js'],
        cli: {
            entitiesDir: 'src/entity',
            migrationsDir: 'src/migration',
            subscribersDir: 'src/subscriber',
        },
    },
    jwtConfig: { secret: 'SECRET', signOptions: { expiresIn: '24h' } },
};
exports.default = def;
//# sourceMappingURL=production.js.map