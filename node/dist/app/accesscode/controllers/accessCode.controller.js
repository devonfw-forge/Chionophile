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
exports.AccessCodeController = void 0;
const openapi = require("@nestjs/swagger");
const common_1 = require("@nestjs/common");
const criteria_1 = require("../dto/criteria");
const accessCode_service_1 = require("../services/accessCode.service");
let AccessCodeController = class AccessCodeController {
    constructor(accessCode) {
        this.accessCode = accessCode;
    }
    searchCode(crit) {
        return this.accessCode.searchCriteria(crit);
    }
};
__decorate([
    (0, common_1.Post)(),
    (0, common_1.HttpCode)(200),
    openapi.ApiResponse({ status: 200, type: require("../dto/accessCodeSearchDto").AccessCodeSearchDTO }),
    __param(0, (0, common_1.Body)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [criteria_1.Criteria]),
    __metadata("design:returntype", Promise)
], AccessCodeController.prototype, "searchCode", null);
AccessCodeController = __decorate([
    (0, common_1.Controller)('accesscodemanagement/v1/accesscode/cto/search'),
    __metadata("design:paramtypes", [accessCode_service_1.AccessCodeService])
], AccessCodeController);
exports.AccessCodeController = AccessCodeController;
//# sourceMappingURL=accessCode.controller.js.map