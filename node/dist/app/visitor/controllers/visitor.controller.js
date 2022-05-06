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
exports.VisitorController = void 0;
const openapi = require("@nestjs/swagger");
const common_1 = require("@nestjs/common");
const visitor_service_1 = require("../services/visitor.service");
const swagger_1 = require("@nestjs/swagger");
const criteria_1 = require("../dto/criteria");
let VisitorController = class VisitorController {
    constructor(visitor) {
        this.visitor = visitor;
    }
    getVisitorByUsername(crit) {
        return this.visitor.searchCriteria(crit);
    }
};
__decorate([
    (0, common_1.Post)(),
    (0, common_1.HttpCode)(200),
    openapi.ApiResponse({ status: 200, type: require("../dto/visitorResponseDto").VisitorResponseDTO }),
    __param(0, (0, common_1.Body)()),
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [criteria_1.Criteria]),
    __metadata("design:returntype", Promise)
], VisitorController.prototype, "getVisitorByUsername", null);
VisitorController = __decorate([
    (0, common_1.Controller)('visitormanagement/v1/visitor/search'),
    (0, swagger_1.ApiTags)('VisitorSearch'),
    __metadata("design:paramtypes", [visitor_service_1.VisitorService])
], VisitorController);
exports.VisitorController = VisitorController;
//# sourceMappingURL=visitor.controller.js.map