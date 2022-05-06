"use strict";
var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
var __metadata = (this && this.__metadata) || function (k, v) {
    if (typeof Reflect === "object" && typeof Reflect.metadata === "function") return Reflect.metadata(k, v);
};
var __param = (this && this.__param) || function (paramIndex, decorator) {
    return function (target, key) { decorator(target, key, paramIndex); }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.WinstonLogger = void 0;
const config_1 = require("@devon4node/config");
const common_1 = require("@nestjs/common");
const winston = require("winston");
let WinstonLogger = class WinstonLogger extends common_1.Logger {
    constructor(configService) {
        var _a, _b, _c, _d, _e, _f, _g, _h, _j, _k;
        super();
        this.configService = configService;
        this.console = true;
        const logLevel = ((_b = (_a = this.configService) === null || _a === void 0 ? void 0 : _a.values.loggerConfig) === null || _b === void 0 ? void 0 : _b.loggerLevel) || WinstonLogger.DEFAULT_LOG_LEVEL;
        const generalDir = (_d = (_c = this.configService) === null || _c === void 0 ? void 0 : _c.values.loggerConfig) === null || _d === void 0 ? void 0 : _d.generalLogFile;
        const errorDir = (_f = (_e = this.configService) === null || _e === void 0 ? void 0 : _e.values.loggerConfig) === null || _f === void 0 ? void 0 : _f.errorLogFile;
        const transports = [];
        if (((_h = (_g = this.configService) === null || _g === void 0 ? void 0 : _g.values.loggerConfig) === null || _h === void 0 ? void 0 : _h.console) !== undefined) {
            this.console = (_k = (_j = this.configService) === null || _j === void 0 ? void 0 : _j.values.loggerConfig) === null || _k === void 0 ? void 0 : _k.console;
        }
        if (generalDir) {
            transports.push(new winston.transports.File({
                filename: generalDir,
            }));
        }
        if (errorDir) {
            transports.push(new winston.transports.File({
                filename: errorDir,
                level: 'error',
            }));
        }
        this.overrideLogger(logLevel);
        if (transports.length) {
            this.logger = winston.createLogger({
                format: winston.format.combine(winston.format.timestamp(), winston.format.json()),
                level: logLevel,
                transports,
            });
        }
    }
    overrideLogger(level) {
        const loggerLevels = ['debug', 'verbose', 'log', 'warn', 'error'];
        let nestLoggerLevel = level;
        if (nestLoggerLevel === 'info' || nestLoggerLevel === 'http') {
            nestLoggerLevel = 'log';
        }
        if (nestLoggerLevel === 'silly') {
            nestLoggerLevel = 'debug';
        }
        const pos = loggerLevels.findIndex(e => e === nestLoggerLevel);
        if (pos !== -1) {
            common_1.Logger.overrideLogger(loggerLevels.slice(pos));
        }
    }
    log(message, context) {
        if (this.console) {
            super.log(message, context);
        }
        if (this.logger) {
            this.logger.info({ message, context });
        }
    }
    error(message, trace, context) {
        if (this.console) {
            super.error(message, trace, context);
        }
        if (this.logger) {
            this.logger.error({ message, trace, context });
        }
    }
    warn(message, context) {
        if (this.console) {
            super.warn(message, context);
        }
        if (this.logger) {
            this.logger.warn({ message, context });
        }
    }
    debug(message, context) {
        if (this.console) {
            super.debug(message, context);
        }
        if (this.logger) {
            this.logger.debug({ message, context });
        }
    }
    verbose(message, context) {
        if (this.console) {
            super.verbose(message, context);
        }
        if (this.logger) {
            this.logger.verbose({ message, context });
        }
    }
};
WinstonLogger.DEFAULT_LOG_LEVEL = 'info';
WinstonLogger = __decorate([
    __param(0, (0, common_1.Optional)()),
    __metadata("design:paramtypes", [config_1.ConfigService])
], WinstonLogger);
exports.WinstonLogger = WinstonLogger;
//# sourceMappingURL=winston.logger.js.map