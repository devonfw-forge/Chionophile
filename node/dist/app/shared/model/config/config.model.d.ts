import { BaseConfig } from '@devon4node/config';
import { ConnectionOptions } from 'typeorm';
import { JwtModuleOptions } from '@nestjs/jwt';
export declare class LoggerConfiguration {
    loggerLevel?: 'error' | 'warn' | 'info' | 'http' | 'verbose' | 'debug' | 'silly';
    generalLogFile?: string;
    errorLogFile?: string;
    console?: boolean;
}
export declare class SwaggerConfig {
    swaggerTitle: string;
    swaggerDescription: string;
    swaggerVersion: string;
}
export declare class Config extends BaseConfig {
    loggerConfig?: LoggerConfiguration;
    database: ConnectionOptions;
    swaggerConfig?: SwaggerConfig;
    jwtConfig: JwtModuleOptions;
}
