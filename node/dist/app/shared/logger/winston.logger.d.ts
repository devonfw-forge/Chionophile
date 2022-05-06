import { ConfigService } from '@devon4node/config';
import { Logger } from '@nestjs/common';
import { Config } from '../model/config/config.model';
export declare class WinstonLogger extends Logger {
    private readonly configService?;
    private static DEFAULT_LOG_LEVEL;
    private console;
    private logger?;
    constructor(configService?: ConfigService<Config> | undefined);
    overrideLogger(level: 'error' | 'warn' | 'info' | 'http' | 'verbose' | 'debug' | 'silly'): void;
    log(message: string, context?: string): void;
    error(message: string, trace: string, context?: string): void;
    warn(message: string, context?: string): void;
    debug(message: any, context?: string): void;
    verbose(message: any, context?: string): void;
}
