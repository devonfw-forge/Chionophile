"use strict";
var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.CoreModule = void 0;
const common_1 = require("@nestjs/common");
const serializer_1 = require("@devon4node/common/serializer");
const core_1 = require("@nestjs/core");
const winston_logger_1 = require("../shared/logger/winston.logger");
const business_logic_filter_1 = require("../shared/filters/business-logic.filter");
const config_1 = require("@devon4node/config");
const config_model_1 = require("../shared/model/config/config.model");
const typeorm_1 = require("@nestjs/typeorm");
let CoreModule = class CoreModule {
};
CoreModule = __decorate([
    (0, common_1.Global)(),
    (0, common_1.Module)({
        imports: [
            typeorm_1.TypeOrmModule.forRootAsync({
                imports: [config_1.ConfigModule],
                useFactory: (config) => {
                    return config.values.database;
                },
                inject: [config_1.ConfigService],
            }),
            config_1.ConfigModule.forRoot({
                configPrefix: 'devon4node',
                configType: config_model_1.Config,
            }),
        ],
        controllers: [],
        providers: [
            { provide: core_1.APP_FILTER, useClass: business_logic_filter_1.BusinessLogicFilter },
            { provide: core_1.APP_INTERCEPTOR, useClass: serializer_1.ClassSerializerInterceptor },
            winston_logger_1.WinstonLogger,
        ],
        exports: [config_1.ConfigModule, winston_logger_1.WinstonLogger],
    })
], CoreModule);
exports.CoreModule = CoreModule;
//# sourceMappingURL=core.module.js.map