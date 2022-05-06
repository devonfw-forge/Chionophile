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
Object.defineProperty(exports, "__esModule", { value: true });
exports.Config = exports.SwaggerConfig = exports.LoggerConfiguration = void 0;
const config_1 = require("@devon4node/config");
const class_transformer_1 = require("class-transformer");
const class_validator_1 = require("class-validator");
class LoggerConfiguration {
}
__decorate([
    (0, class_validator_1.IsOptional)(),
    (0, class_validator_1.IsString)(),
    (0, class_validator_1.IsIn)(['error', 'warn', 'info', 'http', 'verbose', 'debug', 'silly']),
    __metadata("design:type", String)
], LoggerConfiguration.prototype, "loggerLevel", void 0);
__decorate([
    (0, class_validator_1.IsOptional)(),
    (0, class_validator_1.IsString)(),
    __metadata("design:type", String)
], LoggerConfiguration.prototype, "generalLogFile", void 0);
__decorate([
    (0, class_validator_1.IsOptional)(),
    (0, class_validator_1.IsString)(),
    __metadata("design:type", String)
], LoggerConfiguration.prototype, "errorLogFile", void 0);
__decorate([
    (0, class_validator_1.IsOptional)(),
    (0, class_validator_1.IsBoolean)(),
    __metadata("design:type", Boolean)
], LoggerConfiguration.prototype, "console", void 0);
exports.LoggerConfiguration = LoggerConfiguration;
class SwaggerConfig {
}
__decorate([
    (0, class_validator_1.IsDefined)(),
    (0, class_validator_1.IsString)(),
    __metadata("design:type", String)
], SwaggerConfig.prototype, "swaggerTitle", void 0);
__decorate([
    (0, class_validator_1.IsDefined)(),
    (0, class_validator_1.IsString)(),
    __metadata("design:type", String)
], SwaggerConfig.prototype, "swaggerDescription", void 0);
__decorate([
    (0, class_validator_1.IsDefined)(),
    (0, class_validator_1.IsString)(),
    __metadata("design:type", String)
], SwaggerConfig.prototype, "swaggerVersion", void 0);
exports.SwaggerConfig = SwaggerConfig;
class Config extends config_1.BaseConfig {
}
__decorate([
    (0, class_validator_1.IsOptional)(),
    (0, class_validator_1.ValidateNested)(),
    (0, class_transformer_1.Type)(() => LoggerConfiguration),
    __metadata("design:type", LoggerConfiguration)
], Config.prototype, "loggerConfig", void 0);
__decorate([
    (0, class_validator_1.IsDefined)(),
    (0, class_validator_1.IsNotEmptyObject)(),
    __metadata("design:type", Object)
], Config.prototype, "database", void 0);
__decorate([
    (0, class_validator_1.IsDefined)(),
    (0, class_validator_1.ValidateNested)(),
    (0, class_transformer_1.Type)(() => SwaggerConfig),
    __metadata("design:type", SwaggerConfig)
], Config.prototype, "swaggerConfig", void 0);
__decorate([
    (0, class_validator_1.IsDefined)(),
    (0, class_validator_1.IsNotEmptyObject)(),
    __metadata("design:type", Object)
], Config.prototype, "jwtConfig", void 0);
exports.Config = Config;
//# sourceMappingURL=config.model.js.map