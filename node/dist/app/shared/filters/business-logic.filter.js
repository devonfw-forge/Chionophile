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
exports.BusinessLogicFilter = void 0;
const common_1 = require("@nestjs/common");
const business_logic_exception_1 = require("../exceptions/business-logic.exception");
const winston_logger_1 = require("../logger/winston.logger");
let BusinessLogicFilter = class BusinessLogicFilter {
    constructor(logger) {
        this.logger = logger;
    }
    catch(exception, host) {
        const ctx = host.switchToHttp();
        const response = ctx.getResponse();
        const request = ctx.getRequest();
        const responseObj = Object.assign(Object.assign({}, exception.plainObject()), { statusCode: 400, timestamp: new Date().toISOString(), path: request.url });
        if (this.logger) {
            this.logger.error(exception.message, exception.stack, 'LogicFilter');
        }
        response.status(400).json(responseObj);
    }
};
BusinessLogicFilter = __decorate([
    (0, common_1.Catch)(business_logic_exception_1.BusinessLogicException),
    __param(0, (0, common_1.Optional)()),
    __metadata("design:paramtypes", [winston_logger_1.WinstonLogger])
], BusinessLogicFilter);
exports.BusinessLogicFilter = BusinessLogicFilter;
//# sourceMappingURL=business-logic.filter.js.map